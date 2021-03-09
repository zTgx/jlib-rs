use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::error::Error;
use std::fmt;

/*
@4.5获取最新账本信息
LedgerClosedCommand 请求格式
id: u64
command: String
值分别为(固定值): 1, "ledger_closed"
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerClosedCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,
}

impl LedgerClosedCommand {
    pub fn with_params(id: u64, command: String) -> Box<Self> {
        Box::new(
            LedgerClosedCommand {
                id: id,
                command: command,
            }
        )
    }

    pub fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        Ok(j)
    }
}

//实现default方法
impl Default for LedgerClosedCommand {
    fn default() -> Self {
        LedgerClosedCommand {
            id: 1,
            command: "ledger_closed".to_string(),
        }
    }
}

/////////////////////////
/*
LedgerClosedResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct LedgerClosedResponse {
    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,
}

/*
LedgerClosedSideKick 数据返回格式
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerClosedSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : LedgerClosedCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for LedgerClosedSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LedgerClosedSideKick is here!")
    }
}

impl Error for LedgerClosedSideKick  {
    fn description(&self) -> &str {
        "I'm LedgerClosedSideKick side kick"
    }
}
