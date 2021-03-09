use serde::{Deserialize, Serialize};
use serde_json::Result;

//////////////////////
/*
@4.4请求底层服务器信息
ServerInfoCommand 请求格式
id: u64
command: String
值分别为(固定值): 1, "server_info"
*/
#[derive(Serialize, Deserialize, Debug)]
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

    pub fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        Ok(j)
    }
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

///////////////////////////////
/*
ServerInfoResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LastClose {
    #[serde(rename="converge_time_s")]
    pub converge_time_s: f64,

    #[serde(rename="proposers")]
    pub proposers: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ValidatedLedger {
    #[serde(rename="age")]
    pub age: u64,

    #[serde(rename="base_fee_swt")]
    pub base_fee_swt: f64,

    #[serde(rename="fee_account_swt")]
    pub fee_account_swt: String,

    #[serde(rename="hash")]
    pub hash: String,

    #[serde(rename="issuerop_account")]
    pub issuerop_account: String,

    #[serde(rename="manager_account")]
    pub manager_account: String,

    #[serde(rename="reserve_base_swt")]
    pub reserve_base_swt: u64,

    #[serde(rename="reserve_inc_swt")]
    pub reserve_inc_swt: u64,

    #[serde(rename="seq")]
    pub seq: u64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServerInfoResponse {
    #[serde(rename="build_version")]
    pub build_version: String,

    #[serde(rename="complete_ledgers")]
    pub complete_ledgers: String,

    #[serde(rename="hostid")]
    pub hostid: String,

    #[serde(rename="io_latency_ms")]
    pub io_latency_ms: u64,

    #[serde(rename="last_close")]
    pub last_close: LastClose,

    #[serde(rename="load_factor")]
    pub load_factor: u64,

    #[serde(rename="peers")]
    pub peers: u64,

    #[serde(rename="pubkey_node")]
    pub pubkey_node: String,

    #[serde(rename="server_state")]
    pub server_state: String,

    #[serde(rename="startup_time")]
    pub startup_time: String,

    #[serde(rename="validated_ledger")]
    pub validated_ledger: ValidatedLedger,

    #[serde(rename="validation_quorum")]
    pub validation_quorum: u64,
}
