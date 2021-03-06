pub mod secp256k1;
pub mod ed25519;
pub mod sha256;
pub mod ripemd160;
pub mod sm2p256v1;

use basex_rs::{BaseX, SKYWELL, Decode};
use crate::base::ring::{digest};
use crate::base::data::constants::{
    CURVE_ORDER, 
    CURVE_ZERO,

    CURVE_SM2_ORDER
};

use libsm::sm3::hash::Sm3Hash;
use libsm::sm2::ecc::EccCtx;
use libsm::sm2::signature::Seckey;

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

        let mut zero = Vec::new();
        zero.extend_from_slice(CURVE_ZERO);
        if key.as_slice() < CURVE_ORDER && key > zero {
            return key;
        }

        i += 1;
    } // end while
}

// 生成Private generator
pub fn scalar_sm2(bytes: &[u8], discrim: Option<u8>) -> Vec<u8> {
    let mut i = 0u32;
    loop {
        // We hash the bytes to find a 256 bit number, looping until we are sure it
        // is less than the order of the curve.

        let mut vec = Vec::new();
        vec.extend_from_slice(&bytes);
        vec.extend_from_slice(&i.to_be_bytes());

        if let Some(x) = discrim {
            //as i32
            vec.extend_from_slice(&(x as i32).to_be_bytes());
        }

        let mut ctx = Sm3Hash::new(&vec);
        let key = ctx.get_hash();

        if key < *CURVE_SM2_ORDER && key > *CURVE_ZERO {
            return key.to_vec();
        }

        i += 1;
    } // end while
}

// 生成 public generator
/*
需要：private generator，椭圆常数G
1.	Private generator与椭圆常数G进行乘法运算
2.	获得椭圆点（x，y）
3.	压缩成(02+X 如Y 偶), 或(03+X 如 Y 奇)
4.	获得public generator 33字节
*/
pub fn public_generator(private_generator: &[u8]) -> Vec<u8> {
    let ctx = EccCtx::new();

    let src = Seckey::from_bytes_be(&private_generator);
    let ret = ctx.g_mul(&src);

    return ctx.point_to_bytes(&ret, true).to_vec();
}
 