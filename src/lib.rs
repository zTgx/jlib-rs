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

pub mod base;
pub mod message;
pub mod misc;
pub mod api;
pub mod contracts;

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
pub use crate::contracts::solidity::{
    SolidityInitMessage,SolidityInitResponse,
    SolidityInvokeMessage, SolidityInvokeResponse,
};
use crate::base::misc::util::{downcast_to_string};
use std::rc::Rc;
use std::cell::Cell;
use ws::{connect, CloseCode};
use serde_json::Value;
use crate::misc::config::Config;
use message::common::command_trait::CommandConversion;

pub trait ContractAPI {
    fn deploy<F>(&self, op: F)
    where F: Fn(Result<SolidityInitResponse, &'static str>);

    fn invoke<F>(&self, op: F)
    where F: Fn(Result<SolidityInvokeResponse, &'static str>);
}

pub struct Solidity {
    pub config: Box<Rc<Config>>,
    pub init_message: SolidityInitMessage,
    pub invoke_message: SolidityInvokeMessage,
}
impl Solidity {
    pub fn with_config(config: Box<Rc<Config>>) -> Self {
        Solidity {
            config: config,
            init_message: SolidityInitMessage::default(),
            invoke_message: SolidityInvokeMessage::default(),
        }
    }

    pub fn set_init_message(&mut self, message: SolidityInitMessage) {
        self.init_message = message;
    }

    pub fn set_invoke_message(&mut self, message: SolidityInvokeMessage) {
        self.invoke_message = message;
    }
}

impl ContractAPI for Solidity {
    fn deploy<F>(&self, op: F)
    where F: Fn(Result<SolidityInitResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        connect(self.config.addr, |out| {
            let copy = info.clone();

            if let Ok(command) = self.init_message.to_string() {
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
                }
        }
    }

    fn invoke<F>(&self, op: F)
    where F: Fn(Result<SolidityInvokeResponse, &'static str>){

        let info = Rc::new(Cell::new("".to_string()));

        connect(self.config.addr, |out| {
            let copy = info.clone();

            if let Ok(command) = self.invoke_message.to_string() {
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
