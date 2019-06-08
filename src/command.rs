/*
此crate包含了所有到command, 已经实现到command有：
1, SubscribeCommand
2, ServerInfoCommand
*/

#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use std::any::Any;


//command转换相关的trait
pub trait CommandConversion {
    type T;
    fn to_string(&self) -> Result<String>;
    fn box_to_raw(&self) -> &dyn Any;

    //TODO::待实现 Box<T> -> T的转换
    //fn to_concrete<T>(&self, value: Box<dyn Any>) -> T ;
}

/*
SubscribeCommand 请求格式
id: u64
command: String
streams: Vec<String>
值分别为(固定值): 0, "subscribe", ["ledger","server","transactions"]
*/
#[derive(Serialize, Deserialize)]
pub struct SubscribeCommand {
    #[serde(rename="id")]
    id: u64, 

    #[serde(rename="command")]
    command: String, 

    #[serde(rename="streams")]
    streams: Vec<String>,
}

impl SubscribeCommand {
    //应对以后变动，先实现一个有参数到方法
    pub fn with_params(id: u64, command: String, streams: Vec<String>) -> Box<Self> {
        Box::new( SubscribeCommand {
            id: id,
            command: command,
            streams: streams,
        } )
    }
}

impl CommandConversion for SubscribeCommand {
    type T = SubscribeCommand;
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
        // if let Ok(x) = value.downcast::<T>() {
        //     x
        // }

        self
    }

    // fn to_concrete<T>(&self) -> T {
    //     let def: Box<dyn CommandConversion> = Box::new(self);
    //     let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //         Some(b) => b,
    //         None => panic!("&a isn't a B!"),
    //     };
        
    //     b
    // }
}

//实现default方法
impl Default for SubscribeCommand {
    fn default() -> Self {
        SubscribeCommand { 
            id: 0,
            command: "subscribe".to_string(),
            streams: vec!["ledger".to_string(),"server".to_string(),"transactions".to_string()],
        }
    }
}

/*
SubscribeResponse 数据格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeResponse {
    #[serde(rename="fee_base")]
    pub fee_base: u64,

    #[serde(rename="fee_ref")]
    fee_ref: u64, 

    #[serde(rename="hostid")]
    hostid: String,

    #[serde(rename="ledger_hash")] 
    ledger_hash: String,

    #[serde(rename="ledger_index")]
    ledger_index: u64,

    #[serde(rename="ledger_time")]
    ledger_time: u64, 

    #[serde(rename="load_base")]
    load_base: u64, 

    #[serde(rename="load_factor")]
    load_factor: u64,
    
    #[serde(rename="pubkey_node")]
    pubkey_node: String, 

    #[serde(rename="random")]
    random: String,

    #[serde(rename="reserve_base")]
    reserve_base: u64,

    #[serde(rename="reserve_inc")]
    reserve_inc: u64,

    #[serde(rename="server_status")]
    server_status: String,

    #[serde(rename="validated_ledgers")]
    validated_ledgers: String, 
}
impl SubscribeResponse {
    pub fn with_params(fee_base: u64, fee_ref: u64, hostid: String, ledger_hash: String, ledger_index: u64, ledger_time: u64,
                       load_base: u64, load_factor: u64, pubkey_node: String, random: String, reserve_base: u64, reserve_inc: u64,
                       server_status: String, validated_ledgers: String) -> Box<Self> {
        
        Box::new( SubscribeResponse {
            fee_base: fee_base,
            fee_ref: fee_ref,
            hostid: hostid,
            ledger_hash: ledger_hash,
            ledger_index: ledger_index,
            ledger_time: ledger_time,
            load_base: load_base,
            load_factor: load_factor,
            pubkey_node: pubkey_node,
            random: random,
            reserve_base: reserve_base,
            reserve_inc: reserve_inc,
            server_status: server_status,
            validated_ledgers: validated_ledgers,
        })
    }
}

//////////////////////
/*
ServerInfoCommand 请求格式
id: u64
command: String
值分别为(固定值): 1, "server_info"
*/
#[derive(Serialize, Deserialize)]
pub struct ServerInfoCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,
}

impl ServerInfoCommand {
    pub fn with_params(id: u64, command: String) -> Box<Self> {
        Box::new( 
            ServerInfoCommand {
                id: id,
                command: command,
            }
        )
    }
}

impl CommandConversion for ServerInfoCommand {
    type T = ServerInfoCommand;
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
        // if let Ok(x) = value.downcast::<T>() {
        //     x
        // }

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

//实现default方法
impl Default for ServerInfoCommand {
    fn default() -> Self {
        ServerInfoCommand { 
            id: 1,
            command: "server_info".to_string(),
        }
    }
}
