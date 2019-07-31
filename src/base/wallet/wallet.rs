
use crate::base::wallet::config::*;
use crate::base::wallet::keypair::*;
use crate::base::wallet::address::WalletAddress;
use crate::base::wallet::seed::Seed;

use crate::WalletType;

//WalletBuilder
#[derive(Debug)]
pub struct WalletBuilder <'a> {
    pub config: &'a WalletConfig,
}

impl <'a> WalletBuilder <'a> {
    pub fn new(config: &'a WalletConfig) -> Self {
        WalletBuilder {
            config: config,
        }
    }

    pub fn build(&self) -> Wallet {
        //seed
        let seed = Seed::build(&self.config.key_type);

        //keypair
        let key_pair = KeypairBuilder::new(&seed).build();

        //address
        let address = WalletAddress::build(&key_pair);

        Wallet {
            key_type: self.config.key_type,
            address : address,
            secret  : seed,
            keypair : key_pair,
        }
    }
}

#[derive(Debug)]
pub struct Wallet {
    pub key_type: WalletType,
    pub address : String,
    pub secret  : String,
    pub keypair : Keypair,
}

impl Wallet {
    pub fn new(wtype: WalletType) -> Self {
        let config = WalletConfig::new(wtype);
        WalletBuilder::new(&config).build()
    }
}
