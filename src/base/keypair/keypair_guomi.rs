use libsm::sm3::hash::Sm3Hash; 
use libsm::sm2::signature::{Pubkey, Seckey};
use libsm::sm2::ecc::EccCtx;
use basex_rs::{BaseX, SKYWELL, Encode}; 
use crate::base::curve::ripemd160::JRipemd160;
use hex;
use crate::base::crypto::traits::generator::GeneratorI;

// The order of the sm2p256v1 curve
pub const CURVE_ORDER_SM2P256V1: &[u8; 32] = &[
    0xff, 0xff, 0xff, 0xfe, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x72, 0x03, 0xdf, 0x6b, 0x21, 0xc6, 0x05, 0x2b,
    0x53, 0xbb, 0xf4, 0x09, 0x39, 0xd5, 0x41, 0x23
];

// The zero of sm2p256v1 curve
pub const ZERO_ORDER_SM2P256V1: &[u8; 32] = &[
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

static PREFIX_PUBLIC_KEY: &[u8] = &[0];


pub struct KeypairGuomi {
}

impl KeypairGuomi {
    pub fn new() -> Self {
        KeypairGuomi {
        }
    }

    pub fn build(&self, _masterphrase: &String) -> Result<(String, String), &'static str> {
        let x = [122, 174, 27, 9, 106, 99, 19, 141, 183, 13, 193, 169, 42, 98, 171, 188].to_vec();
        let private_generator = self.private_generator(&x);
        println!("private_generator = {:?}", private_generator);
        let public_generator  = self.public_generator(&private_generator);
        println!("public_generator: {:?}", public_generator);

        let private_key = self.generate_private_key(&private_generator, &public_generator);
        println!("private key: {:?}", hex::encode(private_key));

        let public_key  = self.generate_public_key(&public_generator);
        println!("public key: {:?}", public_key);
        println!("public_key_hex : {:?}", hex::encode(&public_key));

        //public key
        {
            {
                let mut vec = Vec::new();
                vec.extend_from_slice(&[35]);
                vec.extend_from_slice(&public_key);

                println!("vec: {:?}", vec);
                let mut hash = Sm3Hash::new(&vec);
                let hash1 = hash.get_hash();
                println!("hash1: {:?}", hash1);
                let mut hash2 = Sm3Hash::new(&hash1);
                let digest = hash2.get_hash();
                println!("hash2: {:?}", digest);
            
                let checksum = digest.get(..4).unwrap().to_vec();
                println!("checksum: {:?}", checksum);
        
                //add: 0 + 20 + 4
                let mut vec: Vec<u8> = Vec::new();
                vec.extend_from_slice(&[35]);
                vec.extend_from_slice(&public_key);
                vec.extend_from_slice(&checksum);
        
                //public key。
                let pk = BaseX::new(SKYWELL).encode(&vec);
                println!("------pk: {:?}", pk);
    
            }
        }
        

        let human_readable_public_key = self.human_readable_public_key(&public_key);
        println!("human_readable_public_key: {}", human_readable_public_key);

        Ok(
            (
                "private...".to_string(),
                "public....".to_string()
            )
        )
    }
}

impl GeneratorI for KeypairGuomi {
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

    fn public_key_hash_generator(&self, public_generator: &Vec<u8>)  -> Vec<u8> {
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

            if privx < n {
                let ret = privx.to_bytes_be();

                println!("ret: {:?}, seq = {}", ret, seq);

                println!("public_key_hash_generator = {:02X?}", ret);

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
        let private_key_hash_generator = self.public_key_hash_generator(&public_generator);
        println!("private_key_hash_generator: {:?}", private_key_hash_generator);

        let a: Seckey = Seckey::from_bytes_be(&private_key_hash_generator);
        let b: Seckey = Seckey::from_bytes_be(&private_generator);
        let c = a + b;

        let private_key = c.to_bytes_be();
        return private_key;
    }

    fn generate_public_key(&self, public_generator: &Vec<u8>) -> Vec<u8> {
        let public_key_hash_generator = self.public_key_hash_generator(&public_generator);

        let ecc_ctx = EccCtx::new();
        let m = Seckey::from_bytes_be(&public_key_hash_generator);
        let a: Pubkey = ecc_ctx.g_mul(&m);

        println!("a: {:?}", ecc_ctx.point_to_bytes(&a, true));
        let b = ecc_ctx.bytes_to_point(&public_generator).unwrap();

        println!("b: {:?}", ecc_ctx.point_to_bytes(&b, true));

        let c = ecc_ctx.add(&a, &b);

        println!("c: {:?}", ecc_ctx.point_to_bytes(&c, true));

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

        println!("ripp: {:?}", ripemd160_hash);

        // 对 0 + 20 进行 hash
        let mut vec = Vec::new();
        vec.extend_from_slice(PREFIX_PUBLIC_KEY);
        vec.extend_from_slice(&ripemd160_hash);
    
        let mut hash = Sm3Hash::new(&vec);
        let hash1 = hash.get_hash();
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();
    
        let checksum = digest.get(..4).unwrap().to_vec();

        //add: 0 + 20 + 4
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(PREFIX_PUBLIC_KEY);
        vec.extend_from_slice(&ripemd160_hash);
        vec.extend_from_slice(&checksum);

        BaseX::new(SKYWELL).encode(&vec)
    }
}