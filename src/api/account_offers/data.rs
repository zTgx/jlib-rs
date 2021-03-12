use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::error::Error;
use std::fmt;
use crate::api::message::amount::{Amount, string_or_struct};

/*
@4.11 获得账号挂单
RequestAccountOfferCommand 请求格式
id: u64,         //(固定值): 1
command: String, //(固定值): account_offers
relation_type: Option<String>, //None
account: String,     //需要用户传递的参数，钱包的地址
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountOfferCommand {
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

impl RequestAccountOfferCommand {
    pub fn with_params(account: String) -> Box<Self> {
        Box::new(
            RequestAccountOfferCommand {
                id: 1,
                command: "account_offers".to_string(),
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
RequestAccountOfferResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct TakerPay {
    #[serde(rename="flags")]
    pub flags: u64,

    #[serde(rename="seq")]
    pub seq: u64,

    #[serde(rename="taker_pays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,

    #[serde(rename="taker_gets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakerGet {
    #[serde(rename="flags")]
    pub flags: u64,

    #[serde(rename="seq")]
    pub seq: u64,

    #[serde(rename="taker_gets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,

    #[serde(rename="taker_pays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offers {
    pub taker_pay: Option<TakerPay>,
    pub taker_get: Option<TakerGet>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountOfferResponse {
    #[serde(rename="account")]
    pub account: String,

    // #[serde(rename="ledger_hash")]
    // pub ledger_hash: String,

    #[serde(rename="ledger_current_index")]
    pub ledger_index: u64,

    #[serde(rename="offers")]
    pub offers: Vec<Offers>, //(TakerPay, TakerGet), //???

    #[serde(rename="validated")]
    pub validated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffersSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountOfferCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for AccountOffersSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccountOffersSideKick is here!")
    }
}

impl Error for AccountOffersSideKick  {
    fn description(&self) -> &str {
        "I'm AccountOffersSideKick side kick"
    }
}
