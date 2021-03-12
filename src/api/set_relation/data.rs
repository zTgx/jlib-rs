#![allow(unused)]

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};

use crate::message::common::amount::{Amount, string_or_struct};
use crate::message::tx_flags::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RelationType {
    TRUST     = 0,
    AUTHORIZE = 1,
    FREEZE    = 3,
}
impl RelationType {
    pub fn get(&self) -> u32 {
        match *self {
            RelationType::TRUST     => { 0 },
            RelationType::AUTHORIZE => { 1 },
            RelationType::FREEZE    => { 3 },
        }
    }
}

/*
LimitAmount Can't be Native.!!!
*/
#[derive(Deserialize, Debug, Default)]
pub struct RelationTxJson {
    #[serde(rename="Flags")]
    pub flags: u32,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Target")]
    pub target: String,

    #[serde(rename="RelationType")]
    pub relation_type: u32,

    #[serde(rename="LimitAmount")]
    #[serde(deserialize_with = "string_or_struct")]
    pub limit_amount: Amount,
}

impl RelationTxJson {
    pub fn new(account: String, target: String, rtype: u32, amount: Amount) -> Self {
        let flag = Flags::Other;

        RelationTxJson {
            flags: flag.get(),
            fee: 10000,
            transaction_type: "RelationSet".to_string(),
            account: account,
            target: target,
            relation_type: rtype,
            limit_amount: amount,
        }
    }
}

impl Serialize for RelationTxJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 7 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("RelationTxJson", 7)?;

        state.serialize_field("Flags", &self.flags)?;
        state.serialize_field("Fee", &self.fee)?;
        state.serialize_field("TransactionType", &self.transaction_type)?;
        state.serialize_field("Account", &self.account)?;

        state.serialize_field("Target", &self.target)?;
        state.serialize_field("RelationType", &self.relation_type)?;

        if self.limit_amount.is_string() {
            state.serialize_field("LimitAmount", &Amount::mul_million(&self.limit_amount.value))?;
        } else {
            state.serialize_field("LimitAmount", &self.limit_amount)?;
        }

        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationTx {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    pub command: String,

    #[serde(rename="secret")]
    pub secret: String,

    #[serde(rename="tx_json")]
    pub tx_json: RelationTxJson,
}

impl RelationTx {
    pub fn new(secret: String, tx_json: RelationTxJson) -> Box<RelationTx> {
        Box::new( RelationTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }

    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="LimitAmount")]
    #[serde(deserialize_with = "string_or_struct")]
    pub limit_amount: Amount,

    #[serde(rename="RelationType")]
    pub relation_type: u64,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Target")]
    pub target: String,

    #[serde(rename="Timestamp")]
    pub time_stamp: Option<u64>,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: RelationTxJsonResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RelationTx,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for RelationSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RelationSideKick is here!")
    }
}

impl Error for RelationSideKick  {
    fn description(&self) -> &str {
        "I'm RelationSideKick side kick"
    }
}
