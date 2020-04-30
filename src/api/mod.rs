pub mod query;
pub mod transaction;
pub mod subscribe;
pub mod contracts;

use crate::base::wallet::wallet::Wallet;
use crate::WalletType;

pub fn generate_wallet(wtype: WalletType) -> Wallet {
    Wallet::new(wtype)
}
