#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::message::common::command_trait::CommandConversion;
use std::error::Error;
use std::fmt;

/*
@4.14获得挂单佣金设置信息
RequestBrokerageCommand 请求格式
id: u64,         //(固定值): 1
command: String, //(固定值): Fee_Info
issuer: String, //需要用户传递的参数，[货币发行方]
app_type: u64,          //需要用户传递的参数，[应用来源]
currency: String,       //需要用户传递的参数，[货币种类]
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBrokerageCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="issuer")]
    issuer: String,

    #[serde(rename="app_type")]
    app_type: u64,

    #[serde(rename="currency")]
    currency: String,

    #[serde(rename="ledger_index")]
    ledger_index: String,
}

impl RequestBrokerageCommand {
    pub fn with_params(issuer: String, app_type: u64, currency: String) -> Box<Self> {
        Box::new(
            RequestBrokerageCommand {
                id: 1,
                command: "Fee_Info".to_string(),
                issuer: issuer,
                app_type: app_type,
                currency: currency,
                ledger_index: "validated".to_string(),
            }
        )
    }
}

impl CommandConversion for RequestBrokerageCommand {
    type T = RequestBrokerageCommand;
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
RequestBrokerageResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBrokerageResponse {
    #[serde(rename="AppType")]
    pub app_type: String,

    #[serde(rename="currency")]
    pub currency: String,

    #[serde(rename="issuer")]
    pub issuer: String,

    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,

    #[serde(rename="rate_den")]
    pub rate_den: String,

    #[serde(rename="rate_num")]
    pub rate_num: String,

    #[serde(rename="validated")]
    pub validated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrokerageSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestBrokerageCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for BrokerageSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BrokerageSideKick is here!")
    }
}

impl Error for BrokerageSideKick  {
    fn description(&self) -> &str {
        "I'm BrokerageSideKick side kick"
    }
}
