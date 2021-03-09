use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::error::Error;
use std::fmt;

/*
@4.8 请求账号信息
RequestLedgerCommand 请求格式
id: u64, //(固定值): 1
command: String, //(固定值): account_info
relation_type: Option<String>, //None
account: String, //需要用户传递的参数，钱包的地址
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountInfoCommand {
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

impl RequestAccountInfoCommand {
    pub fn with_params(account: String) -> Box<Self> {
        Box::new(
            RequestAccountInfoCommand {
                id: 1,
                command: "account_info".to_string(),
                relation_type: None,
                account: account,
                ledger_index: "validated".to_string(),
            }
        )
    }
    
    pub fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        Ok(j)
    }
}

/////////////////////////
/*
RequestAccountInfoResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountInfoResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Balance")]
    pub balance: String,

    #[serde(rename="Flags")]
    pub flags: u64,

    #[serde(rename="LedgerEntryType")]
    pub ledger_entry_type: String,

    #[serde(rename="OwnerCount")]
    pub owner_count: u64,

    #[serde(rename="PreviousTxnID")]
    pub previous_txn_id: String,

    #[serde(rename="PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u64,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="index")]
    pub index: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccounInfoSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountInfoCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for AccounInfoSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccounInfoSideKick is here!")
    }
}

impl Error for AccounInfoSideKick  {
    fn description(&self) -> &str {
        "I'm AccounInfoSideKick side kick"
    }
}
