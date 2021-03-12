extern crate rand;
use rand::Rng;

// 16字节长度的随机数据
pub struct Brorand {}
impl Brorand {
    //random generate a specific length's [u8]
    pub fn brorand(len: usize) -> Vec<u8> {
        let u: Vec<u8> = (0..len).map(|_| {
            let idx: u8 = rand::thread_rng().gen();
            // let _hexs = format!("{:x}", idx);
            idx
        }).collect::<Vec<u8>>();

        u
    }
}
