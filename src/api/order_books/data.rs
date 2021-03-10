use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::message::common::amount::{Amount, string_or_struct};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OrderBookItem {
    #[serde(rename="currency")]
    pub currency: String,

    #[serde(rename="issuer")]
    pub issuer: String,
}

impl OrderBookItem {
    pub fn with_params(currency: String, issuer: String) -> Self {
        OrderBookItem {
            currency: currency,
            issuer: issuer,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestOrderBookCommand {
    #[serde(rename="id")]
    pub id: u64,

    #[serde(rename="command")]
    pub command: String,

    #[serde(rename="taker_gets")]
    pub taker_gets: OrderBookItem,

    #[serde(rename="taker_pays")]
    pub taker_pays: OrderBookItem,

    #[serde(rename="taker")]
    pub taker: String,
}

impl RequestOrderBookCommand {
    pub fn with_params(gets: OrderBookItem, pays: OrderBookItem) -> Box<Self> {
        Box::new(
            RequestOrderBookCommand {
                id: 1,
                command: "book_offers".to_string(),
                taker_gets: gets,
                taker_pays: pays,
                taker: "jjjjjjjjjjjjjjjjjjjjBZbvri".to_string(),
            }
        )
    }

    pub fn to_string(&self) -> Result<String> {
        let j = serde_json::to_string(&self)?;

        Ok(j)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="BookDirectory")]
    pub book_directory: String,

    #[serde(rename="BookNode")]
    pub book_node: String,

    #[serde(rename="Flags")]
    pub flags: u64,

    #[serde(rename="LedgerEntryType")]
    pub ledger_entry_type: String,

    #[serde(rename="OwnerNode")]
    pub owner_node: String,

    #[serde(rename="PreviousTxnID")]
    pub previous_txn_id: String,

    #[serde(rename="PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: u64,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,

    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,

    #[serde(rename="index")]
    pub index: String,

    #[serde(rename="owner_funds")]
    pub owner_funds: Option<String>,

    #[serde(rename="quality")]
    pub quality: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestOrderBookResponse {
    #[serde(rename="ledger_current_index")]
    pub ledger_current_index: u64,

    #[serde(rename="offers")]
    pub offers: Vec<Offer>,

    #[serde(rename="validated")]
    pub validated: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestOrderBookCommand,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for OrderBookSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OrderBookSideKick is here!")
    }
}

impl Error for OrderBookSideKick  {
    fn description(&self) -> &str {
        "I'm OrderBookSideKick side kick"
    }
}
