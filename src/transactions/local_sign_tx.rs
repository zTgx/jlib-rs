
// use serde_json::json;
// use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;
// use std::rc::Rc;
use std::any::Any;

use crate::message::command_trait::CommandConversion;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LocalSignTx {
    #[serde(rename="id")]
    id: u64, 

    //如果需要本地签名为false， secret必须，否则可以为空。
    #[serde(rename="secret")]
    pub secret: Option<String>,

    #[serde(rename="command")]
    pub command: String, //Submit

    #[serde(rename="tx_blob")]
    pub tx_json: String,
}

impl LocalSignTx {
    pub fn new(secret: Option<String>, tx_json: String) -> Box<LocalSignTx> {
        Box::new( LocalSignTx {
            id: 1,
            command: "submit".to_string(),
            secret: secret,
            tx_json: tx_json,
        })
    }
}

impl CommandConversion for LocalSignTx {
    type T = LocalSignTx;
    fn to_string(&self) -> Result<String> {
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
}
