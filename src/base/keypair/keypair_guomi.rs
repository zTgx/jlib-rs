use crate::base::keypair::keypair_trait::KeypairI;
use libsm::sm3::hash::Sm3Hash; 
use libsm::sm2::signature::{Pubkey, Seckey};
use libsm::sm2::ecc::EccCtx;
use basex_rs::{BaseX, SKYWELL, Encode}; 
use crate::base::curve::ripemd160::JRipemd160;

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

    // pub fn build(&self) -> Result<(String, String), &'static str> {
    //     Ok()
    // }
}

impl KeypairI for KeypairGuomi {
    fn private_generator(&self, masterphrase: &Vec<u8>)  -> Vec<u8> {
        let mut seq = 0u32;

        let mut vec = Vec::new();
        loop {
            vec.extend_from_slice(&masterphrase);
            vec.extend_from_slice(&seq.to_be_bytes());
        
            let mut sm3_hash = Sm3Hash::new(&vec);
            let digest = sm3_hash.get_hash();
            
            // We hash the bytes to find a 256 bit number, looping until we are sure it
            // is less than the order of the curve.
            if digest < *CURVE_ORDER_SM2P256V1 && digest > *ZERO_ORDER_SM2P256V1 {
                return digest.to_vec();
            }
    
            vec.clear();
            seq += 1;
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
        let (seq, mut sub_seq) = (0u32, 0u32);

        let mut vec = Vec::new();

        loop {
            vec.extend_from_slice(&public_generator);
            vec.extend_from_slice(&seq.to_be_bytes());
            vec.extend_from_slice(&sub_seq.to_be_bytes());
    
            let mut sm3_hash =  Sm3Hash::new(&vec);
            let digest = sm3_hash.get_hash();
    
            if digest < *CURVE_ORDER_SM2P256V1 && digest > *ZERO_ORDER_SM2P256V1 {
                let hash = digest.to_vec();

                //与private generator相加，获得结果，32字节
                let hash = Seckey::from_bytes_be(&hash);
                let prg  = Seckey::from_bytes_be(&private_generator);

                let add = hash + prg;
                return add.to_bytes_be();
            }

            vec.clear();
            sub_seq += 1;
        }
    }

    fn generate_public_key(&self, public_generator: &Vec<u8>) -> Vec<u8> {
        let (seq, mut sub_seq) = (0u32, 0u32);

        let mut vec = Vec::new();

        loop {
            vec.extend_from_slice(&public_generator);
            vec.extend_from_slice(&seq.to_be_bytes());
            vec.extend_from_slice(&sub_seq.to_be_bytes());
    
            let mut sm3_hash =  Sm3Hash::new(&vec);
            let digest = sm3_hash.get_hash();
    
            if digest < *CURVE_ORDER_SM2P256V1 && digest > *ZERO_ORDER_SM2P256V1 {
                let hash = digest.to_vec();

                let a_p = self.public_generator(&hash);

                let ecc_ctx = EccCtx::new();
                let a = ecc_ctx.bytes_to_point(&a_p).unwrap();

                let b = ecc_ctx.bytes_to_point(&public_generator).unwrap();

                let c = ecc_ctx.add(&a, &b);

                let public_key = ecc_ctx.point_to_bytes(&c, true);
                return public_key;
            }

            vec.clear();
            sub_seq += 1;
        }
    }

    fn human_readable_public_key(&self, public_key: &Vec<u8>) -> String {
        let mut sm3_hash =  Sm3Hash::new(&public_key);
        let digest = sm3_hash.get_hash();

        let mut input = [0u8; 32];
        input.copy_from_slice(&digest);

        let mut ripemd160 = JRipemd160::new();
        ripemd160.input(&input);
        let ret: &mut [u8] = &mut [0u8;20];
        ripemd160.result(ret);

        let prefix_public_key: Vec<u8> = PREFIX_PUBLIC_KEY.to_vec();

        let mut hash = Sm3Hash::new(&digest);
        let hash1 = hash.get_hash();
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();
    
        let checksum = digest.get(..4).unwrap().to_vec();

        //add
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&prefix_public_key);
        vec.extend_from_slice(&ret);
        vec.extend_from_slice(&checksum);

        BaseX::new(SKYWELL).encode(&vec)
    }
}