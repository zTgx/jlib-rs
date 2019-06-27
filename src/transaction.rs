#![allow(unused)]

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

use crate::commands::command_trait::CommandConversion;
use crate::common::*;

/*
支付对象:
*/
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MemoData {
    #[serde(rename="MemoData")]
    pub memo_data: String,
}
impl MemoData {
    pub fn new(memo_data: String) -> Self {
        MemoData {
            memo_data: memo_data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Memo {
    #[serde(rename="Memo")]
    pub memo_data: MemoData,
}

impl Memo {
    pub fn new(memo_data: MemoData) -> Self {
        Memo {
            memo_data: memo_data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxJson {
    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="Memos")]
    pub memo: Option<Vec<Memo>>,

    #[serde(rename="Sequence")]
    pub sequence: Option<String>,

    #[serde(rename="SigningPubKey")]
    pub signing_pubKey: Option<String>,

    #[serde(rename="TxnSignature")]
    pub txn_signature: Option<String>,
}

impl TxJson {
    pub fn new(from: String, to: String, amount: Amount, memo: Option<Vec<Memo>>, sequence: Option<String>) -> Self {
        let flag = Flags::Other;
        TxJson {
            flags: flag.get(),
            fee: 10000,
            transaction_type: "Payment".to_string(),
            account: from,
            destination: to,
            amount: "500000".to_string(), //amount ?????
            memo: memo,
            sequence: sequence,
            signing_pubKey: None,
            txn_signature: None,
        }
    }
}
impl CommandConversion for TxJson {
    type T = TxJson;
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


#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTx {
    #[serde(rename="id")]
    id: u64, 

    //如果需要本地签名为false， secret必须，否则可以为空。
    #[serde(rename="secret")]
    pub secret: Option<String>,

    #[serde(rename="command")]
    pub command: String, //Submit

    #[serde(rename="tx_json")]
    pub tx_json: TxJson,
}

impl TransactionTx {
    pub fn new(secret: Option<String>, tx_json: TxJson) -> Box<TransactionTx> {
        Box::new( TransactionTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for TransactionTx {
    type T = TransactionTx;
    fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

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

    // fn to_concrete<T>(&self) -> T {
    //     let def: Box<dyn CommandConversion> = self;
    //     let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //         Some(b) => b,
    //         None => panic!("&a isn't a B!"),
    //     };
        
    //     b
    // }
}

/*
TransactionResponse
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct TxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Destination")]
    pub destination: String,
    
    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Memos")]
    pub memos: Vec<Memo>,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Timestamp")]
    pub time_stamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: TxJsonResponse,
}