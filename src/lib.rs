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
pub use crate::api::subscribe::subs::{Subscribe, SubscribeI};

#[derive(Debug)]
pub enum RelationType {
    TRUST     = 0,
    AUTHORIZE = 1,
    FREEZE    = 3,
}
impl RelationType {
    pub fn get(&self) -> u32 {
        match *self {
            RelationType::TRUST     => { 0 },
            RelationType::AUTHORIZE => { 1 },
            RelationType::FREEZE    => { 3 },
        }
    }
}

//Offer Type
#[derive(PartialEq)]
pub enum OfferType {
    Sell,
    Buy,
}
impl OfferType {
    pub fn get(&self) -> &'static str {
        match *self {
            OfferType::Sell => { "Sell" },
            OfferType::Buy  => { "Buy"  },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
}

pub fn generate_wallet(wtype: WalletType) -> Wallet {
    Wallet::new(wtype)
}
