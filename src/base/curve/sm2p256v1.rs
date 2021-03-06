use libsm::sm2::signature::{Pubkey, SigCtx};

use crate::base::curve::{entropy, scalar_sm2};
use hex;
use libsm::sm2::ecc::EccCtx;

static PRE_PRIVATE_KEY: &'static str = "00";
 
pub struct SM2P256V1 {}
impl SM2P256V1 {
    pub fn build_keypair_str(seed: &String) -> Result<(String, String), &'static str> {
        if let Some(seed) = entropy(seed) {
            let scalar = scalar_sm2(&seed, None);
            
            let ctx  = SigCtx::new();
            // if let Ok(secret_key) = SecretKey::from_slice(&scalar) {
                if let Ok(secret_key) = ctx.load_seckey(&scalar) {

                // let public_key = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
                let pk: Pubkey = ctx.pk_from_sk(&secret_key);
                let pk_raw: Vec<u8> = ctx.serialize_pubkey(&pk, true);
                
                let scalar_public_key = scalar_sm2(&pk_raw, Some(0));

                // if let Ok(mut secret_key_gen) = SecretKey::from_slice(&scalar_public_key) {
                if let Ok(secret_key_gen) = ctx.load_seckey(&scalar_public_key) {

                    // if let Ok(_) = secret_key_gen.add_assign(&secret_key[..]) {

                        // secret_key_gen = ctx.add(&secret_key);

                        let ecc_ctx = EccCtx::new();

                        let yy: Vec<u8> = ctx.serialize_seckey(&secret_key);
                        println!("yy: {:?}", yy);

                        let (pko, _secret) = ctx.new_keypair();

                        let mut yy1 = pko;
                        match ecc_ctx.bytes_to_point(&yy) {
                            Ok(o) => {
                                yy1 = o;
                            },
                            Err(e) => {
                                println!("yy1 eer: {:?}", e)
                            }

                        }


                        let xx:  Vec<u8> = ctx.serialize_seckey(&secret_key_gen);
                        println!("{:?}", xx);

                        let mut xx1 = pk;
                        match ecc_ctx.bytes_to_point(&xx) {

                            Ok(o) => {
                                xx1 = o;
                            },
                            Err(e) => {
                                println!("eer: {:?}", e)
                            }

                        }
                        // let xx1 = ecc_ctx.bytes_to_point(&xx).unwrap();

                        let secret_key_gen = ecc_ctx.add(&xx1, &yy1);

                        let skg_raw: Vec<u8> = ecc_ctx.point_to_bytes(&secret_key_gen, true);
                        println!("raw : {:?}", skg_raw);

                        let sparkle_heart = String::from_utf8_lossy(&skg_raw);
                        println!("spakle: {:?}", sparkle_heart);

                        //private_key
                        let private_key = PRE_PRIVATE_KEY.to_owned() + &sparkle_heart;
                        println!("private key: {:?}", private_key);

                        if let Ok(key_str) = hex::decode(&private_key) {

                            // if let Ok(secret_key) = SecretKey::from_slice(&key_str[1..]) {

                                let secret_key = ctx.load_seckey(&key_str).unwrap();
                                // let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
                                let public_gen = ctx.pk_from_sk(&secret_key);
                                let pk_raw = ctx.serialize_pubkey(&public_gen, true);


                                //public key
                                let public_key = hex::encode(pk_raw);

                                //so 33bytes = 33 * 8 = 264 = 32 * 8 + 8 = 256 + 8;
                                return Ok( ( private_key.to_ascii_uppercase(), public_key.to_ascii_uppercase() ) );

                            // } else {
                            //     return Err("32 bytes, within curve order");
                            // }

                        } else {
                            return Err("hex decode private_key error.");
                        }

                    // } else {
                    //     //.expect("32 bytes, within curve order");
                    //     return Err("32 bytes, within curve order");fff 
                    // }

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
