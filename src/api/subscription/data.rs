use serde::{Deserialize, Serialize};
use serde_json::Result;

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
    pub fn with_params(id: u64, command: String, streams: Vec<String>) -> Box<Self> {
        Box::new( SubscribeCommand {
            id: id,
            command: command,
            streams: streams,
        } )
    }

    pub fn to_string(&self) -> Result<String> {
        let j = serde_json::to_string(&self)?;
        Ok(j)
    }
}

impl Default for SubscribeCommand {
    fn default() -> Self {
        SubscribeCommand {
            id: 0,
            command: "subscribe".to_string(),
            streams: vec!["ledger".to_string(),"server".to_string(),"transactions".to_string()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeResponse {
    #[serde(rename="fee_base")]
    pub fee_base: u64,

    #[serde(rename="fee_ref")]
    fee_ref: u64,

    #[serde(rename="hostid")]
    hostid: Option<String>,

    #[serde(rename="ledger_hash")]
    ledger_hash: String,

    #[serde(rename="ledger_index")]
    ledger_index: u64,

    #[serde(rename="ledger_time")]
    ledger_time: u64,

    #[serde(rename="load_base")]
    load_base: Option<u64>,

    #[serde(rename="load_factor")]
    load_factor: Option<u64>,

    #[serde(rename="pubkey_node")]
    pubkey_node: Option<String>,

    #[serde(rename="random")]
    random: Option<String>,

    #[serde(rename="reserve_base")]
    reserve_base: u64,

    #[serde(rename="reserve_inc")]
    reserve_inc: u64,

    #[serde(rename="server_status")]
    server_status: Option<String>,

    #[serde(rename="validated_ledgers")]
    validated_ledgers: String,

    #[serde(rename="txn_count")]
    txn_count: Option<u64>,

    #[serde(rename="type")]
    ttype: Option<String>,
}