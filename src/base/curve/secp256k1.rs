use crate::base::secp256k1::key::{ SecretKey, PublicKey };
use crate::base::secp256k1::Secp256k1;
use crate::base::curve::{entropy, scalar_multiple};


pub struct J256k1 {}
impl J256k1 {
    pub fn build_keypair_str(seed: &String) -> (String, String) {
        let seed = entropy(seed);
        // println!("seed: {:?}", seed);
        let private_gen = scalar_multiple(&seed, None);
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&private_gen).expect("32 bytes, within curve order");
        // println!("secret_key: {:?}", secret_key);
        let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
        // println!("public_gen: {:?}", public_gen);

        let public_gen_output = scalar_multiple(public_gen.as_slice(), Some(0));
        // println!("before add : {:?}", public_gen_output);
        let mut secret_key2 = SecretKey::from_slice(&public_gen_output).expect("32 bytes, within curve order");
        secret_key2.add_assign(&secret_key[..]).unwrap();

        let private_key = "00".to_owned() + secret_key2.to_string().as_str();

        //////////////public key
        let mut xy = "oo".to_string();
        if let Ok(keyx) = hex::decode(&private_key) {
            let secret_key = SecretKey::from_slice(&keyx[1..]).expect("32 bytes, within curve order");
            let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
            let public_key = hex::encode(public_gen);
            xy = public_key;
        }

        ( private_key.to_ascii_uppercase(), xy.to_ascii_uppercase() )
    }
}
