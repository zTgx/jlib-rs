pub mod secp256k1;
pub mod ed25519;
pub mod sha256;
pub mod ripemd160;

use basex_rs::{BaseX, SKYWELL, Decode};
use crate::base::ring::{digest};
use crate::base::data::constants::{CURVE_ORDER, CURVE_ZERO};

//entropy的生成方式: 取值index范围，1 ~ 倒数第5
pub fn entropy(secret: &String) -> Option<Vec<u8>> {
    // let buf = BaseX::decode(secret.to_string()).unwrap();
    // buf[1..buf.len()-4].to_vec()

    if let Some(buf) = BaseX::new(SKYWELL).decode(secret.to_string()) {
        return Some( buf[1..buf.len()-4].to_vec() );
    }

    None
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
