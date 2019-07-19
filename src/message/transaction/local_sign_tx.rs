
// use serde_json::json;
// use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::any::Any;
use serde::ser::{Serializer, SerializeStruct};
use crate::message::common::command_trait::CommandConversion;

#[derive(Deserialize, Debug, Default)]
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

impl Serialize for LocalSignTx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
        S: Serializer,
        {
            // 3 is the number of fields in the struct.
            let mut state = serializer.serialize_struct("OfferCreateTxJson", 3)?;

            state.serialize_field("id", &self.id)?;
            state.serialize_field("command", &self.command)?;
            state.serialize_field("tx_json", &self.tx_json)?;

            state.end()
        }
}


impl LocalSignTx {
    pub fn new(secret: Option<String>, tx_json: String) -> Box<LocalSignTx> {
        Box::new( LocalSignTx {
id: 2,
command: "submit".to_string(),
secret: secret,
tx_json: tx_json,
})
}
}

impl CommandConversion for LocalSignTx {
    type T = LocalSignTx;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
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
