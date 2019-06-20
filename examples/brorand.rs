extern crate hex;
extern crate mylib;

use mylib::base::brorand::*;

fn main() {
    let ret = Brorand::brorand(16);
    println!("brorand : {:?}", ret);
}