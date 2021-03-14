use serde::{Deserialize, Serialize};

use crate::api::utils::tx_flags::*;
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTxJson {
    #[serde(rename="Flags")]
    pub flags: u32,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="OfferSequence")]
    pub offer_sequence: u64,
}

impl OfferCancelTxJson {
        pub fn new(account: String, offer_sequence: u64) -> Self {
            let flag = Flags::Other;
            OfferCancelTxJson {
                flags: flag.get(),
                fee  : 10000,
                transaction_type: "OfferCancel".to_string(),
                account: account,
                offer_sequence: offer_sequence,
            }
        }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTx {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    pub command: String,

    #[serde(rename="secret")]
    pub secret: String,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCancelTxJson,
}

impl OfferCancelTx {
    pub fn new(secret: String, tx_json: OfferCancelTxJson) -> Box<OfferCancelTx> {
        Box::new( OfferCancelTx {
            id     : 1,
            command: "submit".to_string(),
            secret : secret,
            tx_json: tx_json,
        })
    }
    
    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="OfferSequence")]
    pub offer_sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCancelTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: Option<OfferCancelTxJsonResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferCancelSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : OfferCancelTx,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for OfferCancelSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OfferCancelSideKick is here!")
    }
}

impl Error for OfferCancelSideKick  {
    fn description(&self) -> &str {
        "I'm OfferCancelSideKick side kick"
    }
}
