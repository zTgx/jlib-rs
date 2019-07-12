extern crate rand;
use rand::Rng;

pub struct Brorand {
}

impl Brorand {
    //随机生成一个指定长度的u8数组
    pub fn brorand(len: usize) -> Vec<u8> {
        let u: Vec<u8> = (0..len).map(|_| {
            let idx: u8 = rand::thread_rng().gen();
            
            let _hexs = format!("{:x}", idx);
            // println!("hexs : {}", hexs);

            idx
        }).collect::<Vec<u8>>();

        u
    }
}