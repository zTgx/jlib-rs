#![allow(unused)]

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

use crate::message::common::command_trait::CommandConversion;
use crate::message::common::amount::{Amount, string_or_struct};
use crate::message::common::memo::*;
use crate::message::tx_flags::*;
use std::error::Error;
use std::fmt;

#[derive(Deserialize, Debug, Default)]
pub struct TxJson {
    #[serde(rename="Flags")]
    pub flags: u32,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    #[serde(deserialize_with = "string_or_struct")]
    pub amount: Amount,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="Memos")]
    pub memo: Option<Vec<Memos>>,

    #[serde(rename="Sequence")]
    pub sequence: u32,
}

impl Serialize for TxJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 6 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("OfferCreateTxJson", 6)?;

        state.serialize_field("Flags", &self.flags)?;
        state.serialize_field("Fee", &self.fee)?;
        state.serialize_field("TransactionType", &self.transaction_type)?;
        state.serialize_field("Account", &self.account)?;

        state.serialize_field("Amount", &self.amount)?;
        //state.serialize_field("Amount", &Amount::mul_million(&self.amount))?;
        state.serialize_field("Destination", &self.destination)?;
        state.serialize_field("Memos", &self.memo)?;

        state.end()
    }
}

impl TxJson {
    pub fn new(from: String, to: String, amount: Amount, sequence: u32, memo: Option<Vec<Memos>>) -> Self {
        let flag = Flags::Other;
        TxJson {
            flags: flag.get(),
            fee: 10000,
            transaction_type: "Payment".to_string(),
            account: from,
            destination: to,
            amount: amount,//(amount.value.parse::<f64>().unwrap() * 1000000f64).to_string(),
            sequence: sequence,
            memo: memo,
        }
    }
}
impl CommandConversion for TxJson {
    type T = TxJson;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }

    fn box_to_raw(&self) -> &dyn Any {
        self
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTx {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="secret")]
    pub secret: String,

    #[serde(rename="command")]
    pub command: String,

    #[serde(rename="tx_json")]
    pub tx_json: TxJson,
}

impl TransactionTx {
    pub fn new(secret: String, tx_json: TxJson) -> Box<TransactionTx> {
        Box::new( TransactionTx {
            id: 1,
            command: "submit".to_string(),
            secret : secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for TransactionTx {
    type T = TransactionTx;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }

    fn box_to_raw(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Memos")]
    pub memos: Option<Vec<Memo>>,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

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
pub struct TransactionTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: TxJsonResponse,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : TransactionTx,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for PaymentSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PaymentSideKick is here!")
    }
}

impl Error for PaymentSideKick  {
    fn description(&self) -> &str {
        "I'm PaymentSideKick side kick"
    }
}
