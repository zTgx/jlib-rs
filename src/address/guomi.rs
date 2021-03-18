use crate::base::crypto::traits::checksum::ChecksumI;
use libsm::sm3::hash::Sm3Hash; 
use basex_rs::{BaseX, SKYWELL, Encode}; 
use crate::base::crypto::traits::generator::GeneratorI;
use crate::address::traits::address::AddressI;
use crate::address::constants::VersionEncoding;

use crate::address::impls::guomi::GeneratorGuomi;

pub struct AddressGuomi {
    seed: Vec<u8>,
    generator: GeneratorGuomi,
}
impl AddressGuomi {
    pub fn new(seed: &Vec<u8>) -> Self {
        AddressGuomi {
            seed: seed.to_vec(),
            generator: GeneratorGuomi::new(),
        }
    }
}

impl AddressI for AddressGuomi {
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
    
        let checksum = self.checksum(&vec);

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

impl ChecksumI for AddressGuomi {
    fn checksum(&self, digest: &Vec<u8>) -> Vec<u8> {
        let mut hash = Sm3Hash::new(digest.as_slice());
        let hash1 = hash.get_hash();
        let mut hash2 = Sm3Hash::new(&hash1);
        let digest = hash2.get_hash();

        let checksum = digest.get(..4).unwrap().to_vec();
        return checksum;
    }
}

