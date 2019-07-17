#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::message::common::command_trait::CommandConversion;

/*
@4.6获取某一账本具体信息
RequestLedgerCommand 请求格式
id: u64
command: String
ledger_index: Option<String>
ledger_hash: Option<String> 
transactions: bool
值分别为(固定值): 1, "ledger", ledger_index/ledger_hash 二选一， transactions 为bool类型必需.
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestLedgerCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="ledger_index")]
    ledger_index: Option<u64>,

    #[serde(rename="ledger_hash")]
    ledger_hash: Option<String>,

    #[serde(rename="transactions")]
    transactions: bool,
}

impl RequestLedgerCommand {
    pub fn with_params(ledger_index: Option<u64>, ledger_hash: Option<String>, transactions: bool) -> Box<Self> {
        Box::new( 
            RequestLedgerCommand {
                id: 1,
                command: "ledger".to_string(),
                ledger_index: ledger_index,
                ledger_hash: ledger_hash,
                transactions: transactions,
            }
        )
    }
}

impl CommandConversion for RequestLedgerCommand {
    type T = RequestLedgerCommand;
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

//实现default方法, 此command不提供default方法~
// impl Default for RequestLedgerCommand {
//     fn default() -> Self {
//         RequestLedgerCommand { 
//             id: 1,
//             command: "ledger".to_string(),
//         }
//     }
// }

/////////////////////////
/*
RequestLedgerResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestLedgerResponse {
    #[serde(rename="accepted")]
    pub accepted: bool,

    #[serde(rename="account_hash")]
    pub account_hash: String,

    #[serde(rename="close_time")]
    pub close_time: u64,

    #[serde(rename="close_time_human")]
    pub close_time_human: String,

    #[serde(rename="close_time_resolution")]
    pub close_time_resolution: u64,

    #[serde(rename="closed")]
    pub closed: bool,

    #[serde(rename="hash")]
    pub hash: String,

    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: String,

    #[serde(rename="parent_hash")]
    pub parent_hash: String,

    #[serde(rename="seqNum")]
    pub seq_num: String,

    #[serde(rename="totalCoins")]
    pub total_coins: String,

    #[serde(rename="total_coins")]
    pub total_coins_x: String,

    #[serde(rename="transaction_hash")]
    pub transaction_hash: String,

    #[serde(rename="transactions")]
    pub transactions: Vec<String>,
}