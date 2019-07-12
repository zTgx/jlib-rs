    
    
extern crate secp256k1;

use secp256k1::key::{SecretKey, PublicKey};
// use secp256k1::constants;
use secp256k1::{Secp256k1, Message};
// use secp256k1::Error::{InvalidMessage, IncorrectSignature, InvalidSignature};

pub struct SignatureX {
}

impl SignatureX {
    /*
    @sign
    message: [u8]   /   message bytes needed to be sign.
    key    : [u8]   /   secret key bytes array.
    Output : signed hex string
    */
    pub fn sign(message: &[u8], key: &[u8]) -> String {
        let sign = Secp256k1::signing_only();
        let message = Message::from_slice(message).unwrap();
        let secret_key = SecretKey::from_slice(key).expect("32 bytes, within curve order");
        let signature = sign.sign(&message, &secret_key);
        // println!("tx_json signed : {:?}", signature.to_string().to_ascii_uppercase());   

        signature.to_string().to_ascii_uppercase()    
    }
    
    /*
    @verify
    message  : [u8]    /  raw message bytes.
    signature: [u8]    /  signed bytes array.
    Output   : bool    /  verify success or not.
    */
    pub fn verify(message: &[u8], signature: &[u8], key: &[u8]) -> bool {
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
}

