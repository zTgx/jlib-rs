#![allow(unused)]

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

use crate::message::common::amount::*;
use serde::ser::{Serializer, SerializeStruct};

use crate::message::common::command_trait::CommandConversion;
use crate::misc::common::*;
use std::error::Error;
use std::fmt;

/*
5.22.3设置挂单佣金
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SetBrokerageTxJson {
    #[serde(rename="Flags")]
    pub flags: u32,

    #[serde(rename="Fee")]
    pub fee: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub manage_account: String,

    #[serde(rename="OfferFeeRateNum")]
    pub offer_feerate_num: u64,

    #[serde(rename="OfferFeeRateDen")]
    pub offer_feerate_den: u64,

    #[serde(rename="FeeAccountID")]
    pub  fee_account: String,

    #[serde(rename="Amount")]
    #[serde(deserialize_with = "string_or_struct")]
    pub amount: Amount,
}

impl SetBrokerageTxJson {
        pub fn new(account: String, fee_account: String, offer_feerate_num: u64, offer_feerate_den: u64, amount: Amount) -> Self {
            let flag = Flags::Other;
            SetBrokerageTxJson {
                flags: flag.get(),
                fee  : 10000,
                transaction_type: "Brokerage".to_string(),
                manage_account: account,
                offer_feerate_num: offer_feerate_num,
                offer_feerate_den: offer_feerate_den,
                fee_account: fee_account,
                amount: amount,
            }
        }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SetBrokerageTx {
    #[serde(rename="command")]
    pub command: String,

    #[serde(rename="secret")]
    pub secret: String,

    #[serde(rename="tx_json")]
    pub tx_json: SetBrokerageTxJson,
}

impl SetBrokerageTx {
    pub fn new(secret: String, tx_json: SetBrokerageTxJson) -> Box<SetBrokerageTx> {
        Box::new( SetBrokerageTx {
            command: "submit".to_string(),
            secret : secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for SetBrokerageTx {
    type T = SetBrokerageTx;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
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
OfferCancelTxJsonResponse
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SetBrokerageTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Amount")]
    pub amount: Amount,

    #[serde(rename="feeAccountID")]
    pub fee_account: String,

    #[serde(rename="OfferFeeRateDen")]
    pub den: i32,

    #[serde(rename="OfferFeeRateNum")]
    pub num: i32,

    #[serde(rename="Sequence")]
    pub sequence: u64,

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
pub struct SetBrokerageTxResponse {
    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: i32,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: Option<SetBrokerageTxJsonResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetBrokerageSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : SetBrokerageTx,
    pub status          : String,

    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for SetBrokerageSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SetBrokerageSideKick is here!")
    }
}

impl Error for SetBrokerageSideKick  {
    fn description(&self) -> &str {
        "I'm SetBrokerageSideKick side kick"
    }
}
