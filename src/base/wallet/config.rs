/*
base config
*/
use crate::WalletType;

//钱包属性：地址长度，加密算法等
#[derive(Debug, Copy, Clone)]
pub struct WalletConfig {
    pub key_type: WalletType,
}
impl WalletConfig {
    pub fn new(key_type: WalletType) -> Self {
        WalletConfig {
            key_type: key_type,
        }
    }
}
//实现default方法
impl Default for WalletConfig {
    fn default() -> Self {
        WalletConfig {
            key_type: WalletType::SECP256K1,
        }
    }
}
