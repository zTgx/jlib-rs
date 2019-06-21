extern crate mylib;
use mylib::base::base_data::*;

fn main() {
    let s = BASEStates::Seed(Seed::default());
    println!("seed : {:?}", s);
}