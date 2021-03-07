
use crate::base::wallet::config::*;
use crate::base::wallet::keypair::*;
use crate::base::wallet::address::WalletAddress;
use crate::base::wallet::seed_builder::SeedBuilder;

use crate::WalletType;
use hex;

//WalletBuilder
#[derive(Debug)]
pub struct WalletBuilder {
    pub config: WalletConfig,
}

impl WalletBuilder {
    // impl <'a> WalletBuilder <'a> {
    pub fn new(config: WalletConfig) -> Self {
        // pub fn new(config: &'a WalletConfig) -> Self {
        WalletBuilder {
            config: config,
        }
    }

    pub fn build(&self) -> Wallet {
        //seed
        let passphrase = Some("Masterphrase");
        let seed_builder = SeedBuilder::new(self.config.key_type);
        
        let master_seed_hex = seed_builder.get_seed(passphrase);
        let master_seed     = seed_builder.human_seed(&master_seed_hex);

        // println!("master_seed: {:?}", master_seed);
        // println!("master_seed_hex: {:?}", hex::encode_upper(master_seed_hex));
        
        //keypair
        let key_pair = KeypairBuilder::new(&master_seed, &self.config.key_type).build().unwrap();

        //address
        let address = WalletAddress::build(&key_pair);

        Wallet {
            key_type: self.config.key_type,
            address : address,
            secret  : "secret".to_string(),
            keypair : key_pair,

            master_seed : master_seed,
            master_seed_hex : hex::encode_upper(master_seed_hex),
        }
    }
}

#[derive(Debug)]
pub struct Wallet {
    pub key_type: WalletType,
    pub address : String,
    pub secret  : String,
    pub keypair : Keypair,

    master_seed     : String,
    master_seed_hex : String,
}

impl Wallet {
    pub fn new(wtype: WalletType) -> Self {
        let config = WalletConfig::new(wtype);
        WalletBuilder::new(config).build()
    }
}
