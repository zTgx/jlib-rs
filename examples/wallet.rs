
extern crate jlib;
use jlib::base::wallet::*;
use jlib::base::config::*;

fn main() {
    let config = WalletConfig::new(KeyType::SECP256K1);
    let wallet = Wallet::new(&config);
    println!("new wallet : {:#?}", wallet);

    let config = WalletConfig::default();
    let wallet = Wallet::new(&config);
    println!("default wallet : {:#?}", wallet);

}