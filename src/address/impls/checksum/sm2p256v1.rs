use libsm::sm3::hash::Sm3Hash; 
use crate::address::traits::checksum::ChecksumI;

pub struct ChecksumSM2P256V1 {}
impl ChecksumSM2P256V1 {
    pub fn new() -> Self {
        ChecksumSM2P256V1 {
            
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// 
// 国密 sm3 算法的 checksum
// 
// ---------------------------------------------------------------------------------------------------------
impl ChecksumI for ChecksumSM2P256V1 {
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8> {
        // 第一次 hash 计算
        let mut hash = Sm3Hash::new(digest.as_slice());
        let hash1 = hash.get_hash();

        // 第二次hash计算
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();

        // 取前四个字节
        digest.get(..4).unwrap().to_vec()
    }
}