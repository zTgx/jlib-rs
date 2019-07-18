#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::message::common::command_trait::CommandConversion;
use crate::message::common::amount::Amount;
use crate::message::common::meta::*;

use std::error::Error;
use std::fmt;

/*
@4.12获得账号交易列表 
RequestAccountTxCommand 请求格式
id: u64,              //(固定值): 1
command: String,      //(固定值): account_tx
account: String,      //需要用户传递的参数，钱包的地址
ledger_index_min: i32 //(固定值): 0
ledger_index_max: i32 //(固定值): -1
limit: Option<u64>    //需要用户传递的参数，限定返回多少条记录，默认200
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTxCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="account")]
    account: String,

    #[serde(rename="ledger_index_min")]
    ledger_index_min: i32,

    #[serde(rename="ledger_index_max")]
    ledger_index_max: i32,

    #[serde(rename="limit")]
    limit: Option<u64>,
}

impl RequestAccountTxCommand {
    pub fn with_params(account: String, limit: Option<u64>) -> Box<Self> {
        let mut n = Some(200);
        if limit.is_some() {
            n = limit;
        }

        Box::new( 
            RequestAccountTxCommand {
                id: 1,
                command: "account_tx".to_string(),
                account: account,
                ledger_index_min: 0,
                ledger_index_max: -1,
                limit: n,
            }
        )
    }
}

impl CommandConversion for RequestAccountTxCommand {
    type T = RequestAccountTxCommand;
    fn to_string(&self) -> Result<String> {
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

//实现default方法, 此command不提供default方法~
// impl Default for RequestLedgerCommand {
//     fn default() -> Self {
//         RequestLedgerCommand { 
//             id: 1,
//             command: "ledger".to_string(),
//         }
//     }
// }

/////////////////////////
/*
RequestAccountTxResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Marker {
    #[serde(rename="ledger")]
    pub ledger: u64,

    #[serde(rename="seq")]
    pub seq: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tx {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: u64,

    #[serde(rename="OfferSequence")]
    pub offer_sequence: u64,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename="meta")]
    pub meta: Meta,

    #[serde(rename="tx")]
    pub tx: Tx,

    #[serde(rename="validated")]
    pub validated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTxResponse {
    #[serde(rename="account")]
    pub account: String,   

    #[serde(rename="ledger_index_max")]
    pub ledger_index_max: u64,

    #[serde(rename="ledger_index_min")]
    pub ledger_index_min: u64,

    #[serde(rename="marker")]
    pub marker: Marker, 

    #[serde(rename="limit")]
    pub limit: u64, 

    #[serde(rename="transactions")]
    pub transactions: Vec<Transaction>,
}

//AccounTx
#[derive(Debug, Serialize, Deserialize)]
pub struct AccounTxSideKick {
    pub error           : String,
    pub error_code      : i32,
    pub error_message   : String,
    pub id              : u32,
    pub request         : RequestAccountTxCommand,
    pub status          : String,
    
    #[serde(rename="type")]
    pub rtype            : String,
}

impl fmt::Display for AccounTxSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AccounTxSideKick is here!")
    }
}

impl Error for AccounTxSideKick  {
    fn description(&self) -> &str {
        "I'm AccounTxSideKick side kick"
    }
}