use hex;
use crate::address::traits::address::AddressI;
use crate::base::seed::builder::SeedBuilder;
use crate::address::builder::AddressBuilder;
use crate::wallet::wallet::{
    Wallet,
    WalletType
};

#[derive(Debug)]
pub struct WalletBuilder {
    pub wallet_type: WalletType,
}

impl WalletBuilder {
    pub fn new(wallet_type: WalletType) -> Wallet {
        WalletBuilder::build(wallet_type)
    }

    pub fn build(wallet_type: WalletType) -> Wallet {
        let passphrase = None;
        // let passphrase = Some("Masterphrase");

        let seed_builder = SeedBuilder::new(wallet_type);
        
        let master_seed_hex = seed_builder.get_seed(passphrase);
        let master_seed     = seed_builder.human_seed(&master_seed_hex);
        let human_seed_rfc1751 = seed_builder.human_seed_rfc1751(&master_seed_hex);

        let address = AddressBuilder::new(wallet_type, &master_seed_hex);
        let account_id = address.human_account_id();

        let public_key = address.public_key();
        let public_key_hex = address.public_key_hex();

        Wallet {
            key_type        : wallet_type,
            account_id      : account_id,
            master_key      : human_seed_rfc1751,
            master_seed     : master_seed,
            master_seed_hex : hex::encode_upper(master_seed_hex),
            public_key      : public_key,
            public_key_hex  : public_key_hex,
        }
    }
}

/*
    需要WalletType

    1、该方法是创建钱包的便利方法。
    2、也可以使用WalletBuilder。
*/
pub fn generate_wallet(wallet_type: WalletType) -> Wallet {
    WalletBuilder::new(wallet_type)
}
