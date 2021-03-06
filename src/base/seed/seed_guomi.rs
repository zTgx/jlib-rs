#![allow(unused)]
use crate::base::seed::seed_trait::SeedI;
use libsm::sm3::hash::Sm3Hash; 
use basex_rs::{BaseX, SKYWELL, Encode};
use crate::base::misc::brorand::Brorand;

static PREFIX_SEED  : [u8; 1] = [0x21];
static PHRASE_LENGTH: usize   = 16;

pub struct SeedGuomi {
    seed_prefix: [u8; 1],
    phrase_length: usize
}
impl SeedGuomi {
    pub fn new() -> Self {
        SeedGuomi {
            seed_prefix  : PREFIX_SEED,
            phrase_length: PHRASE_LENGTH
        }
    }
}
impl SeedI for SeedGuomi {
    /*
    生成seed算法：
    sm3(passphrase) 的结果取128位。
    */
    fn generate_masterphrase(&self, passphrase: Option<&str>) -> Vec<u8> {
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

        println!("masterphrase: {:?}", seed);
        return seed.to_vec();
    }

    /*
    算法说明：
    第一步： SEEDPREFIX(0x21) 追加到 seed 前，共17字节，对该数据进行 sm3计算出 hash。
    第二步： 计算checksum。
    第三步： SEEDPREFIX(0x21)，seed，checksum合并, 做base58计算。
    第四步： 结果为29字节字符串。（如：shQRzBzq9akA2C2o4MKt1fM51WWs9）
    */
    fn human_readable_seed(&self, seed: &Vec<u8>) -> String {
        //第一步
        let mut prefix_and_seed = Vec::new();
        prefix_and_seed.extend(&self.seed_prefix);
        prefix_and_seed.extend(seed);
    
        //第二步
        let checksum = self.checksum(&prefix_and_seed);

        //第三步
        let mut target = Vec::new();
        target.extend(&self.seed_prefix);  // 0x21
        target.extend(seed);         // seed
        target.extend(checksum);     // checksum
    
        //第四步
        BaseX::new(SKYWELL).encode(target.as_mut_slice())
    }

    /*
    计算checksum
    输入参数： PREFIX_SEED + seed
    步骤： 连续2次sm3操作， 取前四个字节， 作为checksum。
    */
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8> {
        let mut hash = Sm3Hash::new(digest.as_slice());
        let hash1 = hash.get_hash();
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();
    
        let checksum = digest.get(..4).unwrap().to_vec();
        return checksum;
    }

    fn is_valid(&self, _readable_seed: &String) -> bool {
        true
    }
}