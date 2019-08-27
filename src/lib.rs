#[macro_use]
extern crate lazy_static;

extern crate typename;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate num;
extern crate void;
extern crate ws;
extern crate basex_rs;
extern crate cast_rs;
#[macro_use]
extern crate juniper;

pub mod base;
pub mod message;
pub mod misc;
pub mod api;
pub mod contracts;
pub mod graphyql;

pub use crate::base::wallet::wallet::Wallet as Wallet;

//Default` cannot be derived for enums, only structs
#[derive(Debug)]
pub enum RelationType {
    TRUST     = 0,
    AUTHORIZE = 1,
    FREEZE    = 3,
}
impl RelationType {
    pub fn get(&self) -> u32 {
        match *self {
            RelationType::TRUST     => { 0 },
            RelationType::AUTHORIZE => { 1 },
            RelationType::FREEZE    => { 3 },
        }
    }
}

//Offer Type
#[derive(PartialEq)]
pub enum OfferType {
    Sell,
    Buy,
}
impl OfferType {
    pub fn get(&self) -> &'static str {
        match *self {
            OfferType::Sell => { "Sell" },
            OfferType::Buy  => { "Buy"  },
        }
    }
}

//Generate Wallet
/*
Wallet DataStruct:
#[derive(Debug)]
pub struct Wallet {
    pub key_type: WalletType,
    pub address : String,    //starts with 'j'
    pub secret  : String,    //secret seed
    pub keypair : Keypair,   //public key & private key
}

Keypair DataStruct:
#[derive(Debug, Clone)]
pub struct Keypair {
    pub private_key: String, //hex string
    pub public_key: String,  //hex string
}
*/

#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
}

pub fn generate_wallet(wtype: WalletType) -> Wallet {
    Wallet::new(wtype)
}

//Subscribe
mod subscribe {

use ws::{connect, Handler, Sender, Handshake, Message, CloseCode};

use std::rc::Rc;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::subscribe::*;
use crate::message::common::command_trait::CommandConversion;

pub struct Client {
    out: Sender,
    op: Rc<dyn Fn(Result<SubscribeResponse, serde_json::error::Error>)>,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        if let Ok(command) = SubscribeCommand::default().to_string() {
            self.out.send(command).unwrap();
        }

        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        // Close the connection when we get a response from the server
        let resp = msg.into_text().unwrap();
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let result: String = x["result"].to_string();
            if let Ok(x) = serde_json::from_str(&result) as Result<SubscribeResponse, serde_json::error::Error> {
                //to call the function stored in `op`, surround the field access with parentheses
                (self.op)(Ok(x))
            } else if let Ok(x) = serde_json::from_str(&resp) as Result<SubscribeResponse, serde_json::error::Error> {
                (self.op)(Ok(x))
            }
        }

        // self.out.close(CloseCode::Normal)
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

pub trait SubscribeI {
    fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
    where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>);
}

pub struct Subscribe {}
impl Subscribe {
    pub fn new() -> Self {
        Subscribe {
        }
    }
}

impl SubscribeI for Subscribe {
    fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
    where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>) {

        let op_rc = Rc::new(op);

        connect(config.addr, |out| {

            let op = op_rc.clone();

            Client {
                out: out,
                op: op,
            }

        }).unwrap();
    }
}
} //end mod subscribe

lazy_static! {
    pub static ref SUBSCRIBE: subscribe::Subscribe = {
        subscribe::Subscribe::new()
    };
}
pub use subscribe::SubscribeI as SubscribeI;

///////////////////////////////////////////////////////////////////////////////////////
//
// Solidity contract APIs: deploy && invoke
//
///////////////////////////////////////////////////////////////////////////////////////

use crate::contracts::solidity::{
    SolidityInitMessage,
    SolidityInvokeMessage,
};
use std::rc::Rc;
use std::cell::Cell;
use ws::{connect, CloseCode};
use serde_json::Value;
use crate::misc::config::Config;
use message::common::command_trait::CommandConversion;
use serde::{Deserialize, Serialize};
use crate::base::misc::util::{downcast_to_string, check};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Args {
    #[serde(rename="Arg")]
    pub arg: Arg,
}
impl Args {
    pub fn new(arg: Arg) -> Self {
        Args {
            arg: arg,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Arg {
    #[serde(rename="Parameter")]
    pub parameter: String, //Hex String, 0xb6e456bb ===>getUnit 调用方法的（以太坊方式）十六进制处理。

    #[serde(rename="ContractParamsType")]
    pub contract_params_type: u8, //0 -> Address type; 1 -> general type.
}
impl Arg {
    pub fn new(parameter: String, contract_params_type: u8) -> Self {
        Arg {
            parameter: check(parameter),
            contract_params_type: contract_params_type,
        }
    }
}

//
// SolidityInitResponse
//
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInitTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: u64,

    #[serde(rename="Payload")]
    pub payload: String,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="Method")]
    pub method: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Timestamp")]
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInitResponse {
    #[serde(rename="ContractState")]
    pub address: String,

    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: u64,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: SolidityInitTxJsonResponse,
}


/////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInvokeTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Args")]
    pub args: Vec<Args>,

    #[serde(rename="ContractMethod")]
    pub method: String,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Timestamp")]
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInvokeResponse {
    #[serde(rename="ContractState")]
    pub contract_state: String,

    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: u64,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: SolidityInvokeTxJsonResponse,
}

pub struct SolidityDeploy <'a> {
    pub config  : Box<Rc<Config>>,
    pub account : &'a String,
    pub secret  : &'a String,
}
impl <'a> SolidityDeploy <'a> {
    pub fn with_params(config: Box<Rc<Config>>, account: &'a String, secret: &'a String) -> Self {
        SolidityDeploy {
            config: config,
            account: account,
            secret: secret,
        }
    }

    pub fn deploy<F>(&self, payload: &'a String, op: F)
    where F: Fn(Result<SolidityInitResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account = Rc::new(Cell::new( String::from( self.account.as_str() )) );
        let secret = Rc::new(Cell::new( String::from( self.secret.as_str() )) );
        let payload = Rc::new(Cell::new( String::from( payload.as_str() )) );

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account.clone();
            let secret = secret.clone();
            let payload = payload.clone();

            let message = SolidityInitMessage::with_params(account.take(), secret.take(), payload.take());
            if let Ok(command) = message.to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<SolidityInitResponse, serde_json::error::Error> {
                    op(Ok(v))
                } else {
                    op(Err("deploy Error!"))
                }
        } else {
            op(Err("deploy Error!"))
        }
    }
}

pub struct SolidityCall <'a> {
    pub config: Box<Rc<Config>>,
    pub account : &'a String,
    pub secret  : &'a String,
    pub address : &'a String,
}
impl <'a> SolidityCall <'a> {
    pub fn with_params(config: Box<Rc<Config>>, account: &'a String, secret: &'a String, address: &'a String) -> Self {
        SolidityCall {
            config: config,
            account: account,
            secret: secret,
            address: address,
        }
    }

    pub fn call<F>(&self, method_name: &'a String, args: Vec<Arg>, op: F)
    where F: Fn(Result<SolidityInvokeResponse, &'static str>){
        let info = Rc::new(Cell::new("".to_string()));

        let account = Rc::new(Cell::new( String::from( self.account.as_str())) );
        let secret = Rc::new(Cell::new( String::from(self.secret.as_str())) );
        let address = Rc::new(Cell::new( String::from( self.address.as_str() )));
        let method_name = Rc::new(Cell::new( String::from( method_name.as_str() )) );
        let args = Rc::new(Cell::new(args));

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account.clone();
            let secret = secret.clone();
            let address = address.clone();
            let method_name = method_name.clone();
            let args = args.clone();

            let message = SolidityInvokeMessage::with_params(account.take(), secret.take(), address.take(), method_name.take(), args.take());
            if let Ok(command) = message.to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<SolidityInvokeResponse, serde_json::error::Error> {
                    op(Ok(v))
                } else {
                    op(Err("Err..."))
                }
        } else {
            op(Err("Err..."))
        }
    }
}
