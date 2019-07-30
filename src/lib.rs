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

//Wallet
pub use crate::base::wallet::wallet::Wallet as Wallet;

////////////////////////////////////////////////////////////////////////////////////////////////////////
//0信任；1授权；3冻结/解冻；
//Default` cannot be derived for enums, only structs
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

//Generate Wallet
/*
Wallet DataStruct:
#[derive(Debug)]
pub struct Wallet {
    pub key_type: KeyType,
    pub address : WalletAddress,   //starts with 'j'
    pub secret  : Seed,            //secret seed
    pub keypair : Option<Keypair>, //public key & private key
}
*/

//key的加密算法：ed25519 / secp256k1
#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
}

pub fn generate_wallet(wtype: WalletType) -> Wallet {
    Wallet::new(wtype)
}
