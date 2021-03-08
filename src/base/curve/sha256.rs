use ring::{digest};

pub struct JSha256 {}

impl JSha256 {
    pub fn sha256(so:  &Vec<u8>) -> Vec<u8> {
        let v8: &[u8] = so.as_slice();

        let ac      = digest::digest(&digest::SHA256, v8);
        let checked = digest::digest(&digest::SHA256, &ac.as_ref());

        let ret: Vec<u8> = checked.as_ref().iter().map(|c| {
            let x = format!("{:x}", c);
            x.as_str().chars().nth(0).unwrap() as u8
        }).collect::<Vec<u8>>();

        ret
    }
}
