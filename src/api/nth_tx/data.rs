
use serde_json::Result;

use std::fmt; //fmt METHOD
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

extern crate void;
use void::Void;

use crate::message::common::meta::*;
use crate::message::common::amount::{Amount, string_or_struct};
use crate::message::tx_flags::*;

use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTxCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="transaction")]
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

    pub fn to_string(&self) -> Result<String> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTxResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Timestamp")]
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

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
    pub meta: Option<Meta>,

    #[serde(rename="validated")]
    pub validated: bool,

    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,

    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,

    #[serde(rename="Memos")]
    pub memos: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecTxSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestTxCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for SpecTxSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SpecTxSideKick is here!")
    }
}

impl Error for SpecTxSideKick  {
    fn description(&self) -> &str {
        "I'm SpecTxSideKick side kick"
    }
}
