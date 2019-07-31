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
}
