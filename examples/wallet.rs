
extern crate jlib;
use jlib::WalletType;
use jlib::generate_wallet;

fn main() {
    let wallet = generate_wallet(WalletType::SECP256K1);
    println!("new wallet : {:#?}", wallet);
}
