use crate::address::traits::checksum::{ChecksumI};
use crate::address::impls::checksum::sm2p256v1::ChecksumSM2P256V1;

use basex_rs::{BaseX, SKYWELL, Encode}; 
use crate::address::traits::address::AddressI;
use crate::address::constants::VersionEncoding;

use crate::address::traits::generator::GeneratorI;
use crate::address::impls::generator::sm2p256v1::GeneratorSM2P256V1;

pub struct AddressSM2P256V1 {
    seed: Vec<u8>,
    generator: GeneratorSM2P256V1,
}
impl AddressSM2P256V1 {
    pub fn new(seed: &Vec<u8>) -> Self {
        AddressSM2P256V1 {
            seed: seed.to_vec(),
            generator: GeneratorSM2P256V1::new(),
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// 实现 trait AddressI
// ---------------------------------------------------------------------------------------------------------
impl AddressI for AddressSM2P256V1 {
    // account id
    fn human_account_id(&self) -> String {
        let private_generator   = self.generator.private_generator(&self.seed);
        let public_generator    = self.generator.public_generator(&private_generator);
        println!("public_generator: {:?}", public_generator);
        
        let public_key          = self.generator.generate_public_key(&public_generator);
        // println!("第一遍public_key: {:?}", public_key);
        // println!("public_key_hex : {:?}", hex::encode(&public_key));

        let account_id          = self.generator.human_readable_public_key(&public_key);

        account_id
    }

    // public_key
    fn public_key(&self) -> String {
        let private_generator   = self.generator.private_generator(&self.seed);
        let public_generator    = self.generator.public_generator(&private_generator);
        let public_key          = self.generator.generate_public_key(&public_generator);
        // println!("第3遍public_key: {:?}", public_key);
        // println!("public_key_hex : {:?}", hex::encode(&public_key));

        let mut vec = Vec::new();
        vec.extend_from_slice(&(VersionEncoding::VerAccountPublic as u8).to_be_bytes());
        vec.extend_from_slice(&public_key);
    
        let checksum = ChecksumSM2P256V1::new().checksum(&vec);

        //add: 0 + 20 + 4
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&(VersionEncoding::VerAccountPublic as u8).to_be_bytes());
        vec.extend_from_slice(&public_key);
        vec.extend_from_slice(&checksum);

        //public key。
        let public_key = BaseX::new(SKYWELL).encode(&vec);

        public_key
    }

    // public_key_hex
    fn public_key_hex(&self) -> String {
        let private_generator   = self.generator.private_generator(&self.seed);
        let public_generator    = self.generator.public_generator(&private_generator);
        let public_key          = self.generator.generate_public_key(&public_generator);
        // println!("第4遍public_key: {:?}", public_key);
        // println!("public_key_hex : {:?}", hex::encode(&public_key));

        hex::encode_upper(public_key)
    }

    // private key
    fn private_key(&self) -> String {
        let private_generator = self.generator.private_generator(&self.seed);
        let public_generator = self.generator.public_generator(&private_generator);
        let generate_private_key = self.generator.generate_private_key(&private_generator, &public_generator);

        hex::encode(generate_private_key)
    }
}

// ---------------------------------------------------------------------------------------------------------
//
// 测试用例
//
// ---------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #[test]
    fn guomi_public_key() {
        assert_eq!(2 + 2, 4);
    }
}