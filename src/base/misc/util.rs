
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;
use basex_rs::BaseX;
use crate::base::wallet::keypair::{Keypair, KeypairBuilder};
use crate::WalletType;
use crate::message::common::amount::Amount;
use crate::misc::base_config::*;
use crate::base::wallet::address::{WalletAddress};
use crate::base::wallet::seed::Seed;

pub fn get_keypair_from_secret(secret: &String) -> Result<Keypair, &'static str> {
    let wtype = fetch_wtype_from_secret(&secret);

    //keypair
     KeypairBuilder::new(&secret, &wtype).build()
}

pub fn fetch_wtype_from_secret(_secret: &String) -> WalletType {
    WalletType::SECP256K1
}

/////////////////////////////////////////////////////////////////////////
pub fn check_secret(secret: &String) -> Option<bool> {
    Seed::check_secret(secret)
}

pub fn check_address(address: &String) -> Option<bool> {
    WalletAddress::check_address(address)
}

pub fn check_currency(cur: &Option<String>) -> bool {
    let mut f = true;
    if let Some(x) = cur {
        if x.len() != 3 {
            f = false;
        }

        for c in x.chars() {
            if c >= 'A' && c <= 'Z' {
            } else {
                f = false;
                break;
            }
        }
    }

    f
}
//is valid amount
pub fn check_amount(amount: &Amount) -> bool {
    // check amount value
    let value = &amount.value;
    let len = value.len();
    if len == 1 && value == "." {
        return false;
    }

    for c in value.chars() {
        if c >= '0' && c <= '9' || c == '.' {
        } else {
            return false;
        }
    }

    // check amount currency
    if check_currency(&amount.currency) == false {
        return false;
    }

    // native currency issuer is empty
    if amount.currency == Some(CURRENCY.to_string()) && amount.issuer.is_some() {
        return false;
    }

    // non native currency issuer is not allowed to be empty
    let mut is_issuer = None;
    if let Some(ref x) = amount.issuer {
        is_issuer = check_address(&x);
    }

    if amount.currency != Some(CURRENCY.to_string()) && is_issuer.is_none() {
        return false;
    }

    true
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
