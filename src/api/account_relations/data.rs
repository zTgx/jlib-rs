use serde::{Deserialize, Serialize};
use serde_json::Result;
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

    pub fn to_string(&self) -> Result<String> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }
}

/////////////////////////
/*
RequestAccountRelationsResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountRelationsResponse {
    #[serde(rename="account")]
    pub account: String,

    // #[serde(rename="ledger_hash")]
    // pub ledger_hash: String,

    #[serde(rename="ledger_current_index")]
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
