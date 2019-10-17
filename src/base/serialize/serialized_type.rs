use crate::base::misc::amount::*;
use crate::base::misc::*; //util
use typename::TypeName;

extern crate num;
use num::bigint::{BigInt, ToBigInt, Sign};
use std::ops::{BitAndAssign, BitOrAssign, BitOr, BitAnd, Shr, Mul};
use num::{Zero};

use crate::base::misc::util::{decode_j_address};

pub trait SerializedType {
  fn serialize(&self) -> Vec<u8>;
  fn parse(&self);
}

pub trait SerializedSTInt8 {
  fn serialize(value: u8) -> Vec<u8>;
  fn parse();
}

//STInt8
#[derive(TypeName, Debug)]
pub struct STInt8 {
  pub id: i32,
  pub value: u8,
}
impl STInt8 {
  pub fn new(value: u8) -> Self {
    STInt8 {
      id: 16, //default value
      value: value,
    }
  }
}

impl SerializedSTInt8 for STInt8 {
  fn serialize(value: u8) -> Vec<u8> {
     value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//----------------------------------------------------
pub trait SerializedSTInt16 {
  fn serialize(value: u16) -> Vec<u8>;
  fn parse();
}

//STInt16
#[derive(TypeName, Debug)]
pub struct STInt16 {
  pub id: i32,
  pub value: u16,
}
impl STInt16 {
  pub fn new(value: u16) -> Self {
    STInt16 {
      id: 1, //default value
      value: value,
    }
  }
}

impl SerializedSTInt16 for STInt16 {
  fn serialize(value: u16) -> Vec<u8> {
     value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//----------------------------------------------------
pub trait SerializedSTInt32 {
  fn serialize(value: u32) -> Vec<u8>;
  fn parse();
}
//STInt32
#[derive(TypeName, Debug)]
pub struct STInt32 {
  pub id: i32,
  pub value: u32,
}
impl STInt32 {
  pub fn new(value: u32) -> Self {
    STInt32 {
      id: 2, //default value
      value: value,
    }
  }
}

impl SerializedSTInt32 for STInt32 {
  fn serialize(value: u32) -> Vec<u8> {
    value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//STInt64
pub trait SerializedSTInt64 {
  fn serialize(value: u64) -> Vec<u8>;
  fn parse();
}
#[derive(TypeName, Debug)]
pub struct STInt64 {
  pub id: i32,
  pub value: u64,
}
impl STInt64 {
  pub fn new(value: u64) -> Self {
    STInt64 {
      id: 3, //default value
      value: value,
    }
  }
}

impl SerializedSTInt64 for STInt64 {
  fn serialize(value: u64) -> Vec<u8> {
    value.to_be_bytes().to_vec()
  }

  fn parse() {}
}

//STHash128
#[derive(TypeName, Debug)]
pub struct STHash128 {
  pub id: i32,
  pub value: u128,
}
impl STHash128 {
  pub fn new(value: u128) -> Self {
    STHash128 {
      id: 4, //default value
      value: value,
    }
  }
}

impl SerializedType for STHash128 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STHash256
#[derive(TypeName, Debug)]
pub struct STHash256 {
  pub id: i32,
  pub value: u128,
}
impl STHash256 {
  pub fn new(value: u128) -> Self {
    STHash256 {
      id: 5, //default value
      value: value,
    }
  }
}

impl SerializedType for STHash256 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STHash160
#[derive(TypeName, Debug)]
pub struct STHash160 {
  pub id: i32,
  pub value: u128,
}
impl STHash160 {
  pub fn new(value: u128) -> Self {
    STHash160 {
      id: 17, //default value
      value: value,
    }
  }
}

impl SerializedType for STHash160 {
  fn serialize(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }

  fn parse(&self) {}
}

//STCurrency
// #[derive(TypeName, Debug)]
// pub struct STCurrency {
//     pub id: i32,
// }
// impl STCurrency {
//     pub fn new() -> Self {
//         STCurrency {
//             id:
//         }
//     }
// }



pub trait SerializedSTAmount {
  fn serialize(value: Amount) -> Vec<u8>;
  fn parse();
}

//STAmount
pub struct STAmount {
    pub id: i32,
}
impl STAmount {
    pub fn new() -> Self {
        STAmount {
            id: 6,
        }
    }
}
impl SerializedSTAmount for STAmount {
    fn serialize(amount: Amount) -> Vec<u8> {
        //SWTC
        if amount.is_native {
            let mut value_hex = amount.value.to_str_radix(16);

            // Enforce correct length (64 bits)
            if value_hex.len() > 16 {
                // throw new Error('Amount Value out of bounds');
            }

            while value_hex.len() < 16 {
                value_hex.insert(0, '0');
            }

            let mut value_bytes = hex::decode(value_hex).unwrap();

            // Clear most significant two bits - these bits should already be 0 if
            // Amount enforces the range correctly, but we'll clear them anyway just
            // so this code can make certain guarantees about the encoded value.
            value_bytes[0] &= 0x3f;

            if !amount.is_negative {
                value_bytes[0] |= 0x40;
            }

            return value_bytes.to_vec();

        } else {

            let mut so: Vec<u8> = vec![];

            //For other non-native currency
            //1. Serialize the currency value with offset
            //Put offset
            let mut hi = 0;

            let mut big_hi = Zero::zero();
            let mut big_lo = Zero::zero();

            // First bit: non-native
            hi |= 1 << 31;

            if !amount.is_zero() {
                // Second bit: non-negative?
                if !amount.is_negative() {
                    hi |= 1 << 30;
                }

                // Next eight bits: offset/exponent
                hi |= ((97 + amount.offset) & 0xff) << 22;

                // Remaining 54 bits: mantissa
                let mut mantissa = amount.value.clone();
                mantissa = mantissa.shr(32usize);

                let ff = BigInt::parse_bytes(b"3fffff", 16).unwrap();
                mantissa.bitand_assign(ff);

                big_hi = BigInt::from(hi);
                big_hi.bitor_assign(mantissa);

                /////////lo
                //words1 * 0x4ffffff
                let words1 = amount.value.clone();
                let sign = &words1.sign();

                let ff4 = BigInt::parse_bytes(b"4ffffff", 16).unwrap();
                let ret = words1.mul(&ff4);
                big_lo = ret.to_bigint().unwrap().bitand(BigInt::parse_bytes(b"ffffffff", 16).unwrap());
                if *sign != Sign::Minus {
                    big_lo = -1 * big_lo;
                }
            }

            let arr = vec![big_hi, big_lo];
            let l = arr.len();
            let mut bl: BigInt = Zero::zero();
            if l != 0 {
                let index = l - 1 as usize;
                if let Some(x) = arr.get(index) {
                    let base32 = BigInt::from(32);
                    let base16 = BigInt::parse_bytes(b"10000000000", 16);
                    bl = (l - BigInt::from(1)).checked_mul(&base32).unwrap() + x.checked_div(&base16.unwrap()).unwrap().bitor(&base32);
                }
            }

            let mut tmp: BigInt = Zero::zero();
            let mut tmparray = vec![];
            let mut i = 0;
            let bl = format!("{}", bl);
            let bl = bl.parse::<usize>().unwrap(); //64
            while i < bl / 8 {
                if i & 3 == 0 {
                    tmp = arr[i / 4].clone();
                }

                let x = tmp.clone().shr(24usize);
                let b_mask = BigInt::parse_bytes(b"ff", 16).unwrap();

                tmparray.push(x.bitand(b_mask).to_str_radix(10).parse::<u8>().unwrap());

                tmp <<= 8;

                i += 1;
            }
            if tmparray.len() > 8 {
                panic!("Invalid byte array length in AMOUNT value representation");
            }

            so.extend_from_slice(&tmparray.as_slice());

            //2. Serialize the currency info with currency code and issuer
            // Currency (160-bit hash)
            let tum_bytes = amount.tum_to_bytes();
            so.extend_from_slice(&tum_bytes);

            // Issuer (160-bit hash)
            let issuer = amount.issuer();
            so.extend_from_slice(&decode_j_address(issuer.to_string()).unwrap());

            return so;
        }
    }

    fn parse() {}
}

pub trait SerializedSTVL {
  fn serialize(value: &String) -> Vec<u8>;
  fn parse();
}
pub fn serialize_varint(byte_data: &mut Vec<u8>) -> Vec<u8> {
    let mut val = byte_data.len();
    let mut v: Vec<u8> = vec![];
    if val <= 192 {
        let mut t = vec![val as u8];
        v.append(&mut t);
    } else if val <= 12480 {
        val -= 193;
        let mut t = [(193 + (val >> 8)) as u8, (val & 0xff) as u8].to_vec();
        v.append(&mut t);
    } else if val <= 918744 {
        val -= 12481;

        let mut t = [(241 + (val >> 16)) as u8, (val >> 8 & 0xff) as u8, (val & 0xff) as u8].to_vec();
        v.append(&mut t);
    }

    v.append(byte_data);

    v
}

//STVL
pub struct STVL {
    pub id: i32,
}
impl STVL {
    pub fn new() -> Self {
        STVL {
            id: 7,
        }
    }

}
impl SerializedSTVL for STVL {
    fn serialize(value: &String) -> Vec<u8> {
        let value = value.trim_start_matches("\"");
        let value = value.trim_end_matches("\"");

        let mut v: Vec<u8> = vec![];
        if let Ok(mut data) = hex::decode(value) {
            v = serialize_varint(&mut data);
        }

        v
    }

    fn parse() {

    }
}

///////////////////////////////////////////
pub trait SerializedSTAccount {
  fn serialize(value: String) -> Vec<u8>;
  fn parse();
}
//STAccount
pub struct STAccount {
    pub id: i32,
}
impl STAccount {
    pub fn new() -> Self {
        STAccount {
            id: 8,
        }
    }
}
impl SerializedSTAccount for STAccount {
  fn serialize(value: String) -> Vec<u8> {
      let mut byte_data = util::decode_j_address(value).unwrap();

      serialize_varint(&mut byte_data)
  }

  fn parse() {}
}


//STPathSet
pub struct STPathSet {
    pub id: i32,
    //
}
impl STPathSet {
    pub fn new() -> Self {
        STPathSet {
            id: 18,
        }
    }
}

//STVector256
pub struct STVector256 {
    pub id: i32,
}
impl STVector256 {
    pub fn new() -> Self {
        STVector256 {
            id: 19,
        }
    }
}

//STMemo
///////////////////////////////////////////
pub trait SerializedSTMemos {
  fn serialize(value: &String) -> Vec<u8>;
  fn parse();
}

pub struct STMemo {}
impl STMemo {
    pub fn new() -> Self {
        STMemo { }
    }
}

impl SerializedSTMemos for STMemo {
    fn serialize(value: &String) -> Vec<u8> {
        //convertStringToHex
        STVL::serialize(&value.to_ascii_uppercase())
    }

    fn parse() {}
}

//STArray
pub trait SerializedSTArray {
  fn serialize(value: &Vec<String>) -> Vec<u8>;
  fn parse();
}
pub struct STArray {}
impl STArray {
    pub fn new() -> Self {
        STArray {}
    }
}
impl SerializedSTArray for STArray {
    fn serialize(value: &Vec<String>) -> Vec<u8> {
        let mut v: Vec<u8> = vec![];
        let mut i = 0;
        while i < value.len() {

            v.extend_from_slice(&STMemo::serialize(&value[i]));

            i += 1;
        }

        v
    }

    fn parse() {}
}
