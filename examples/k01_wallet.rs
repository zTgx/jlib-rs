extern crate jlib;

use jlib::wallet::wallet::{
    WalletType
};
use jlib::wallet::builder::generate_wallet;

fn main() {
    let wallet = generate_wallet(WalletType::SECP256K1);
    println!("new wallet : {:#?}", wallet);
}
