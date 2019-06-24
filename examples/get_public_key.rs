extern crate mylib;

use mylib::base::*;

fn main() {
    let public_key = util::get_public_key_from_secret("snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string());
    println!("public key : {}", public_key);

    /*
    Keypair { property: KeypairProperty { 
        secret_key: "001ACAAEDECE405B2A958212629E16F2EB46B153EEE94CDD350FDEFF52795525B7", 
        public_key: "0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020" } }
    */
}