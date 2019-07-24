#![allow(unused)]

extern crate num;
use num::bigint::{BigInt};
use std::str::FromStr;
use crate::message::common::amount::Amount as RAmount;

const CURRENCY_XNS: u8 = 0;
const CURRENCY_ONE: u8 = 1;
const XNS_PRECISION: u8 = 6;

//big number
// const BI_5: u8 = 5;
// const BI_7: u8 = 7;
// const BI_10: u8 = 10;
static BI_XNS_MAX: &'static str = "9000000000000000000"; //"9e18";  //new BigInteger('9000000000000000000'), // Json wire limit.
static BI_XNS_MIN: &'static str = "-9000000000000000000";//"-9e18"; //new BigInteger('-9000000000000000000'),// Json wire limit.

#[derive(Debug)]
pub struct Amount {
    pub value      : Option<BigInt>,  //big number
    pub offset     : u32,  //0 for SWTC
    pub is_native  : bool, //Default to SWTC
    pub is_negative: bool,
    pub currency   : Option<String>,
    pub issuer     : Option<String>,
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
        println!("j: {:?}", &j);
        // let mut ret: Amount = Amount::default();
                Amount {
                    value: Some(BigInt::from_str("10000").unwrap()),
                    offset: 0,
                    is_native: true,
                    is_negative : false, // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                    currency: Some("SWT".to_string()),
                    issuer: None,
                }
        // if let Ok(amount) = RAmount::from_str(j.as_str()) {
        //     if amount.is_native() {
        //         let mut value: BigInt = BigInt::from_str(amount.value.as_str()).unwrap();
        //         let max: BigInt = BigInt::from_str(BI_XNS_MAX).unwrap();
        //         if value > max {
        //             // value = BigInt::from_str("").unwrap();
        //         }
        //         println!("big int");
        //         ret = Amount {
        //             value: Some(value),
        //             offset: 0,
        //             is_native: true,
        //             is_negative : false, // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        //             currency: Some("SWT".to_string()),
        //             issuer: None,
        //         }
        //     } else {
        //         //Non-swt
        //         //                 this._currency = in_json.currency;
        //         // this._is_native = false;
        //         // if (typeof in_json.issuer != 'undefined' &&
        //         //     in_json.issuer != null) {
        //         //     if (base_wallet.isValidAddress(in_json.issuer)) {
        //         //         this._issuer = in_json.issuer;
        //         //         //TODO, need to find a better way for extracting the exponent and digits
        //         //         var vpow = new Number(in_json.value);
        //         //         vpow = String(vpow.toExponential());
        //         //         vpow = Number(vpow.substr(vpow.lastIndexOf("e") + 1));

        //         //         var offset = 15 - vpow;
        //         //         var factor = Math.pow(10, offset);
        //         //         var newvalue = (new bignumber(in_json.value).multipliedBy(factor)).toString();
        //         //         this._value = new BigInteger(newvalue, 10);
        //         //         this._offset = -1 * offset;
        //     }
        // }

        // ret

        // let x = (j.parse::<f64>().unwrap() * 1000000.0) as u64;
        // let value: BigInt = BigInt::from_str(x.to_string().as_str()).unwrap();
        // let b = BigInt::from_bytes_be(Sign::Plus, b"1000000");
        // println!("b : {:?}", &b);
        // value = value.checked_mul(&b).unwrap();
    }

    pub fn parse_swt_value(strr: String) {

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

