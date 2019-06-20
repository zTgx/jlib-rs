extern crate bs58;

use super::constants::*;
use ring::{digest, test};

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

    println!("source : {:?} and len : {}", source, source.len());
        
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
    println!("string: {}", string);
    println!("digits: {:?}", digits.len());
    // convert digits to a string
    let mut q: i32 = (digits.len() - 1) as i32;
    while q >= 0 {

        let uu: u8 = ALPHABET[digits[q as usize] as usize];
        let xx = uu as char;
        println!("char : {}", xx);
        string.push( xx );

        q -= 1;
    }

    String::from(string.as_str())
}

//entropy的生成方式: 取值index范围，1 ~ 倒数第5
pub fn entropy(secret: String) -> Vec<u8> {
    // let prefix = "00";
    //ssndDcNoc4FvwVPveY3KbfWh8fNh3
    let buf = bs58::decode(secret).with_alphabet(ALPHABET).into_vec().unwrap();
    println!("raw data after decode : {:?}", buf);

    buf[1..buf.len()-4].to_vec()
}

