
extern crate mylib;
use mylib::base::brorand::*;
use mylib::base::*;
use mylib::base::constants::{ALPHABET, PASSWORD_LEN};
use mylib::base::wallet::*;

fn main() {
    // let ret = util::entropy("ssndDcNoc4FvwVPveY3KbfWh8fNh3".to_string());
    // println!("entropy : {:?}", ret);

    let config = WalletConfig::new("secp256k1".to_string());
    let wallet = Wallet::new(config);
    println!("wallet : {:?}", wallet);
}