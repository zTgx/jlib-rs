#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::message::command_trait::CommandConversion;
use crate::misc::common::*;
use crate::message::amount::Amount;

/*
@4.13获得市场挂单列表
RequestOrderBookCommand 请求格式
id: u64,              //(固定值): 1
command: String,      //(固定值): book_offers
taker_gets, Item,//对家想要[获得]的货币信息
taker_pays, Item,//对家想要[支付]的货币信息
taker: String, //'(固定值): jjjjjjjjjjjjjjjjjjjjBZbvri (SWTC的银关地址)
*/
//名字暂定OrderBookItem
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
}

impl CommandConversion for RequestOrderBookCommand {
    type T = RequestOrderBookCommand;
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
RequestOrderBookResponse 数据返回格式
*/
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Amount {
//     #[serde(rename="value")]
//     value: String,

//     #[serde(rename="currency")]
//     currency: String,

//     #[serde(rename="issuer")]
//     issuer: String,
// }

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
    pub taker_gets: String, //未充分测试: 对方得到的。（买卖双方，当货币是swt时，数据类型为对象；否则为string）

    #[serde(rename="TakerPays")]
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