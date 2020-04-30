#[macro_use]
extern crate lazy_static;

extern crate typename;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate num;
extern crate void;
extern crate ws;
extern crate basex_rs;
extern crate cast_rs;

pub mod base;
pub mod message;
pub mod misc;
pub mod api;
pub mod contracts;

pub use crate::base::wallet::wallet::Wallet as Wallet;

#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
}

pub fn generate_wallet(wtype: WalletType) -> Wallet {
    Wallet::new(wtype)
}
