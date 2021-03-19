use crate::wallet::keypair::*;

use crate::address::impls::seed::sm2p256v1::SeedSM2P256V1;
use crate::address::traits::seed::{SeedI, SeedCheckI};

use crate::wallet::wallet::WalletType;
use basex_rs::{BaseX, SKYWELL, Decode};

pub struct SeedBuilder {
    seed: Box<dyn SeedI>
}
impl SeedBuilder {
    pub fn new(seed_type: WalletType) -> Self {
        let seed = SeedBuilder::build(seed_type);

        SeedBuilder {
            seed : seed
        }
    }

    /*
        需要 secret

        1、secret 的 base58 解码。
        2、截取 1..17 区间的16个字节
    */
    pub fn secret_to_seed(secret: &String) -> Vec<u8> {
        let seed = BaseX::new(SKYWELL).decode((&secret).to_string()).unwrap();
        
        seed[1..17].to_vec()
    }
    
    fn build(seed_type: WalletType) -> Box<dyn SeedI> {
        match seed_type {
            WalletType::ED25519 => {
                return Box::new(SeedSM2P256V1::new());
            },
            WalletType::SECP256K1 => {
                return Box::new(SeedSM2P256V1::new());
            },
            WalletType::SM2P256V1 => {
                return Box::new(SeedSM2P256V1::new());
            }
        }
    }
}

// ----------------------------------------------------------------------------------------------------------
// SeedBuilder 对 trait SeedI的实现。
// ----------------------------------------------------------------------------------------------------------
impl SeedI for SeedBuilder {
    fn get_seed(&self, passphrase: Option<&str>) -> Vec<u8> {
        return self.seed.get_seed(passphrase);
    }

    fn human_seed(&self, seed: &Vec<u8>) -> String {
        return self.seed.human_seed(&seed);
    }

    fn human_seed_rfc1751(&self, seed: &Vec<u8>) -> String {
        return self.seed.human_seed_rfc1751(&seed);
    }
}

// ----------------------------------------------------------------------------------------------------------
// SeedBuilder 对 trait SeedCheckI的实现。
// ----------------------------------------------------------------------------------------------------------
impl SeedCheckI for SeedBuilder {
    fn check(seed: &String) -> bool {
        let key_pair = KeypairBuilder::new(&seed, &WalletType::SECP256K1).build();
        if key_pair.is_ok() {
            return true;
        }

        let key_pair = KeypairBuilder::new(&seed, &WalletType::ED25519).build();
        if key_pair.is_ok() {
            return true;
        }

        false
    }
}
