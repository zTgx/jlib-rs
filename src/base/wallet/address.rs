
use crate::base::ring::{digest};

use crate::base::curve::ripemd160::JRipemd160;
use crate::base::wallet::keypair::*;
use crate::base::wallet::generate_str;

use crate::base::xcodec::{is_valid_address};
use hex;

static H_ADDRESS: &[u8] = &[0];

#[derive(Debug)]
pub struct WalletAddress {}
impl WalletAddress {
    pub fn build(key_pair: &Keypair) -> String {
        WalletAddressBuilder::new(&key_pair).build()
    }

    //Codes-x
    pub fn check_address(address: &String) -> Option<bool> {
        is_valid_address(address)
    }
}

#[derive(Debug)]
pub struct WalletAddressBuilder <'a> {
    pub key_pair: &'a Keypair,
}
impl <'a> WalletAddressBuilder <'a> {
    pub fn new(key_pair: &'a Keypair) -> Self {
        WalletAddressBuilder {
            key_pair: key_pair,
        }
    }

    pub fn build(&self) -> String {
        self.generate()
    }

    pub fn generate(&self) -> String {
        let key: Vec<u8> = hex::decode(&self.key_pair.public_key).unwrap();
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(&key);

        let mut input = [0u8; 32];
        input.copy_from_slice(ctx.finish().as_ref());

        let mut ripemd160 = JRipemd160::new();
        ripemd160.input(&input);
        let ret: &mut [u8] = &mut [0u8;20];
        ripemd160.result(ret);

        let mut version: Vec<u8> = H_ADDRESS.to_vec();
        generate_str(&mut version, &ret.to_vec())
    }
}
