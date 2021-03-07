/*
    需要digest
    1、对digest进行第一次 【摘要计算】，结果为hash1。
    2、对hash1进行一次 【摘要计算】，结果为hash2。
    3、取hash2的前四个字节，即为checksum的结果。
*/
pub trait ChecksumI {
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8>;
}
