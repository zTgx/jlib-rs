/*
base config
*/

#[derive(Debug, Copy, Clone)]
//key的加密算法：ed25519 / secp256k1
pub enum KeyType {
    SECP256K1,
    ED25519,
}

//钱包属性：地址长度，加密算法等
#[derive(Debug, Copy, Clone)]
pub struct WalletConfig {
    pub key_type: KeyType,
}
impl WalletConfig {
    pub fn new(key_type: KeyType) -> Self {
        WalletConfig {
            key_type: key_type,
        }
    }
}
//实现default方法
impl Default for WalletConfig {
    fn default() -> Self {
        WalletConfig { 
            key_type: KeyType::SECP256K1,
        }
    }
}
