
extern crate mylib;
use mylib::base::wallet::*;
use mylib::base::config::*;

fn main() {
    let config = WalletConfig::new(KeyType::SECP256K1);
    let wallet = Wallet::new(&config);
    println!("new wallet : {:#?}", wallet);

    let config = WalletConfig::default();
    let wallet = Wallet::new(&config);
    println!("default wallet : {:#?}", wallet);

}