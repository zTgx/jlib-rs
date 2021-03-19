// TODO:: 三个接口都在重复计算 GeneratorI中的数据！！！
// 用内置一个Data来缓存计算的generator.

use crate::address::traits::generator::GeneratorI;
use libsm::sm3::hash::Sm3Hash; 
use libsm::sm2::signature::{Pubkey, Seckey};
use libsm::sm2::ecc::EccCtx;
use basex_rs::{BaseX, SKYWELL, Encode}; 
use crate::address::constants::VersionEncoding;
use crate::base::crypto::ripemd160::JRipemd160;


// type Blob = Vec<u8>;
pub struct GeneratorSM2P256V1 {
    // data: Blob,
}
impl GeneratorSM2P256V1 {
    pub fn new() -> Self {
        GeneratorSM2P256V1 {
            // data: Blob::new()
        }
    }
}

impl GeneratorI for GeneratorSM2P256V1 {
    fn private_generator(&self, masterphrase: &Vec<u8>)  -> Vec<u8> {
        // println!("xxx: {:?}", masterphrase);

        let mut seq = 0u32;

        let ecc_ctx = EccCtx::new();
        let n: Seckey = ecc_ctx.get_n();

        let mut vec = Vec::new();
        loop {
            vec.extend_from_slice(&masterphrase);
            vec.extend_from_slice(&seq.to_be_bytes());
        
            let mut sm3_hash = Sm3Hash::new(&vec);
            let digest = sm3_hash.get_hash();

            let privx = Seckey::from_bytes_be(&digest);

            //TODO:: 对 zero 值还需要进行判断！！！
            if privx < n {
                let ret = privx.to_bytes_be();

                // println!("ret: {:?}, seq = {}", ret, seq);

                return ret;
            }
            
            // We hash the bytes to find a 256 bit number, looping until we are sure it
            // is less than the order of the curve.
            // if digest < *CURVE_ORDER_SM2P256V1 && digest > *ZERO_ORDER_SM2P256V1 {
            //     return digest.to_vec();
            // }
    
            vec.clear();
            seq += 1;
        } // end while    
    }

    fn public_key_root_generator(&self, public_generator: &Vec<u8>)  -> Vec<u8> {
        let seq = 0u32;
        let mut sub_seq = 0u32;

        let ecc_ctx = EccCtx::new();
        let n: Seckey = ecc_ctx.get_n();

        let mut vec = Vec::new();
        loop {
            vec.extend_from_slice(&public_generator);
            vec.extend_from_slice(&seq.to_be_bytes());
            vec.extend_from_slice(&sub_seq.to_be_bytes());
        
            let mut sm3_hash = Sm3Hash::new(&vec);
            let digest = sm3_hash.get_hash();

            let privx = Seckey::from_bytes_be(&digest);

            //TODO:: 对 zero 值还需要进行判断！！！
            if privx < n {
                let ret = privx.to_bytes_be();

                // println!("ret: {:?}, seq = {}", ret, seq);

                // println!("public_key_root_generator = {:02X?}", ret);

                return ret;
            }
            
            // We hash the bytes to find a 256 bit number, looping until we are sure it
            // is less than the order of the curve.
            // if digest < *CURVE_ORDER_SM2P256V1 && digest > *ZERO_ORDER_SM2P256V1 {
            //     return digest.to_vec();
            // }
    
            vec.clear();
            sub_seq += 1;
        } // end while    
    }

    fn public_generator(&self, private_generator: &Vec<u8>) -> Vec<u8> {
        let ecc_ctx = EccCtx::new();

        let m = Seckey::from_bytes_be(&private_generator);
        let x: Pubkey = ecc_ctx.g_mul(&m);
        let bytes: Vec<u8> = ecc_ctx.point_to_bytes(&x, true); // 压缩

        return bytes;
    }

    fn generate_private_key(&self, private_generator: &Vec<u8>, public_generator: &Vec<u8>) -> Vec<u8> {
        let private_key_hash_generator = self.public_key_root_generator(&public_generator);
        // println!("private_key_hash_generator: {:?}", private_key_hash_generator);

        let a: Seckey = Seckey::from_bytes_be(&private_key_hash_generator);
        let b: Seckey = Seckey::from_bytes_be(&private_generator);
        let c = a + b;

        let private_key = c.to_bytes_be();
        return private_key;
    }

    fn generate_public_key(&self, public_generator: &Vec<u8>) -> Vec<u8> {
        let public_key_root_generator = self.public_key_root_generator(&public_generator);

        let ecc_ctx = EccCtx::new();
        let m = Seckey::from_bytes_be(&public_key_root_generator);
        let a: Pubkey = ecc_ctx.g_mul(&m);

        // println!("a: {:?}", ecc_ctx.point_to_bytes(&a, true));
        let b = ecc_ctx.bytes_to_point(&public_generator).unwrap();

        // println!("b: {:?}", ecc_ctx.point_to_bytes(&b, true));

        let c = ecc_ctx.add(&a, &b);

        // println!("c: {:?}", ecc_ctx.point_to_bytes(&c, true));

        let public_key = ecc_ctx.point_to_bytes(&c, true);
        return public_key;
    }

    fn human_readable_public_key(&self, public_key: &Vec<u8>) -> String {
        let mut sm3_hash =  Sm3Hash::new(&public_key);
        let digest = sm3_hash.get_hash();

        let mut ripemd160 = JRipemd160::new();
        ripemd160.input(&digest);
        let ripemd160_hash: &mut [u8] = &mut [0u8;20];
        ripemd160.result(ripemd160_hash);

        // println!("ripp: {:?}", ripemd160_hash);

        // 对 0 + 20 进行 hash
        let mut vec = Vec::new();
        vec.extend_from_slice(&(VersionEncoding::VerAccountId as u8).to_be_bytes());
        vec.extend_from_slice(&ripemd160_hash);
    
        let mut hash = Sm3Hash::new(&vec);
        let hash1 = hash.get_hash();
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();
    
        let checksum = digest.get(..4).unwrap().to_vec();

        //add: 0 + 20 + 4
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&(VersionEncoding::VerAccountId as u8).to_be_bytes());
        vec.extend_from_slice(&ripemd160_hash);
        vec.extend_from_slice(&checksum);

        BaseX::new(SKYWELL).encode(&vec)
    }
}
