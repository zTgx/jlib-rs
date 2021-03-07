use crate::base::misc::brorand::Brorand;
use crate::base::data::constants::PASSWORD_LEN;
use crate::WalletType;
use crate::base::wallet::{
    generate_str
};
use crate::base::wallet::keypair::*;
use crate::base::seed::guomi::SeedGuomi;
use crate::base::seed::traits::seed::SeedI;


// 33 = 0x21
static H_SECP256K1: &[u8] = &[33];
static H_ED25519:   &[u8] = &[33];

pub struct SeedBuilder {
    seed_type : WalletType,
}
impl SeedBuilder {
    pub fn new(seed_type: WalletType) -> Self {
        SeedBuilder {
            seed_type : seed_type
        }
    }

    pub fn get_seed(&self, passphrase: Option<&str>) -> Vec<u8> {
        match &self.seed_type {
            &WalletType::ED25519 => {
                let mut version = H_ED25519.to_vec();
                let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);
                return generate_str(&mut version, &u).as_bytes().to_vec();
            },

            &WalletType::SECP256K1 => {
                let mut version = H_SECP256K1.to_vec();
                let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);
                return generate_str(&mut version, &u).as_bytes().to_vec();
            },
            &WalletType::SM2P256V1 => {
                let seed_guomi = SeedGuomi::new();
                let seed = seed_guomi.get_seed(passphrase);

                return seed;
            }
        }
    }

    pub fn human_seed(&self, seed: &Vec<u8>) -> String {
        match &self.seed_type {
            &WalletType::ED25519 => {
                let mut version = H_ED25519.to_vec();
                let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);
                return generate_str(&mut version, &u);
            },

            &WalletType::SECP256K1 => {
                let mut version = H_SECP256K1.to_vec();
                let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);
                return generate_str(&mut version, &u);
            },
            &WalletType::SM2P256V1 => {
                let seed_guomi = SeedGuomi::new();
                let seed = seed_guomi.human_seed(&seed);

                return seed;
            }
        }
    }

    pub fn human_seed_rfc1751(&self, seed: &Vec<u8>) -> String {
        let seed_guomi = SeedGuomi::new();
        let human_seed_rfc1751 = seed_guomi.human_seed_rfc1751(&seed);

        return human_seed_rfc1751;
    }
}

impl SeedBuilder {
    pub fn check_secret(seed: &String) -> Option<bool> {
        let key_pair = KeypairBuilder::new(&seed, &WalletType::SECP256K1).build();
        if key_pair.is_ok() {
            return Some(true);
        }

        let key_pair = KeypairBuilder::new(&seed, &WalletType::ED25519).build();
        if key_pair.is_ok() {
            return Some(true);
        }

        None
    }
}
