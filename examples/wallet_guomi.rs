extern crate jlib;
use jlib::base::wallet::wallet::{WalletType, generate_wallet};

fn main() {
    let wallet = generate_wallet(WalletType::SM2P256V1);
    println!("new wallet : {:#?}", wallet);
}
