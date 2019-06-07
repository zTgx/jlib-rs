use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

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

    //TODO: 可抽离成trait
    pub fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        println!("{}", j);

        Ok(j)
    }
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

// pub trait paser {
//     type 
//     fn to_string() -> Option<String>;
// }

//////////////////////
/*
SubscribeCommand 请求格式
id: u64
command: String
streams: Vec<String>
值分别为(固定值): 0, "subscribe", ["ledger","server","transactions"]
*/