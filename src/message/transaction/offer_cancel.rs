#![allow(unused)]

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

use crate::message::common::command_trait::CommandConversion;
use crate::misc::common::*;

/*
4.19取消挂单
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTxJson {
    #[serde(rename="Flags")]
    pub flags: i32, ///How ???????????

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="OfferSequence")]
    pub offer_sequence: u64,
}

impl OfferCancelTxJson {
        pub fn new(account: String, offer_sequence: u64) -> Self {
            let flag = Flags::Other;
            OfferCancelTxJson {
                flags: 0, ///////////////Hard code
                fee: 10000, /////////////////////Hard code
                transaction_type: "OfferCancel".to_string(),
                account: account,
                offer_sequence: offer_sequence,
            }
        }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTx {
    #[serde(rename="id")]
    id: u64, 

    #[serde(rename="command")]
    pub command: String, //Submit

    //如果需要本地签名为false， secret必须，否则可以为空。
    #[serde(rename="secret")]
    pub secret: Option<String>,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCancelTxJson,
}

impl OfferCancelTx {
    pub fn new(secret: Option<String>, tx_json: OfferCancelTxJson) -> Box<OfferCancelTx> {
        Box::new( OfferCancelTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for OfferCancelTx {
    type T = OfferCancelTx;
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
OfferCancelTxJsonResponse
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

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
pub struct OfferCancelTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCancelTxJsonResponse,
}