pub mod wallet;
pub mod config;
pub mod keypair;
pub mod address;
pub mod seed;

use basex_rs::{BaseX, SKYWELL, Encode};
use crate::base::curve::sha256::JSha256;
use libsm::sm3::hash::Sm3Hash; 

//generate seed && address
pub fn generate_str(version: &mut Vec<u8>, so: &Vec<u8>) -> String {
    //4. concat args
    version.extend(so);

    //5. sha256
    let checked: Vec<u8> = JSha256::sha256(&version);

    //6. take 0..4
    let token = checked.get(..4).unwrap().to_vec();

    //7. concat args
    version.extend(token);

    //end. base58 encode
    BaseX::new(SKYWELL).encode(version.as_mut_slice())
}

pub fn generate_guomi(version: &Vec<u8>, so: &Vec<u8>) -> String {
    let mut hash = Sm3Hash::new(&so);
    let sm3_hash: [u8;32] = hash.get_hash();
    println!("sm3_hash: {:?}", sm3_hash);
    let seed: &[u8] = &sm3_hash[..16];
    println!("seed: {:?}", seed);

    let mut x = Vec::new();
    x.extend(version);
    x.extend(seed);

    // mut -> imut
    let tt = &mut x;
    let t = &*tt;
    // let () = t;

    println!("17字节： {:?}", t);
    // let mut hash = Sm3Hash::new(t.as_slice());
    // let digest: [u8;32] = hash.get_hash();
    // println!("hash {:?}", digest);
  
    // //6. take 0..4
    // let checksum = digest.get(..4).unwrap().to_vec();
    // println!("前四个字节checksum {:?}", checksum);

//--
    let mut hash = Sm3Hash::new(t.as_slice());
    let hash1 = hash.get_hash();
    let mut hash2 = Sm3Hash::new(&hash1);
    let digest = hash2.get_hash();
    println!("digest: {:?}", digest);

    let checksum = digest.get(..4).unwrap().to_vec();
//--
  
    //7. concat args
    let mut src = Vec::new();
    src.extend(version); // 0x21
    src.extend(seed);    // seed
    src.extend(checksum);   // checksum

    println!("src: {:?}", src);

    //end. base58 encode
    BaseX::new(SKYWELL).encode(src.as_mut_slice())    
}

pub fn check_address(_address: &String) -> bool {

    true
}

pub fn check_secret(_secret: &String) -> bool {


    true
}
