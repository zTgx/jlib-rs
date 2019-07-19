extern crate bs58;

use super::constants::*;
use ring::{digest};

extern crate secp256k1;
// use secp256k1::key::{ SecretKey};
// use secp256k1::key::PublicKey;
// use secp256k1::Secp256k1;
// use secp256k1::key::ONE_KEY;

use crate::base::constants::ALPHABET;
use std::collections::HashMap;

use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;
// use std::cell::RefCell;

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
        
    let base = ALPHABET.len() as u16;

    let mut digits: Vec<u16> = vec![0u16; 1];
    
    let mut i = 0;
    while i < source.len() {

        let mut j = 0;
        let mut carry: u16 = source[i] as u16;
        
        let digits_len = digits.len();
        while j < digits_len {
            carry += digits.as_slice()[j] << 8;

            digits.as_mut_slice()[j] = carry % (base as u16);

            carry = (carry / (base as u16)) | 0;

            j += 1;
        }

        while carry > 0 {
            digits.push(carry % (base as u16));
            carry = (carry / base) | 0;
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
    let mut i = 0u64;
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

    let alphabet_map = generate_alpha_map();
    let base = ALPHABET.len() as u16;
    let ledger = ALPHABET[0] as char;

    let mut bytes: Vec<u8> = vec![];
    let mut i = 0;
    while i < string.len() {
        let c = string.as_str().chars().nth(i).unwrap();
        let val = alphabet_map.get(&c);
        if val.is_none() {
            return None;
        }
        
        let mut j = 0;
        let mut carry: u16 = *val.unwrap() as u16;
        while j < bytes.len() {
            carry += bytes[j] as u16 * base;
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
    while string.as_str().chars().nth(k).unwrap() == ledger && k < string.len() - 1 {
      bytes.push(0);

      k += 1;
    }

    bytes.as_mut_slice().reverse();

    Some(bytes)
}

//default source ALPHABET. 
pub fn generate_alpha_map() -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    let lens = ALPHABET.len();
    // let leader = ALPHABET[0];

    // pre-compute lookup table
    let mut i = 0; 
    while i < lens {
        let x = ALPHABET[i] as char;
        map.insert(x, i);
        
        i += 1;
    }

    map
}

pub fn downcast_to_string(value: Rc<dyn Any>) -> String {
    match value.downcast::<Cell<String>>() {
        Ok(string) => {
            return string.take();
        },
        Err(_) => { "None".to_string() }
    }
}

pub fn downcast_to_usize(value: Rc<dyn Any>) -> usize {
    let mut us: usize = 0;
    if let Ok(u) = value.downcast::<Cell<usize>>() {
        us = u.take();
    } 

    us
}

/*
工具方法
*/
pub fn string_to_hex(s: &String) -> String {
    let mut ss = String::from("");
    for x in s.as_bytes() {
        let hexs = format!("{:x}", x);
        ss.push_str(&hexs);
    }
    
    ss
}

//https://solidity.readthedocs.io/en/v0.5.4/abi-spec.html
//solidity abi spec
//处理格式说明：
/*
输入参数：79， “dave” 等格式的原始数据。
处理过程:
1，所有的输入的数据，转换成十六进制字符串格式。如：79 =》 "4f"
2，补零。（数字前0到32字节长度，字符串则后补零到32字节）。如："4f" => "000000000000000000000000000000000000000000000000000000000000004f"
3，最后，整体转换一次十六进制, 为最终结果。如：
"000000000000000000000000000000000000000000000000000000000000004f" =>
"30303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303466"
*/
pub fn check_value(p: i32) -> String {
    let mut hex_string = format!("{:x}", p);
    while hex_string.len() < 64 {
        hex_string.insert(0, '0');
    }

    return hex::encode(hex_string);
}

pub fn check_string(p: String) -> String {
    let mut hex_string = hex::encode(p);
    while hex_string.len() < 64 {
        hex_string.push('0');
    }
    
    return hex::encode(hex_string); 
}

pub fn check(p: String) -> String {
    let hex_string: String = p;
    if let Ok(x) = hex_string.parse::<i32>() {
        return check_value(x);
    } 

    return check_string(hex_string);
}

