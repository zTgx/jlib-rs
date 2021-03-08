use crate::base::wallet::wallet::WalletType;

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

impl Default for WalletConfig {
    fn default() -> Self {
        WalletConfig {
            key_type: WalletType::SECP256K1,
        }
    }
}
