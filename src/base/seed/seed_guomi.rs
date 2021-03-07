#![allow(unused)]
use libsm::sm3::hash::Sm3Hash; 
use basex_rs::{BaseX, SKYWELL, Encode};
use crate::base::misc::brorand::Brorand;
use crate::base::seed::seed_trait::SeedI;
use hex;
use crate::base::crypto::traits::checksum::ChecksumI;
use crate::base::address::constants::VersionEncoding;

// ----------------------------------------------------------------
// 生成国密版本seed需要的常量
static PHRASE_LENGTH: usize   = 16;
// ----------------------------------------------------------------

pub struct SeedGuomi {
    phrase_length: usize
}

impl SeedGuomi {
    pub fn new() -> Self {
        SeedGuomi {
            phrase_length: PHRASE_LENGTH
        }
    }
}

impl SeedI for SeedGuomi {
    fn get_seed(&self, passphrase: Option<&str>) -> Vec<u8> {
        // TOOD: warning: value assigned to `phrase_bytes` is never read
        let mut phrase_bytes: Vec<u8> = vec![0; 16];
        if let Some(phrase) = passphrase  {
            // 使用指定的passphrase作为种子， 生成seed
            phrase_bytes = phrase.as_bytes().to_vec();
        } else {
            // 使用16字节的随机数作为种子， 生成seed
            phrase_bytes = Brorand::brorand(self.phrase_length);
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
        let checksum = self.checksum(&prefix_and_seed);

        //第三步
        let mut target = Vec::new();
        target.extend(&(VersionEncoding::VerFamilySeed as u8).to_be_bytes());  // 0x21
        target.extend(seed);         // seed
        target.extend(checksum);     // checksum
    
        //第四步
        BaseX::new(SKYWELL).encode(target.as_mut_slice())
    }

    fn is_valid(&self, _readable_seed: &String) -> bool {
        true
    }
}

impl ChecksumI for SeedGuomi {
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8> {
        let mut hash = Sm3Hash::new(digest.as_slice());
        let hash1 = hash.get_hash();
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();

        let checksum = digest.get(..4).unwrap().to_vec();
        return checksum;
    }
}