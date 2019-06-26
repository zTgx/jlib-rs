
extern crate num;
use num::bigint::{BigInt, Sign};
use std::str::FromStr;


const CURRENCY_XNS: u8 = 0;
const CURRENCY_ONE: u8 = 1;
const XNS_PRECISION: u8 = 6;

//big number
// const BI_5: u8 = 5;
// const BI_7: u8 = 7;
// const BI_10: u8 = 10;


#[derive(Debug)]
pub struct Amount {
    pub value: Option<BigInt>,  //big number
    pub offset: u32,  //0 for SWTC
    pub is_native: bool, //Default to SWTC
    pub is_negative: bool,
    pub currency: Option<String>,
    pub issuer: Option<String>,
}
impl Amount {
    pub fn new(value: Option<BigInt>, offset: u32, is_native: bool,
               is_negative: bool, currency: Option<String>, issuer: Option<String>) -> Self {
        
        Amount {
            value: value,
            offset: offset,
            is_native: is_native,
            is_negative: is_negative,
            currency: currency,
            issuer: issuer,
        }
    }

    pub fn from_json(j: String) -> Self {
        let x = (j.parse::<f64>().unwrap() * 1000000.0) as u64;
        let mut value: BigInt = BigInt::from_str(x.to_string().as_str()).unwrap();
        // let b = BigInt::from_bytes_be(Sign::Plus, b"1000000");
        // println!("b : {:?}", &b);
        // value = value.checked_mul(&b).unwrap();
        println!("value : {:?}", value);

        Amount {
            value: Some(value),
            offset: 0,
            is_native: true,
            is_negative: false,
            currency: None,
            issuer: None,
        }
    }
}

impl Default for Amount {
    fn default() -> Self {
        Amount { 
            value: None,
            offset: 0,
            is_native: true,
            is_negative: false,
            currency: None,
            issuer: None,
        }
    }
}

