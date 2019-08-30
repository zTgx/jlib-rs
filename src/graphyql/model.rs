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

///Wallet
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
