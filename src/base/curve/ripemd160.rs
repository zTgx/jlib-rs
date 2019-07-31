use crate::base::crypto::ripemd160::Ripemd160 ;
use crate::base::crypto::digest::Digest;

pub struct JRipemd160 {
    pub jripemd160x: Ripemd160,
}

impl JRipemd160 {
    pub fn new() -> Self {
        JRipemd160 {
            jripemd160x: Ripemd160::new(),
        }
    }

    pub fn input(&mut self, input: &[u8]) {
        self.jripemd160x.input(&input);
    }

    pub fn result(&mut self, ret: &mut [u8]) {
        self.jripemd160x.result(ret);
    }

    pub fn result_str(&mut self) -> String {
         self.jripemd160x.result_str()
    }
}
