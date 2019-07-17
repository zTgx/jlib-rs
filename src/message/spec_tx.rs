
#![allow(unused)]

use serde_json::json;
use serde_json::Result;
use std::any::Any;

use std::fmt; //fmt METHOD
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

extern crate void;
use void::Void;

use crate::message::command_trait::CommandConversion;
use crate::message::meta::*;
use crate::message::amount::{Amount, string_or_struct};
use crate::misc::common::*;


//////////////////////
/*
@4.7查询某一交易具体信息
RequestTxCommand 请求格式
id: u64,          //为(固定值): 1
command: String,  //为(固定值): tx
hash: String,     //需要用户传递的参数，[交易hash]
*/
#[derive(Serialize, Deserialize)]
pub struct RequestTxCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="transaction")]
    hash: String,
}

impl RequestTxCommand {
    pub fn with_params(hash: String) -> Box<Self> {
        Box::new( 
            RequestTxCommand {
                id: 1,
                command: "tx".to_string(),
                hash: hash,
            }
        )
    }
}

impl CommandConversion for RequestTxCommand {
    type T = RequestTxCommand;
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
        // if let Ok(x) = value.downcast::<T>() {
        //     x
        // }

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

//实现default方法
// impl Default for RequestTxCommand {
//     fn default() -> Self {
//         ServerInfoCommand { 
//             id: 1,
//             command: "server_info".to_string(),
//         }
//     }
// }

///////////////////////////////
/*
RequestTxResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTxResponse {
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
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="date")]
    pub date: u64,

    #[serde(rename="hash")]
    pub hash: String,

    #[serde(rename="inLedger")]
    pub in_ledger: u64,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,

    #[serde(rename="meta")]
    pub meta: Option<Meta>,

    #[serde(rename="validated")]
    pub validated: bool,

    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,

    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             

    #[serde(rename="Memos")]
    pub memos: Option<Vec<String>>,
}



