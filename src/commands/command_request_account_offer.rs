#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::commands::command_trait::CommandConversion;

/*
@4.10 获得账号关系 
RequestAccountOfferCommand 请求格式
id: u64,         //(固定值): 1
command: String, //(固定值): account_offers
relation_type: Option<String>, //None
account: String,     //需要用户传递的参数，钱包的地址
ledger_index: String //(固定值): 'validated'
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountOfferCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="relation_type")]
    relation_type: Option<String>,

    #[serde(rename="account")]
    account: String,

    #[serde(rename="ledger_index")]
    ledger_index: String,
}

impl RequestAccountOfferCommand {
    pub fn with_params(account: String) -> Box<Self> {
        Box::new( 
            RequestAccountOfferCommand {
                id: 1,
                command: "account_offers".to_string(),
                relation_type: None,
                account: account,
                ledger_index: "validated".to_string(),
            }
        )
    }
}

impl CommandConversion for RequestAccountOfferCommand {
    type T = RequestAccountOfferCommand;
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
RequestAccountOfferResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    #[serde(rename="currency")]
    currency: String,//'USD',

    #[serde(rename="issuer")]
    issuer: String,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',

    #[serde(rename="value")]
    value: String,   //'0.5'
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakerPay {
    #[serde(rename="flags")]
    flags: u64,

    #[serde(rename="seq")]
    seq: u64,

    #[serde(rename="taker_pays")]
    taker_pays: Token,

    #[serde(rename="taker_gets")]
    taker_gets: String,//'1000000'
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakerGet {
    #[serde(rename="flags")]
    flags: u64,

    #[serde(rename="seq")]
    seq: u64,

    #[serde(rename="taker_gets")]
    taker_gets: Token,

    #[serde(rename="taker_pays")]
    taker_pays: String, //'1000000'
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountOfferResponse {
    #[serde(rename="account")]
    pub account: String,   

    #[serde(rename="ledger_hash")]
    pub ledger_hash: String,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,

    #[serde(rename="offers")]
    pub offers: (TakerPay, TakerGet), //???

    #[serde(rename="validated")]
    pub validated: bool,
}
