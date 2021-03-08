
use crate::base::wallet::config::*;
// use crate::base::wallet::keypair::*;
// use crate::base::wallet::address::WalletAddress;

use hex;
use crate::WalletType;
use crate::base::address::guomi::Address;
use crate::base::address::traits::address::AddressI;
use crate::base::seed::builder::SeedBuilder;

//WalletBuilder
#[derive(Debug)]
pub struct WalletBuilder {
    pub config: WalletConfig,
}

impl WalletBuilder {
    pub fn new(config: WalletConfig) -> Self {
        WalletBuilder {
            config: config,
        }
    }

    pub fn build(&self) -> Wallet {
        let passphrase = None;
        // let passphrase = Some("Masterphrase");
        let seed_builder = SeedBuilder::new(self.config.key_type);
        
        let master_seed_hex = seed_builder.get_seed(passphrase);
        let master_seed     = seed_builder.human_seed(&master_seed_hex);
        let human_seed_rfc1751 = seed_builder.human_seed_rfc1751(&master_seed_hex);

        let address = Address::new(&master_seed_hex.to_vec());
        let account_id = address.human_account_id();

        let public_key = address.public_key();
        let public_key_hex = address.public_key_hex();

        //keypair
        // let key_pair = KeypairBuilder::new(&master_seed, &self.config.key_type).build().unwrap();

        //address
        // let address = WalletAddress::build(&key_pair);

        Wallet {
            key_type        : self.config.key_type,
            account_id      : account_id,
            master_key      : human_seed_rfc1751,
            master_seed     : master_seed,
            master_seed_hex : hex::encode_upper(master_seed_hex),
            public_key      : public_key,
            public_key_hex  : public_key_hex,
        }
    }
}

#[derive(Debug)]
pub struct Wallet {
    pub key_type    : WalletType,
    account_id      : String,
    master_key      : String,
    master_seed     : String,
    master_seed_hex : String,

    public_key      : String,
    public_key_hex  : String,
}

impl Wallet {
    pub fn new(wtype: WalletType) -> Self {
        let config = WalletConfig::new(wtype);
        WalletBuilder::new(config).build()
    }
}
