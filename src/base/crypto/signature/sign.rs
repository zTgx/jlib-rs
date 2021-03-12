use secp256k1::key::{SecretKey, PublicKey};
use secp256k1::{Secp256k1, Message};

use ring::{digest};
use hex;
use crate::base::crypto::signature::traits::signature::SignatureI;

use crate::wallet::keypair::*;

pub struct SignatureX {
    pub keypair: Keypair,
}

impl SignatureX {
    pub fn new(keypair: Keypair) -> Self {
        SignatureX {
            keypair: keypair,
        }
    }
}


impl SignatureI for SignatureX {
    fn sign(&self, message: &[u8]) -> String {
        let sign = Secp256k1::signing_only();
        let message = Message::from_slice(message).unwrap();
        
        let secret_key = SecretKey::from_slice(self.keypair.private_key.as_bytes()).expect("32 bytes, within curve order");
        let signature = sign.sign(&message, &secret_key);

        signature.to_string().to_ascii_uppercase()
    }

    fn verify(&self, message: &[u8], signature: &[u8], key: &[u8]) -> bool {
        let vrfy = Secp256k1::verification_only();
        let sig: secp256k1::Signature = secp256k1::Signature::from_der(signature).expect("byte str decode");
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&key).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let msg = Message::from_slice(&message).unwrap();
        if vrfy.verify(&msg, &sig, &public_key).is_ok() {
            return true;
        }

        false
    }

    //Output Hex String
    fn sign_txn_signature(&self, so: &Vec<u8>) -> String {
        let mut ctx = digest::Context::new(&digest::SHA512);
        ctx.update(&[83,84,88, 0]);
        ctx.update(&so);

        let hash = hex::encode(&ctx.finish().as_ref());
        let message = hash.get(0..64).unwrap().to_ascii_uppercase();
        // let private_key = &self.keypair.private_key;
        // let key = &hex::decode(private_key).unwrap()[1..];

        let msg = hex::decode(message).unwrap();
        let signed_hex_string = self.sign(&msg);

        return signed_hex_string;
    }
}

// impl SignatureI for SignatureX {
//     fn sign(&self, message: &[u8], key: &[u8]) -> String {
//         let sign = Secp256k1::signing_only();
//         let message = Message::from_slice(message).unwrap();
        
//         let secret_key = SecretKey::from_slice(key).expect("32 bytes, within curve order");
//         let signature = sign.sign(&message, &secret_key);

//         signature.to_string().to_ascii_uppercase()
//     }

//     fn verify(&self, message: &[u8], signature: &[u8], key: &[u8]) -> bool {
//         let vrfy = Secp256k1::verification_only();
//         let sig: secp256k1::Signature = secp256k1::Signature::from_der(signature).expect("byte str decode");
//         let secp = Secp256k1::new();
//         let secret_key = SecretKey::from_slice(&key).expect("32 bytes, within curve order");
//         let public_key = PublicKey::from_secret_key(&secp, &secret_key);
//         let msg = Message::from_slice(&message).unwrap();
//         if vrfy.verify(&msg, &sig, &public_key).is_ok() {
//             return true;
//         }

//         false
//     }

//     //Output Hex String
//     fn sign_txn_signature(&self, so: &Vec<u8>) -> String {
//         let mut ctx = digest::Context::new(&digest::SHA512);
//         ctx.update(&[83,84,88, 0]);
//         ctx.update(&so);

//         let hash = hex::encode(&ctx.finish().as_ref());
//         let message = hash.get(0..64).unwrap().to_ascii_uppercase();
//         let private_key = &self.keypair.private_key;
//         let key = &hex::decode(private_key).unwrap()[1..];

//         let msg = hex::decode(message).unwrap();
//         let signed_hex_string = self.sign(&msg, &key);

//         return signed_hex_string;
//     }
// }
