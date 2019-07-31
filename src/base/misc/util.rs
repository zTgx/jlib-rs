
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;
use basex_rs::BaseX;
use crate::base::wallet::keypair::{Keypair, KeypairBuilder};
use crate::WalletType;

pub fn get_keypair_from_secret(secret: &String) -> Keypair {
    let wtype = fetch_wtype_from_secret(&secret);

    //keypair
    let key_pair = KeypairBuilder::new(&secret, &wtype).build();

    key_pair
}

pub fn fetch_wtype_from_secret(_secret: &String) -> WalletType {
    WalletType::SECP256K1
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
