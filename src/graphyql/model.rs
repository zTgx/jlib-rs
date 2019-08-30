#[derive(GraphQLObject)]
#[derive(Debug, Default)]
pub struct LastClose {
    pub converge_time_s: String,
    pub proposers: String,
}

#[derive(GraphQLObject)]
#[derive(Debug, Default)]
pub struct ValidatedLedger {
    pub age: String,
    pub base_fee_swt: f64,
    pub fee_account_swt: String,
    pub hash: String,
    pub issuerop_account: String,
    pub manager_account: String,
    pub reserve_base_swt: String,
    pub reserve_inc_swt: String,
    pub seq: String,
}

#[derive(GraphQLObject)]
#[derive(Debug, Default)]
pub struct ServerInfoResponse {
    pub build_version: String,
    pub complete_ledgers: String,
    pub hostid: String,
    pub io_latency_ms: String,
    pub last_close: LastClose,
    pub load_factor: String,
    pub peers: String,
    pub pubkey_node: String,
    pub server_state: String,
    pub startup_time: String,
    pub validated_ledger: ValidatedLedger,
    pub validation_quorum: String,
}

///账号类接口数据模型
#[derive(GraphQLEnum, Debug, Copy, Clone)]
pub enum WType {
    SECP256K1,
    ED25519,
}

#[derive(GraphQLObject, Debug, Clone, Default)]
pub struct Keypair {
    pub private_key: String, //hex string
    pub public_key: String,  //hex string
}

#[derive(GraphQLObject, Debug)]
pub struct Wallet {
    pub key_type: WType,
    pub address : String,
    pub secret  : String,
    pub keypair : Keypair,
}

#[derive(GraphQLObject, Debug)]
pub struct Balance {
    pub value: String,
    pub currency: String,
    pub issuer: String,
    pub freezed: String,
}

#[derive(GraphQLObject, Debug)]
pub struct Balances {
    pub balances: Vec<Balance>,
}

#[derive(GraphQLObject, Debug, Default)]
pub struct AmountG {
    pub value: String,   //0.5
    pub currency: String,//'USD',
    pub issuer: String,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',
}

#[derive(GraphQLObject, Debug, Default)]
pub struct PaymentInfo {
    pub hash: String,
    pub fee: String,
    pub date: String,
    pub memos: Vec<String>,
    pub counterparty: String,
    pub amount: AmountG,
}

#[derive(GraphQLObject, Debug, Default)]
pub struct LedgerInfo {
    pub ledger_hash: String,
    pub ledger_index: String,
}
