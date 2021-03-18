
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer, SerializeStruct};

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

    pub fn to_string(&self) -> Result<String, serde_json::error::Error> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }
}