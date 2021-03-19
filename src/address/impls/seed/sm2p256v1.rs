#![allow(unused)]
use libsm::sm3::hash::Sm3Hash; 
use basex_rs::{BaseX, SKYWELL, Encode};
use crate::base::crypto::brorand::Brorand;
use crate::address::traits::seed::SeedI;

use crate::address::traits::checksum::{ChecksumI};
use crate::address::impls::checksum::sm2p256v1::ChecksumSM2P256V1;

use crate::address::constants::VersionEncoding;
use crate::address::constants::PASS_PHRASE_LENGTH;
use rfc1751::ToRfc1751;

pub struct SeedSM2P256V1 {
}

impl SeedSM2P256V1 {
    pub fn new() -> Self {
        SeedSM2P256V1 {
        }
    }
}

impl SeedI for SeedSM2P256V1 {
    fn get_seed(&self, passphrase: Option<&str>) -> Vec<u8> {
        // TOOD: warning: value assigned to `phrase_bytes` is never read
        let mut phrase_bytes: Vec<u8> = vec![0; 16];
        if let Some(phrase) = passphrase  {
            // 使用指定的passphrase作为种子， 生成seed
            phrase_bytes = phrase.as_bytes().to_vec();
        } else {
            // 使用16字节的随机数作为种子， 生成seed
            phrase_bytes = Brorand::brorand(PASS_PHRASE_LENGTH);
        }
        
        let mut hash = Sm3Hash::new(&phrase_bytes);
        let digest: [u8;32] = hash.get_hash();
        let seed: &[u8] = &digest[..16];

        return seed.to_vec();
    }

    fn human_seed(&self, seed: &Vec<u8>) -> String {
        //第一步
        let mut prefix_and_seed = Vec::new();
        prefix_and_seed.extend(&(VersionEncoding::VerFamilySeed as u8).to_be_bytes());
        prefix_and_seed.extend(seed);
    
        //第二步
        let checksum = ChecksumSM2P256V1::new().checksum(&prefix_and_seed);

        //第三步
        let mut target = Vec::new();
        target.extend(&(VersionEncoding::VerFamilySeed as u8).to_be_bytes());  // 0x21
        target.extend(seed);         // seed
        target.extend(checksum);     // checksum
    
        //第四步
        BaseX::new(SKYWELL).encode(target.as_mut_slice())
    }

    fn human_seed_rfc1751(&self, seed: &Vec<u8>) -> String {
        let human_seed_rfc1751 = seed.to_rfc1751().unwrap();
        return human_seed_rfc1751;
    }
}