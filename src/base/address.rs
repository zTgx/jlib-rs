

use ring::{digest};

extern crate secp256k1;
use secp256k1::key::{ SecretKey};
use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::key::ONE_KEY;
use secp256k1::constants::*;    

extern crate crypto;
use crypto::ripemd160::Ripemd160 ;
use crypto::digest::Digest;

use crate::base::*;
use crate::base::keypair::*;

#[derive(Debug)]
pub struct WalletAddressProperty {
    pub address: String,
}
impl WalletAddressProperty {
    pub fn new(address: String) -> Self {
        WalletAddressProperty {
            address: address,
        }
    }
}

#[derive(Debug)]
pub struct WalletAddress {
    pub property: WalletAddressProperty,
}
impl WalletAddress {
    pub fn new(property: WalletAddressProperty) -> Self {
        WalletAddress {
            property: property,
        }
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

    pub fn build(&self) -> WalletAddress {
        let property = self.generate();

        WalletAddress {
            property: property,
        }
    }

    pub fn generate(&self) -> WalletAddressProperty {
        let mut key: Vec<u8> = vec![0];
        if let Ok(x) = hex::decode(&self.key_pair.property.public_key) {
            // println!("address key : {:?}", key);
            key = x;
        }
        let mut ctx = digest::Context::new(&digest::SHA256);
        ctx.update(&key);
        let mut key = [0u8; 32];
        key.copy_from_slice(ctx.finish().as_ref());

        let input = key;

        let mut ripemd160x = Ripemd160::new();
        ripemd160x.input(&input);

        let mut ret: &mut [u8] = &mut [0u8;20];
        ripemd160x.result(ret);
        
        let ripemd160x= ripemd160x.result_str();

        let mut xy = "".to_string();
        if let Ok(args) = hex::decode(ripemd160x) {

            let mut version: Vec<u8> = [0].to_vec();

            //4. concat args
            util::concat_args(&mut version, &ret.to_vec());

            //5. encodechecked.
            let mut checked: Vec<u8> = util::encode_checked(&mut version);

            //6. concat args
            util::concat_args(&mut version, &checked);

            let address = util::encode_raw(&mut version);

            xy = address;
        }

        WalletAddressProperty {
            address: xy,
        }
    }

    // pub fn get(&self) -> String {
    //     self.address.to_owned()
    // }
}