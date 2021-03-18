extern crate jlib;

use jlib::seed::traits::seed::SeedI;
use jlib::seed::builder::SeedBuilder;

use jlib::wallet::wallet::{
    WalletType
};

fn main() {    
    let passphrase = Some("Masterphrase");

    let seed_builder = SeedBuilder::new(WalletType::SM2P256V1);
    let master_seed_hex = seed_builder.get_seed(passphrase);

    let master_seed     = seed_builder.human_seed(&master_seed_hex);
    
    println!("readable seed : {}", master_seed);
}
