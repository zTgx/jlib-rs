pub struct ServerInfo {
    //pub complete_ledgers: String,//???
    pub ledger: String, //ledger hash
    pub public_key: String, //public key
    pub state: String, //server state
    pub peers: Vec<i32>, //peers vec
    pub version: String, //version
}
impl ServerInfo {

}