extern crate mylib;
use mylib::base::keypair::*;
use mylib::base::config::*;
use mylib::base::seed::*;

 

fn main() {
    //seed
    let seed: &'static str = "shXS3bzZLPHczZPnFCDYQKQ8NtMDM";
    let seed_property = SeedProperty::new(seed, 16);
    let seed = SeedBuilder::new(seed_property).build();

    let x = KeypairBuilder::new(seed).build();
    println!("key pair : {:?}", x);
}