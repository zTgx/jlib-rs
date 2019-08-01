
use serde::{Deserialize, Serialize};
use std::any::Any;
use serde::ser::{Serializer, SerializeStruct};
use crate::message::common::command_trait::CommandConversion;

#[derive(Deserialize, Debug, Default)]
pub struct LocalSignTx {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    pub command: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,
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
            state.serialize_field("tx_blob", &self.tx_blob)?;

            state.end()
        }
}


impl LocalSignTx {
    pub fn new(tx_blob: String) -> LocalSignTx {
        LocalSignTx {
            id: 1,
            command: "submit".to_string(),
            tx_blob: tx_blob,
        }
    }
}

impl CommandConversion for LocalSignTx {
    type T = LocalSignTx;
    fn to_string(&self) -> Result<String, serde_json::error::Error> {
        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        Ok(j)
    }

    fn box_to_raw(&self) -> &dyn Any {
        self
    }
}
