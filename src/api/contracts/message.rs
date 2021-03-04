#![allow(unused)]
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;
use std::cell::RefCell;

use crate::message::common::command_trait::CommandConversion;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use hex;
use crate::base::misc::util::check;

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

    pub fn with_params(account: String, secret: String, payload: String) -> Self {
        SolidityInitMessage {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: SolidityInitTxJson::new(account, payload),
        }
    }
}

impl CommandConversion for SolidityInitMessage {
    type T = SolidityInitMessage;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

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
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        Ok(j)
    }

    fn box_to_raw(&self) -> &dyn Any {
        self
    }
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
    pub parameter: String, //Hex String, 0xb6e456bb ===>getUnit 

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
