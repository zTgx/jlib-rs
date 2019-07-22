
use super::constants::{CURVE_ORDER, CURVE_ZERO};
use ring::{digest};
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;
use basex_rs::BaseX;

pub fn concat_args(left: &mut Vec<u8>, right: &Vec<u8>) {
    //append vs.extend
    left.extend(right);
}

pub fn encode_checked(x: &mut Vec<u8>) -> Vec<u8> {
    let vv: &[u8] = x.as_slice();

    let ac = digest::digest(&digest::SHA256, vv);
    let checked = digest::digest(&digest::SHA256, &ac.as_ref());
    let xx: Vec<u8> = checked.as_ref().iter().map(|c| {
        let x = format!("{:x}", c);
        x.as_str().chars().nth(0).unwrap() as u8
    }).collect::<Vec<u8>>();

    xx.get(..4).unwrap().to_vec()
}

pub fn encode_raw(x: &mut Vec<u8>) -> String {
    BaseX::encode(x.as_mut_slice())
}

//entropy的生成方式: 取值index范围，1 ~ 倒数第5
pub fn entropy(secret: &String) -> Vec<u8> {
    let buf = BaseX::decode(secret.to_string()).unwrap();
    buf[1..buf.len()-4].to_vec()
}

pub fn scalar_multiple(bytes: &[u8], discrim: Option<u8>) -> Vec<u8> {
    let mut i = 0u32;
    // while i <= 0xFFFFFFFF  {
    loop {
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

        let mut key = key.to_vec();
        key.truncate(32);

        if key.as_slice() < CURVE_ORDER && key.as_slice() > CURVE_ZERO {
            return key;
        }

        i += 1;
    } // end while
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
    let buf = decode_raw(encoded).unwrap();

    Some(buf[1..21].to_vec())
}

pub fn decode_raw(encoded: String) -> Option<Vec<u8>> {
    // decode(encoded)
    BaseX::decode(encoded)
}

pub fn downcast_to_string(value: Rc<dyn Any>) -> String {
    match value.downcast::<Cell<String>>() {
        Ok(string) => {
            return string.take();
        },
        Err(_) => { "None".to_string() }
    }
}

pub fn downcast_to_usize(value: Rc<dyn Any>) -> u32 {
    let mut us: u64 = 0;
    if let Ok(u) = value.downcast::<Cell<u64>>() {
        us = u.take();
    }

    us as u32
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
