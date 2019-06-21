
extern crate mylib;
use mylib::base::wallet::*;

fn main() {
    let config = WalletConfig::new(KeyType::SECP256K1);
    let wallet = Wallet::new(config);
    println!("wallet : {:?}", wallet);
}