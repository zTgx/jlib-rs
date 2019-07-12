

extern crate bs58;

// use crate::base::constants::ALPHABET;
use crate::base::*;

extern crate secp256k1;
use secp256k1::key::{ SecretKey};
use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;
// use secp256k1::key::ONE_KEY;
// use secp256k1::constants::*; 

use crate::base::seed::*;

#[derive(Debug)]
pub struct KeypairProperty {
    pub secret_key: String, //hex string 私钥
    pub public_key: String, //hex string 公钥
}

impl KeypairProperty {
    pub fn new(secret_key: String, public_key: String) -> Self {
        KeypairProperty {
            secret_key: secret_key,
            public_key: public_key,
        }
    }

    //to hex
    // pub fn to_bytes() -> Vec<u8>
}
impl Clone for KeypairProperty {
    fn clone(&self) -> KeypairProperty {
        match self {
            _ =>
            KeypairProperty {
                secret_key: self.secret_key.to_owned(),
              
                public_key: self.public_key.to_owned(),   
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Keypair {
    pub property: KeypairProperty,
}

impl Keypair {
    pub fn new(property: KeypairProperty) -> Self {
        Keypair {
            property: property,
        }
    }

    //some util method.
}

#[derive(Debug, Clone)]
pub struct KeypairBuilder <'a> {
    pub seed: &'a Seed,
}

impl <'a> KeypairBuilder <'a> {
    pub fn new(seed: &'a Seed) -> Self {
        KeypairBuilder {
            seed: seed,
        }
    }

    pub fn build(&self) -> Keypair {
        // let seed = "sh7xcXLQmWYk2eV5M3nSGVU8KqX9i".to_string();
        let seed = &self.seed.seed_property.seed;
        let x = self.generate(seed);

        Keypair {
            property: x,
        }
    }

    fn generate(&self, seed: &String) -> KeypairProperty {
        let seed = util::entropy(seed);

        let private_gen = util::scalar_multiple(&seed, None);
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&private_gen).expect("32 bytes, within curve order");
        let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();

        let public_gen_output = util::scalar_multiple(public_gen.as_slice(), Some(0));
        let secret_key2 = SecretKey::from_slice(&public_gen_output).expect("32 bytes, within curve order");
        // let x = secret_key2.add_assign(&secret_key[..]);

        let private_key = "00".to_owned() + secret_key2.to_string().as_str();

        //////////////public key
        let mut xy = "oo".to_string();
        if let Ok(keyx) = hex::decode(&private_key) {
            let secret_key = SecretKey::from_slice(&keyx[1..]).expect("32 bytes, within curve order");
            let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
            let public_key = hex::encode(public_gen);
            xy = public_key;
        }

        KeypairProperty {
            secret_key: private_key.to_ascii_uppercase(),
            public_key: xy.to_ascii_uppercase(),
        }
    }
}

