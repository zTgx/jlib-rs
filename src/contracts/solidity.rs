
/*
Deploy paras:
send message:  { id: 1,
  command: 'submit',
  secret: 'snoPBjXtMeMyMHUVTgbuqAfg1SUTb',
  tx_json:
   { Flags: 0,
     Fee: 10000,
     TransactionType: 'AlethContract',
     Account: 'jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh',
     Amount: 100000000,
     Method: 0,
     Payload:
      '363038303630343035323334383031353630306635373630303038306664356235303630393238303631303031653630303033393
      6303030663366653630383036303430353233343830313536303066353736303030383066643562353036303034333631303630343
      4353737633031303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303
      0303030303030303030363030303335303436336236653435366262383131343630343935373562363030303830666435623630346
      6363036313536356236303430383035313931383235323531393038313930303336303230303139306633356236303231393035366
      6656131363536323761376137323330353832303834373030326437366537623336636234306661396138383432663534303765616
      439343662386339633665653638323539633333383263346236303361383330303239' } }


InVoke params;
send message:  { id: 1,
  command: 'submit',
  secret: 'snoPBjXtMeMyMHUVTgbuqAfg1SUTb',
  tx_json:
   { Flags: 0,
     Fee: 10000,
     TransactionType: 'AlethContract',
     Account: 'jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh',
     Method: 1,
     Destination: 'jsQZVJXd3dvf9y1Lmghu3EAiNKgbpTAaKv',
     Amount: 0,
     Args: [ [Object] ] } }
*/


#![allow(unused)]
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;
use std::cell::RefCell;

use crate::misc::config::*;
use crate::commands::command_trait::CommandConversion;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::{Value};
use crate::base::util::{downcast_to_string, check};

//////////////////////
/*
@合约部署请求
SolidityInitMessage 请求格式
id      : u64
command : String
secret  : String,
tx_json : TxJson Object
值分别为(固定值): 1, "submit"
*/


#[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct SolidityInitTxJson {
    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: u64,

    #[serde(rename="Method")]
    pub method: i32,

    #[serde(rename="Payload")]
    pub payload: String, //hex string
}
impl SolidityInitTxJson {
    pub fn new(account: String, payload: String) -> Self {
        SolidityInitTxJson {
            flags: 0,
            fee: 10000,
            transaction_type: "AlethContract".to_string(),
            account: account,
            amount: 100000000,
            method: 0,
            payload: hex::encode(payload),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInitMessage {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="secret")]
    secret: String,

    #[serde(rename="tx_json")]
    tx_json: SolidityInitTxJson,
}

impl SolidityInitMessage {
    pub fn new(secret: String, tx_json: SolidityInitTxJson) -> Self {
        SolidityInitMessage {
          id: 1,
          command: "submit".to_string(),
          secret: secret,
          tx_json: tx_json,
        }
    }
}

impl CommandConversion for SolidityInitMessage {
    type T = SolidityInitMessage;
    fn to_string(&self) -> Result<String> {
        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        println!("{}", j);

        Ok(j)
    }
    
    fn box_to_raw(&self) -> &dyn Any {
        self
    }
}

//invoke
#[derive(Serialize, Deserialize, Debug, Default)]
// #[serde(rename_all = "camelCase")]
pub struct SolidityInvokeTxJson {
    #[serde(rename="Flags")]
    pub flags: i32,                                                                                                                                                 

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: u64,

    #[serde(rename="Method")]
    pub method: i32,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="ContractMethod")]
    contract_method: String,
 
    #[serde(rename="Args")]
    pub args: Vec<Args>,
}

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

impl SolidityInvokeTxJson {
    pub fn new(account: String, destination: String, contract_method: String, args: Vec<Args>) -> Self {
        SolidityInvokeTxJson {
            flags: 0,
            fee: 10000,
            transaction_type: "AlethContract".to_string(),
            account: account,
            amount: 0,
            method: 1,
            destination: destination, //contract address
            contract_method: contract_method,
            args: args,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInvokeMessage {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="secret")]
    secret: String,

    #[serde(rename="tx_json")]
    tx_json: SolidityInvokeTxJson,
}

impl SolidityInvokeMessage {
    pub fn new(secret: String, tx_json: SolidityInvokeTxJson) -> Self {
        SolidityInvokeMessage {
          id: 1,
          command: "submit".to_string(),
          secret: secret,
          tx_json: tx_json,
        }
    }

    pub fn with_params(account: String, secret: String, address: String, contract_method: String, args: Vec<Arg>) -> Self {
        if account.len() != 34 || secret.len() != 29 || address.len() != 34 || contract_method.len() < 8 {
            panic!("Input params Error!");
        }

        //prepare
        let mut v: Vec<Args> = vec![];
        for x in args {
            let t = Args::new(x);
            v.push(t);
        }

        //convert [contract_method] to hex.
        let mut hex_method = contract_method;
        if hex_method.starts_with("0x") {
            hex_method = hex_method.get(2..10).unwrap().to_string();
        } else {
            hex_method = hex_method.get(0..8).unwrap().to_string();
        }

        SolidityInvokeMessage {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: SolidityInvokeTxJson::new(account, address, hex::encode(hex_method), v)
        }
    }
}

impl CommandConversion for SolidityInvokeMessage {
    type T = SolidityInvokeMessage;
    fn to_string(&self) -> Result<String> {
        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        println!("{}", j);

        Ok(j)
    }
    
    fn box_to_raw(&self) -> &dyn Any {
        self
    }
}

pub trait ContractAPI {
    fn deploy(&self);
    fn invoke(&self);
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
    fn deploy(&self) {
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
        println!("resp : {:?}", resp);

        // if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
        //     let x: String = x["result"].to_string();
        //     if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
        //         let x: String = x["info"].to_string();
        //         if let Ok(v) = serde_json::from_str(&x) as Result<ServerInfoResponse, serde_json::error::Error> {
        //             op(Ok(v))
        //         }
        //     }
        // }
    }

    fn invoke(&self) {
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
        println!("resp : {:?}", resp);
    }
}
