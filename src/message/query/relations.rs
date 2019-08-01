#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::message::common::command_trait::CommandConversion;
use std::error::Error;
use std::fmt;

/*
@4.10 获得账号关系
RequestAccountRelationsCommand 请求格式
id: u64,         //(固定值): 1
command: String, //(固定值): account_lines
relation_type: Option<String>, //None
account: String,     //需要用户传递的参数，钱包的地址
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountRelationsCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="relation_type")]
    relation_type: Option<String>,

    #[serde(rename="account")]
    account: String,

    #[serde(rename="ledger_index")]
    ledger_index: String,
}

impl RequestAccountRelationsCommand {
    pub fn with_params(account: String, relation_type: Option<String>) -> Box<Self> {
        Box::new(
            RequestAccountRelationsCommand {
                id: 1,
                command: "account_lines".to_string(),
                relation_type: relation_type,
                account: account,
                ledger_index: "validated".to_string(),
            }
        )
    }
}

impl CommandConversion for RequestAccountRelationsCommand {
    type T = RequestAccountRelationsCommand;
    fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
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
RequestAccountRelationsResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountRelationsResponse {
    #[serde(rename="account")]
    pub account: String,

    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,

    #[serde(rename="lines")]
    pub lines: Vec<Line>,

    #[serde(rename="validated")]
    pub validated: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    #[serde(rename="account")]
    pub account: String,

    #[serde(rename="balance")]
    pub balance: String,

    #[serde(rename="currency")]
    pub currency: String,

    #[serde(rename="limit")]
    pub limit: String,

    #[serde(rename="limit_peer")]
    pub limit_peer: String,

    #[serde(rename="no_skywell")]
    pub no_skywell: bool,

    #[serde(rename="quality_in")]
    pub quality_in: u64,

    #[serde(rename="quality_out")]
    pub quality_out: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationsSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountRelationsCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for RelationsSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RelationsSideKick is here!")
    }
}

impl Error for RelationsSideKick  {
    fn description(&self) -> &str {
        "I'm RelationsSideKick side kick"
    }
}
