use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::error::Error;
use std::fmt;

/*
@4.9 获得账号可接收和发送的货币
RequestAccountTumsCommand 请求格式
id: u64, //(固定值): 1
command: String, //(固定值): account_currencies
relation_type: Option<String>, //None
account: String, //需要用户传递的参数，钱包的地址
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTumsCommand {
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

impl RequestAccountTumsCommand {
    pub fn with_params(account: String) -> Box<Self> {
        Box::new(
            RequestAccountTumsCommand {
                id: 1,
                command: "account_currencies".to_string(),
                relation_type: None,
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
RequestAccountInfoResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTumsResponse {
    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,

    #[serde(rename="receive_currencies")]
    pub receive_currencies: Vec<String>,

    #[serde(rename="send_currencies")]
    pub send_currencies: Vec<String>,

    #[serde(rename="validated")]
    pub validated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccounTumSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountTumsCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for AccounTumSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccounTumSideKick is here!")
    }
}

impl Error for AccounTumSideKick  {
    fn description(&self) -> &str {
        "I'm AccounTumSideKick side kick"
    }
}
