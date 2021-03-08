extern crate jlib;
use jlib::base::wallet::wallet::{
    Wallet,
    WalletType
};
use jlib::base::wallet::builder::generate_wallet;

fn main() {
    let wallet: Wallet = generate_wallet(WalletType::SM2P256V1);
    println!("new wallet : {:#?}", wallet);
}
