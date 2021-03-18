use libsm::sm3::hash::Sm3Hash; 

// ---------------------------------------------------------------------------------------------------------
// ChecksumGuomiI trait
// 
// 国密 sm3 算法的 checksum
// 
// ---------------------------------------------------------------------------------------------------------
pub trait ChecksumGuomiI {
    /*
        需要digest

        【计算过程】
        1、对digest进行第一次 【摘要计算】，结果为hash1。
        2、对hash1进行一次 【摘要计算】，结果为hash2。
        3、取hash2的前四个字节，即为checksum的结果。

        【输出】
        四个字节的数组
    */
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

