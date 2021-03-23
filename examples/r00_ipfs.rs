extern crate jlib;

use jlib::api::ipfs::hash::only_hash;
 
fn main() {
    /*
        hello world -> Qmf412jQZiuVUtdgnB36FXFX7xg5V6KEbSJ4dpQuhkLyfD
        1220F852C7FA62F971817F54D8A80DCD63FCF7098B3CBDE9AE8EC1EE449013EC5DB0
    */
    let target = "Qmf412jQZiuVUtdgnB36FXFX7xg5V6KEbSJ4dpQuhkLyfD".to_owned();
    let digest = "hello world".as_bytes().to_vec();
    if let Some(hash) = only_hash( &digest ) {
        assert_eq!(hash, target);
    } else {
        println!("ipfs hash none");
    }
} 
