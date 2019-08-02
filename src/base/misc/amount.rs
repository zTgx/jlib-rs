#![allow(unused)]
use std::str::FromStr;
extern crate num;
use num::bigint::{BigInt};
use num::{Zero, One};
use crate::cast_rs::str_t;
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

static CURRENCY_NAME_LEN: usize = 3;//货币长度
static CURRENCY_NAME_LEN2: usize = 6;//货币长度

#[derive(Debug)]
pub struct Amount {
    pub value      : BigInt,  //big number
    pub offset     : i32,    //0 for SWTC
    pub is_native  : bool,   //Default to SWTC
    pub is_negative: bool,
    pub currency   : Option<String>,
    pub issuer     : Option<String>,
}
impl Amount {
    pub fn new(value: BigInt, offset: i32, is_native: bool, is_negative: bool, currency: Option<String>, issuer: Option<String>) -> Self {
        Amount {
            value       : value,
            offset      : offset,
            is_native   : is_native,
            is_negative : is_negative,
            currency    : currency,
            issuer      : issuer,
        }
    }

    pub fn from_ramount(ramount: &RAmount) -> Amount {
        if ramount.is_native() {
            let mut point_len = 0;
            let mut value = String::from( ramount.value.as_str() );
            if value.as_str().contains(".") {
                let tmp =  String::from( ramount.value.as_str() );
                let mut v: Vec<&str> = tmp.split('.').collect();
                let mut right = v[1];
                let mut right_len = v[1].len() - 1;
                loop {
                    if right.chars().nth(right_len).unwrap() != '0' {
                        break;
                    }

                    if right_len.checked_sub(1).is_none() {
                        break;
                    }

                    right_len -= 1;
                }
                right_len += 1;

                point_len = right_len;
                value = right.get(..right_len).unwrap().parse::<u64>().unwrap().to_string();
            } else {
                value = String::from( ramount.value.as_str() );
            }
            let base_str = Amount::calc_base_str(point_len);
            let mut value: BigInt = BigInt::from_str(value.as_str()).unwrap();
            let base: BigInt = BigInt::from_str(base_str).unwrap();
            let mut evalue = value.checked_mul(&base).unwrap();
            let max: BigInt = BigInt::from_str(BI_XNS_MAX).unwrap();
            if evalue > max {
                evalue = Zero::zero();
            }

            let neg = value.is_zero();
            Amount {
                value       : evalue,
                offset      : 0,
                is_native   : true,
                is_negative : neg,
                currency    : Some("SWT".to_string()),
                issuer      : None,
            }
        } else {
            //non-native amount
            if ramount.issuer.is_some() {
                // if (base_wallet.isValidAddress(in_json.issuer)) {
                //TODO, need to find a better way for extracting the exponent and digits
                // let vpow = Amount::calc_exponential();

                let vpow = str_t::to_expo(ramount.value.as_str()).unwrap();
                let vpow = str_t::get_exponent(&vpow);

                let offset = 15 - vpow;
                let base10 = BigInt::from(10 as usize);

                let mut base_offset = BigInt::from(offset);
                if vpow < 0 {
                    base_offset = BigInt::from(offset + vpow);
                }

                let factor = base10.modpow(&base_offset, &BigInt::from_str("10000000000000000000").unwrap());

                //0.01
                let mut rv = String::from(ramount.value.as_str());

                if vpow < 0 {
                    rv = rv.replace(".", "");
                }
                let mut index = 0;
                loop {
                    if index >= rv.len() {
                        break;
                    }

                    if let Some(x) = rv.as_str().chars().nth(index) {
                        if x == '0' {
                            rv = rv.replace("0", "");
                        } else {
                            break;
                        }
                    }

                    index += 1;
                }

                let mut value: BigInt = BigInt::from_str(rv.as_str()).unwrap();

                let base: BigInt = BigInt::from(factor);
                let mut evalue = value.checked_mul(&base).unwrap();

                let offset: i32 = -1 * offset as i32;

                let mut currency = "".to_string();
                if let Some(ref x) = ramount.currency {
                    currency = x.to_string();
                }

                let mut issuer = "".to_string();
                if let Some(ref x) = ramount.issuer {
                    issuer = x.to_string();
                }

                Amount {
                    value: evalue,
                    offset: offset,
                    is_native: false,
                    is_negative: false,
                    currency: Some( currency ),
                    issuer: Some( issuer ),
                }

            } else {
                panic!("Amount.parse_json: Input JSON has invalid issuer info!");
            }
        }
    }

    pub fn f64_2_usize_str(value_str: &String) -> String {
        //0.01
        let mut rv = String::from(value_str.as_str());
        rv = rv.replace(".", "");
        let mut index = 0;
        loop {
            if index >= rv.len() {
                break;
            }

            if let Some(x) = rv.as_str().chars().nth(index) {
                if x == '0' {
                    rv = rv.replace("0", "");
                } else {
                    break;
                }
            }

            index += 1;
        }

        rv
    }

    pub fn from_value(value: u64) -> Self {
        Amount {
            value: BigInt::from(value),
            offset: 0,
            is_native: true,
            is_negative : false,
            currency: Some("SWT".to_string()),
            issuer: None,
        }
    }

    pub fn from_json(j: String) -> Self {
        Amount {
            value: BigInt::from_str(j.as_str()).unwrap(),
            offset: 0,
            is_native: true,
            is_negative : false,
            currency: Some("SWT".to_string()),
            issuer: None,
        }
    }

    pub fn is_zero(&self) -> bool {
        false
    }

    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    pub fn issuer(&self) -> String {
        let mut issuer = "".to_string();
        if let Some(ref x) = self.issuer {
            issuer = x.to_string();
        }

        issuer
    }

    /*
    * Convert the internal Tum Code
    *  to byte array
    * for serialization.
    * Input: a string represents the Tum.
    * Output: Bytes array of size 20 (UINT160).
    *
    */
    pub fn tum_to_bytes(&self) -> Vec<u8> {
        let mut so: Vec<u8> = vec![0; 20];
        if let Some(ref cur) = self.currency {
            let len = cur.len();

            if len >= CURRENCY_NAME_LEN && len <= CURRENCY_NAME_LEN2 {
                let end = 14;
                let mut len = len - 1;
                let mut index = len as isize;
                loop {
                    if index < 0 {
                        break;
                    }

                    so[end - index as usize] = cur.chars().nth(len - index as usize).unwrap() as u8 & 0xff;

                    index -= 1;
                }
            }
        }

        so
     }

     pub fn calc_base_str(point_len: usize) -> &'static str {
         let mut base_str = "1000000";
         match point_len {
             0 => {
                 base_str = "1000000";
             },

             1 => {
                 base_str = "100000";
             },

             2 => {
                 base_str = "10000";
             },

             3 => {
                 base_str = "1000";
             },

             4 => {
                 base_str = "100";
             },

             5 => {
                 base_str = "10";
             },

             6 => {
                 base_str = "1";
             },

             _ => {
                 panic!("invalid value.");
             }
         }

         base_str
     }
}

impl Default for Amount {
    fn default() -> Self {
        Amount {
            value: Zero::zero(),
            offset: 0,
            is_native: true,
            is_negative: false,
            currency: None,
            issuer: None,
        }
    }
}
