use secp256k1::key::{ SecretKey, PublicKey };
use secp256k1::Secp256k1;
use crate::base::curve::{entropy, scalar_multiple};
use hex;

//TODO::需要进行重构！！！
static PRE_PRIVATE_KEY: &'static str = "00";

pub struct J256k1 {}
impl J256k1 {
    pub fn build_keypair_str(seed: &String) -> Result<(String, String), &'static str> {
        if let Some(seed) = entropy(seed) {
            let scalar = scalar_multiple(&seed, None);

            let secp = Secp256k1::new();
            if let Ok(secret_key) = SecretKey::from_slice(&scalar) {
                let public_key = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();

                let scalar_public_key = scalar_multiple(public_key.as_slice(), Some(0));
                if let Ok(mut secret_key_gen) = SecretKey::from_slice(&scalar_public_key) {
                    if let Ok(_) = secret_key_gen.add_assign(&secret_key[..]) {

                        //private_key
                        let private_key = PRE_PRIVATE_KEY.to_owned() + secret_key_gen.to_string().as_str();

                        if let Ok(key_str) = hex::decode(&private_key) {

                            if let Ok(secret_key) = SecretKey::from_slice(&key_str[1..]) {

                                let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();

                                //public key
                                let public_key = hex::encode(public_gen);

                                //so 33bytes = 33 * 8 = 264 = 32 * 8 + 8 = 256 + 8;
                                return Ok( ( private_key.to_ascii_uppercase(), public_key.to_ascii_uppercase() ) );

                            } else {
                                return Err("32 bytes, within curve order");
                            }

                        } else {
                            return Err("hex decode private_key error.");
                        }

                    } else {
                        //.expect("32 bytes, within curve order");
                        return Err("32 bytes, within curve order");
                    }

                } else {
                    //.expect("32 bytes, within curve order");
                    return Err("32 bytes, within curve order");
                }


            } else {
                //.expect("32 bytes, within curve order");
                return Err("32 bytes, within curve order");
            }
        } else {
            return Err("entropy error.");
        }
    }
}
