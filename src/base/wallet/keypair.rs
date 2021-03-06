use crate::base::curve::{
    secp256k1::J256k1,
    sm2p256v1::SM2P256V1
};
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

    pub fn build(&self) -> Result<Keypair, &'static str> {
        let (mut private_key, mut public_key) = ("".to_owned(), "".to_owned());
        match self.wtype {
            &WalletType::ED25519 => {
            },

            &WalletType::SECP256K1 => {
                if let Ok(x) = J256k1::build_keypair_str(&self.seed) {
                    private_key = x.0;
                    public_key = x.1;
                } else {
                    return Err("invalid seed, can't generate keypair.");
                }
            }
            &WalletType::SM2P256V1 => {
                println!("#############开始生成国密版本的 private key  / public key 对。");

                if let Ok(x) = SM2P256V1::build_keypair_str(&self.seed) {
                    private_key = x.0;
                    public_key  = x.1;
                } else {
                    return Err("invalid seed, can't generate keypair.");
                }
            }
        }

        //result
        Ok (
            Keypair {
                private_key: private_key,
                public_key : public_key,
            }
        )

    }
}
