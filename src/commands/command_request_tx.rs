
#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::commands::command_trait::CommandConversion;
use crate::commands::command_request_account_tx::Meta;

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

    #[serde(rename="hash")]
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

    #[serde(rename="AppType")]
    pub app_type: u64,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Memos")]
    pub memos: Vec<String>,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pubKey: String,

    #[serde(rename="Timestamp")]
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transactionType: String,

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
    pub meta: Meta,

    #[serde(rename="validated")]
    pub validated: bool,

}