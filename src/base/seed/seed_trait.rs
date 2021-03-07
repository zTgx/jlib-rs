/*
trait SeedI 接口说明
* generate_masterphrase
用来生成seed所需要的种子， 输出为masterphrase。

* human_readable_seed
生成十六进制的seed

* checksum
生成checksum。

* is_valid
验证seed是否有效
*/

pub trait SeedI {
    fn generate_masterphrase(&self, passphrase: Option<&str>) -> Vec<u8>;
    fn human_readable_seed(&self, seed: &Vec<u8>) -> String;
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8>;
    fn is_valid(&self, readable_seed: &String) -> bool;
}
 
// master_seed
// humanSeed()

// // master_seed_hex
// getSeed