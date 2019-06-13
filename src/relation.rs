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
关系对象
*/
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RelationTxJson {
    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Fee")]
    pub fee: u64,

    //交易类型：TrustSet信任;RelationDel解冻；RelationSet授权/冻结
    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Target")]
    pub target: String,

    //关系类型：0信任；1授权；3冻结/解冻；
    #[serde(rename="RelationType")]
    pub relation_type: u64,

    #[serde(rename="LimitAmount")]
    pub limit_amount: String,//Amount,
}

impl RelationTxJson {
    pub fn new(account: String, target: String, relation_type: u64, amount: Amount) -> Self {
        let flag = Flags::Other;
        RelationTxJson {
            flags: flag.get(),
            fee: 10000,
            transaction_type: "RelationSet".to_string(),
            account: account,
            target: target,
            relation_type: relation_type,
            limit_amount: "500000".to_string(),//amount, ?????
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationTx {
    #[serde(rename="id")]
    id: u64, 

    #[serde(rename="command")]
    pub command: String, //Submit

    //如果需要本地签名为false， secret必须，否则可以为空。
    #[serde(rename="secret")]
    pub secret: Option<String>,

    #[serde(rename="tx_json")]
    pub tx_json: RelationTxJson,
}

impl RelationTx {
    pub fn new(secret: Option<String>, tx_json: RelationTxJson) -> Box<RelationTx> {
        Box::new( RelationTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for RelationTx {
    type T = RelationTx;
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
RelationTxJsonResponse
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RelationTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="LimitAmount")]
    pub limit_amount: String,
    
    #[serde(rename="RelationType")]
    pub relation_type: u64,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Target")]
    pub target: String,

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
pub struct RelationTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: RelationTxJsonResponse,
}