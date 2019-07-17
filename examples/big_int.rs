extern crate num;
use num::{BigInt};

// extern crate ryu;
// use ryu::*;

fn main() {
    let one = BigInt::from(1i32);
    println!("one : {:?}", one);


    // let mut buffer = ryu::Buffer::new();
    // let printed = buffer.format_finite(1.23e40);
    // // assert_eq!(printed, "1.234");
    // println!("ryu: {}", printed);


    use std::str::FromStr;
    let big = BigInt::parse_bytes(b"12340000000000000000000000000000000000000000", 10);
    // println!("big: {:?}", big.unwrap().to_string());
    let raw = BigInt::from_str(big.unwrap().to_string().as_str());
    println!("raw : {:?}", raw);



}