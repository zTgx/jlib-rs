use crate::base::curve::secp256k1::J256k1;
use crate::WalletType;

#[derive(Debug, Clone)]
pub struct Keypair {
    pub private_key: String, //hex string
    pub public_key: String,  //hex string
}

impl Keypair {
    pub fn new(private_key: String, public_key: String) -> Self {
        Keypair {
            private_key: private_key,
            public_key: public_key,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeypairBuilder <'a> {
    pub seed: &'a String,
    pub wtype: &'a WalletType,
}

impl <'a> KeypairBuilder <'a> {
    pub fn new(seed: &'a String, wtype: &'a WalletType) -> Self {
        KeypairBuilder {
            seed: seed,
            wtype: wtype,
        }
    }

    pub fn build(&self) -> Keypair {
        let (mut private_key, mut public_key) = ("".to_owned(), "".to_owned());
        match self.wtype {
            &WalletType::ED25519 => {
            },

            &WalletType::SECP256K1 => {
                let (left, right) = J256k1::build_keypair_str(&self.seed);

                private_key = left;
                public_key = right;
            }
        }

        Keypair {
            private_key: private_key,
            public_key : public_key,
        }
    }

    // fn generate(&self, seed: &String) -> Keypair {
        // let seed = entropy(seed);
        // // println!("seed: {:?}", seed);
        // let private_gen = scalar_multiple(&seed, None);
        // let secp = Secp256k1::new();
        // let secret_key = SecretKey::from_slice(&private_gen).expect("32 bytes, within curve order");
        // // println!("secret_key: {:?}", secret_key);
        // let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
        // // println!("public_gen: {:?}", public_gen);
        //
        // let public_gen_output = scalar_multiple(public_gen.as_slice(), Some(0));
        // // println!("before add : {:?}", public_gen_output);
        // let mut secret_key2 = SecretKey::from_slice(&public_gen_output).expect("32 bytes, within curve order");
        // secret_key2.add_assign(&secret_key[..]).unwrap();
        //
        // let private_key = "00".to_owned() + secret_key2.to_string().as_str();
        //
        // //////////////public key
        // let mut xy = "oo".to_string();
        // if let Ok(keyx) = hex::decode(&private_key) {
        //     let secret_key = SecretKey::from_slice(&keyx[1..]).expect("32 bytes, within curve order");
        //     let public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
        //     let public_key = hex::encode(public_gen);
        //     xy = public_key;
        // }
        //
        // Keypair {
        //     private_key: private_key.to_ascii_uppercase(),
        //     public_key: xy.to_ascii_uppercase(),
        // }
    // }
}
