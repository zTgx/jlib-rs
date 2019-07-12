extern crate hex;

extern crate mylib;
use mylib::base::util::{check, check_value, check_string};

fn main() {
    // let hex_string = hex::encode("Hello world!");
    // println!("{}", hex_string); // Prints '48656c6c6f20776f726c6421'

    // let x = format!("{:x}", 79);
    // // let tar = format!("{:?}", hex_string);
    // println!("tar : {}", x);

    // let bytes = "84e9ec3f".as_bytes();
    // println!("bytes : {:x?}", bytes);
    // println!("bytes : {:?}", bytes);

    // println!("");

    // let bytes = "1".as_bytes();
    // println!("bytes : {:x?}", bytes);
    // println!("bytes : {:?}", bytes);
    // println!("");

    // let bytes = "4f".as_bytes();
    // println!("bytes : {:x?}", bytes);
    // println!("bytes : {:?}", bytes);

    // let target = hex::encode(x);
    // println!("hex: {}", target);
    // println!("hex len : {}", target.len());

    // let x = check("79".to_string());
    // assert_eq!("30303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303466".to_string(), 
    //             x);

    // let xs = check("Hello, world!".to_string());
    // assert_eq!("48656c6c6f2c20776f726c642100000000000000000000000000000000000000".to_string(), xs);


    let mut x = format!("{:x}", 79);
    println!("x : {}", x);
    //0x0000000000000000000000000000000000000000000000000000000000000045
    //64 * 4 => 256bit => 256 / 8 => 32Bytes
    while x.len() < 64 {
        x.insert(0, '0');
    }
    println!("数字前补零后为： {}", x);

    let hex = hex::encode(x);
    println!("hex : {}", hex);

    println!("----------");
    //0x6461766500000000000000000000000000000000000000000000000000000000
    let y = check("dave".to_string());
    println!("y : {}", y);

    println!("-----------------------");
    let z = "84e9ec3f";
    println!("z len: {}", z.len());
    println!("hex : {}", hex::encode(z));
}
