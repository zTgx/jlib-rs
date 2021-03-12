use crate::wallet::wallet::WalletType;
use crate::base::crypto::signature::traits::signature::SignatureI;
use crate::base::crypto::signature::guomi::SignatureGuomi;
use crate::wallet::keypair::*;

use crate::base::crypto::signature::sign::SignatureX;

pub struct SignatureBuilder {
    signature: Box<dyn SignatureI>,
}
 
impl SignatureBuilder {
    pub fn new(key_type: WalletType, keypair: Keypair) -> Self {
        let signature = SignatureBuilder::build(key_type, keypair);

        SignatureBuilder {
            signature: signature,
        }
    }

    fn build(key_type: WalletType, keypair: Keypair) -> Box<dyn SignatureI> {
        match key_type {
            WalletType::ED25519 => {
                return Box::new(SignatureGuomi::new(keypair));
            },
            WalletType::SECP256K1 => {
                return Box::new(SignatureX::new(keypair));
            },
            WalletType::SM2P256V1 => {
                return Box::new(SignatureGuomi::new(keypair));
            }
        } 
    }
}

impl SignatureI for SignatureBuilder {
    fn sign(&self, message: &[u8]) -> String {
        self.signature.sign(&message)
    }

    fn verify(&self, message: &[u8], signature: &[u8], key: &[u8]) -> bool {
        self.signature.verify(&message, &signature, &key)
    }

    fn sign_txn_signature(&self, so: &Vec<u8>) -> String {
        self.signature.sign_txn_signature(&so)
    }
}
