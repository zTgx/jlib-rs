extern crate jlib;

use jlib::api::ipfs::hash::only_hash;

use std::fs::File;
 
fn main() {
    /*
    hello world -> Qmf412jQZiuVUtdgnB36FXFX7xg5V6KEbSJ4dpQuhkLyfD
    */
    let target = "Qmf412jQZiuVUtdgnB36FXFX7xg5V6KEbSJ4dpQuhkLyfD".to_owned();
    let digest = "hello world".as_bytes().to_vec();
    if let Some(hash) = only_hash( &digest ) {
        println!("ipfs hash: {}", hash);
        assert_eq!(hash, target);
    } else {
        println!("ipfs hash none");
    }
}
