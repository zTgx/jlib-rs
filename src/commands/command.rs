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

use crate::commands::command_trait::CommandConversion;

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
