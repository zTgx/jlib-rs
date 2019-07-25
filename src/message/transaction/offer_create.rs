#![allow(unused)]

extern crate void;

use serde::ser::{Serializer, SerializeStruct};
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};

use void::Void;

use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

use crate::message::common::command_trait::CommandConversion;
use crate::message::common::amount::*;
use crate::misc::common::*;
use crate::OfferType;

/*
挂单对象
*/
#[derive(Deserialize, Debug)]
pub struct OfferCreateTxJson {
    #[serde(rename="Flags")]
    pub flags: u32, 

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount, 

    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,
}

impl OfferCreateTxJson {
    pub fn new(account: String, offer_type: &'static str, taker_gets: Amount,  taker_pays: Amount) -> Self {
        let flags = 0;
        
        OfferCreateTxJson {
            flags: OfferCreateTxJson::get_flags( offer_type ),
            fee: 10000, 
            transaction_type: "OfferCreate".to_string(),
            account: account,
            taker_pays: taker_pays,
            taker_gets: taker_gets,
        }
    }

    pub fn get_flags(offer_type: &'static str) -> u32 {
        if offer_type == "Sell" {
            let flag = Flags::OfferCreate{ name: OfferCreate::Sell };
            return flag.get();
        }

        0u32
    }
}

impl Serialize for OfferCreateTxJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("OfferCreateTxJson", 6)?;
        
        state.serialize_field("Flags", &self.flags)?;
        state.serialize_field("Fee", &self.fee)?;
        state.serialize_field("TransactionType", &self.transaction_type)?;
        state.serialize_field("Account", &self.account)?;
        if self.taker_gets.is_string () {
            state.serialize_field("TakerGets", &self.taker_gets.value)?;
        } else {
            state.serialize_field("TakerGets", &self.taker_gets)?;
        }

        if self.taker_pays.is_string () {
            state.serialize_field("TakerPays", &self.taker_pays.value)?;
        } else {
            state.serialize_field("TakerPays", &self.taker_pays)?;
        }

        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTx {
    #[serde(rename="id")]
    pub id: u64, 

    #[serde(rename="command")]
    pub command: String, //Submit

    //如果需要本地签名为false， secret必须，否则可以为空。
    #[serde(rename="secret")]
    pub secret: String,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCreateTxJson,
}

impl OfferCreateTx {
    pub fn new(secret: String, tx_json: OfferCreateTxJson) -> Box<OfferCreateTx> {
        Box::new( OfferCreateTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for OfferCreateTx {
    type T = OfferCreateTx;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        println!("{}", j);

        Ok(j)
    }
    
    fn box_to_raw(&self) -> &dyn Any {
        self
    }

    // fn to_concrete<T>(&self) -> T {
    //     let def: Box<dyn CommandConversion> = self;
    //     let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //         Some(b) => b,
    //         None => panic!("&a isn't a B!"),
    //     };
        
    //     b
    // }
}

/*
OfferCreateTxJsonResponse
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTxJsonResponse {
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

    #[serde(rename="TakerGets")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_gets: Amount,

    #[serde(rename="TakerPays")]
    #[serde(deserialize_with = "string_or_struct")]
    pub taker_pays: Amount,

    #[serde(rename="Timestamp")]
    pub time_stamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OfferCreateTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: OfferCreateTxJsonResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferCreateSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : OfferCreateTx,
    pub status          : String,
    
    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for OfferCreateSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OfferCreateSideKick is here!")
    }
}

impl Error for OfferCreateSideKick  {
    fn description(&self) -> &str {
        "I'm OfferCreateSideKick side kick"
    }
}