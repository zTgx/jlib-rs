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
挂单对象
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTxJson {
    #[serde(rename="Flags")]
    pub flags: i32, ///How ???????????

    #[serde(rename="Fee")]
    pub fee: u64,

    // #[serde(rename="OfferType")]
    // pub offer_type: String,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    // //应用来源序号（正整数），可选
    // #[serde(rename="App")]
    // pub app: Option<u64>,

    #[serde(rename="TakerPays")]
    taker_pays: String, //WHAT>>>>>>>>>>>>>>>>>>>>>>>

    #[serde(rename="TakerGets")]
    taker_gets: AmountTest,
}

impl OfferCreateTxJson {
        pub fn new(account: String, taker_gets: AmountTest,  taker_pays: String) -> Self {
            let flag = Flags::Other;
            OfferCreateTxJson {
                flags: 524288, ///////////////Hard code
                fee: 10000, /////////////////////Hard code
                transaction_type: "OfferCreate".to_string(),
                account: account,
                taker_pays: taker_pays,//amount, ?????
                taker_gets: taker_gets,
            }
        }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTx {
    #[serde(rename="id")]
    id: u64, 

    #[serde(rename="command")]
    pub command: String, //Submit

    //如果需要本地签名为false， secret必须，否则可以为空。
    #[serde(rename="secret")]
    pub secret: Option<String>,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCreateTxJson,
}

impl OfferCreateTx {
    pub fn new(secret: Option<String>, tx_json: OfferCreateTxJson) -> Box<OfferCreateTx> {
        Box::new( OfferCreateTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for OfferCreateTx {
    type T = OfferCreateTx;
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
OfferCreateTxJsonResponse
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTxJsonResponse {
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

    #[serde(rename="TakerGets")]
    pub taker_gets: AmountTest,

    #[serde(rename="TakerPays")]
    pub taker_pays: String,

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
pub struct OfferCreateTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCreateTxJsonResponse,
}