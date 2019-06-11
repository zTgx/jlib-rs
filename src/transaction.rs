#![allow(unused)]

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::commands::command_trait::CommandConversion;
use crate::common::*;

/*
支付对象:
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct TxJson {
    #[serde(rename="Flags")]
    pub flags: Flags,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: Amount,

    #[serde(rename="Destination")]
    pub destination: String,
}

impl TxJson {
    pub fn new(from: String, to: String, amount: Amount) -> Self {
        TxJson {
            flags: Flags::Other,
            fee: 10000,
            transaction_type: "Payment".to_string(),
            account: from,
            destination: to,
            amount: amount,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionTx {
    pub tx_json: TxJson,
}

impl TransactionTx {
    pub fn new(tx_json: TxJson) -> Box<TransactionTx> {
        Box::new( TransactionTx {
            tx_json: tx_json,
        })
    }
}