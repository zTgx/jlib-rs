extern crate bs58;

use super::constants::*;
use ring::{digest, test};

extern crate secp256k1;
use secp256k1::key::{ SecretKey};
use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::key::ONE_KEY;

use crate::base::constants::ALPHABET;
use std::collections::HashMap;

pub fn concat_args(left: &mut Vec<u8>, right: &Vec<u8>) {
    // println!("before concat args: {:?}", left);

    //append vs.extend
    left.extend(right); 
    
    // println!("after concat args : {:?}", left);
}

pub fn encode_checked(x: &mut Vec<u8>) -> Vec<u8> {
    //let vv: &[u8] = &[ 33, 228, 98, 120, 229, 208, 105, 36, 76, 162, 155, 0, 178, 95, 45, 115, 89 ];
    let vv: &[u8] = x.as_slice();
    
    let ac = digest::digest(&digest::SHA256, vv);
    let checked = digest::digest(&digest::SHA256, &ac.as_ref());
    let xx: Vec<u8> = checked.as_ref().iter().map(|c| {
        let x = format!("{:x}", c);
        x.as_str().chars().nth(0).unwrap() as u8
    }).collect::<Vec<u8>>();
    // println!("checked : {:?}", xx.get(..4));

    xx.get(..4).unwrap().to_vec()
}

pub fn encode_raw(x: &mut Vec<u8>) -> String {
    encode(x.as_mut_slice())
}

pub fn encode(source: &[u8]) -> String {
        
    let BASE = ALPHABET.len() as u16;

    let mut digits: Vec<u16> = vec![0u16; 1];
    
    let mut i = 0;
    while i < source.len() {

        let mut j = 0;
        let mut carry: u16 = source[i] as u16;
        
        let digits_len = digits.len();
        while j < digits_len {
            carry += digits.as_slice()[j] << 8;

            digits.as_mut_slice()[j] = carry % (BASE as u16);

            carry = (carry / (BASE as u16)) | 0;

            j += 1;
        }

        while carry > 0 {
            digits.push(carry % (BASE as u16));
            carry = (carry / BASE) | 0;
        }

        i += 1;
    }

    let mut string = "".to_string();

    //  for (var k = 0; source[k] === 0 && k < source.length - 1; ++k) string += ALPHABET[0]
    // deal with leading zeros
    let mut k = 0;
    while source[k] == 0 && k < source.len() - 1 {

        string.push(ALPHABET[0] as char);

        k += 1;
    }        

    // convert digits to a string
    let mut q: i32 = (digits.len() - 1) as i32;
    while q >= 0 {

        let uu: u8 = ALPHABET[digits[q as usize] as usize];
        let xx = uu as char;

        string.push( xx );

        q -= 1;
    }

    String::from(string.as_str())
}

//entropy的生成方式: 取值index范围，1 ~ 倒数第5
pub fn entropy(secret: &String) -> Vec<u8> {
    // let prefix = "00";
    //ssndDcNoc4FvwVPveY3KbfWh8fNh3
    let buf = bs58::decode(secret).with_alphabet(ALPHABET).into_vec().unwrap();
    
    buf[1..buf.len()-4].to_vec()
}


pub fn scalar_multiple(bytes: &[u8], discrim: Option<u8>) -> Vec<u8> {
    let mut i = 0u32;
    while i <= 0xFFFFFFFF  {
        // We hash the bytes to find a 256 bit number, looping until we are sure it
        // is less than the order of the curve.
        let mut ctx = digest::Context::new(&digest::SHA512);
        ctx.update(&bytes);
        if let Some(x) = discrim {
            //as i32
            ctx.update(&(x as i32).to_be_bytes());
        }
        ctx.update(&i.to_be_bytes());

        let mut key = [0u8; 64];
        key.copy_from_slice(ctx.finish().as_ref());
        // for x in key.iter() {
        //     println!("{}", x );
        // }
        let mut key = key.to_vec();
        key.truncate(32);
        
        // let finish = ctx.finish();
        // let xx: String = finish.as_ref().iter().map(|c| {
        //     let x = format!("{:x}", c);
        //     x 
        // }).collect();
        // let key = xx.get(0..32).unwrap().to_string();

        if key.as_slice() < CURVE_ORDER && key.as_slice() > CURVE_ZERO {


            // println!("scalar key : {:?}", key);
            // let mut key = key.to_vec();
            // key.truncate(32);
            return key;
        }

        i += 1;
    } // end while

    //never get this
    vec![0]
}

//通过secret计算出publickey
use crate::base::keypair::*;
pub fn get_public_key_from_secret(secret: &String) -> Keypair {

    use crate::base::seed::*;
    use crate::base::keypair::*;

    //seed
    let seed = secret;
    let seed_property = SeedProperty::new(&seed, 16);
    let seed = SeedBuilder::new(seed_property).build();

    //keypair
    let key_pair = KeypairBuilder::new(&seed).build();
    println!("key pair : {:?}", key_pair);  

    key_pair
}

/////////////////////////////////////////////////////////////////////////////
//decode
//decode j开头的hex string地址到Vec<u8>
pub fn decode_j_address(address: String) -> Option<Vec<u8>> {
    decode_address(address)
}

pub fn decode_address(address: String) -> Option<Vec<u8>> {
    if true { //is_set
        return decode_versioned(address);
    }

    None
}

pub fn decode_versioned(address: String) -> Option<Vec<u8>> {
    decode_multi_versioned(address)
}

pub fn decode_multi_versioned(address: String) -> Option<Vec<u8>> {
    let x = decode_checked(address);

    //calc payload
    x
}

pub fn decode_checked(encoded: String) -> Option<Vec<u8>> {
        println!("decode_versioned.");

    let buf = decode_raw(encoded).unwrap();

    println!("decode_versioned.buf : {:?}", buf);

    Some(buf[1..21].to_vec())
}

pub fn decode_raw(encoded: String) -> Option<Vec<u8>> {
    decode(encoded)
}

pub fn decode(string: String) -> Option<Vec<u8>> {
    if string.len() == 0 { return None; }

    let ALPHABET_MAP = generate_alpha_map();
    let BASE = ALPHABET.len() as u16;
    let LEADER = ALPHABET[0] as char;

    let mut bytes: Vec<u8> = vec![];
    let mut i = 0;
    while i < string.len() {
        let c = string.as_str().chars().nth(i).unwrap();
        let val = ALPHABET_MAP.get(&c);
        if val.is_none() {
            return None;
        }
        
        let mut j = 0;
        let mut carry: u16 = *val.unwrap() as u16;
        while j < bytes.len() {
            carry += bytes[j] as u16 * BASE;
            bytes[j] = (carry as u8) & 0xff;
            carry >>= 8;

            j += 1;
        }

        while carry > 0 {
            bytes.push((carry as u8) & 0xff );
            carry >>= 8;
        } 

        i += 1;
    }

    // deal with leading zeros
    let mut k = 0;
    while string.as_str().chars().nth(k).unwrap() == LEADER && k < string.len() - 1 {
      bytes.push(0);

      k += 1;
    }

    bytes.as_mut_slice().reverse();

    Some(bytes)
}

pub fn calc(payload: Option<Vec<u8>>) -> Option<Vec<u8>> {
    None
}

//default source ALPHABET. 
pub fn generate_alpha_map() -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    let lens = ALPHABET.len();
    let leader = ALPHABET[0];

    // pre-compute lookup table
    let mut i = 0; 
    while i < lens {
        let x = ALPHABET[i] as char;
        map.insert(x, i);
        
        i += 1;
    }

    map
}