use crate::base::secp256k1::key::{SecretKey, PublicKey};
use crate::base::secp256k1::{Secp256k1, Message};

use crate::base::ring::{digest};
use cast_rs::hex_t;
use crate::base::wallet::keypair::*;

pub struct SignatureX <'a> {
    pub keypair: &'a Keypair,
}

impl <'a> SignatureX <'a> {
    pub fn new(keypair: &'a Keypair) -> Self {
        SignatureX {
            keypair: keypair,
        }
    }
}

impl <'a> SignatureX <'a> {
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

    //Output Hex String
    pub fn sign_txn_signature(&self, so: &Vec<u8>) -> String {
        let mut ctx = digest::Context::new(&digest::SHA512);
        ctx.update(&[83,84,88, 0]);
        ctx.update(&so);

        let hash = hex_t::encode(&ctx.finish().as_ref());
        println!("hash : {}", hash);
        let message = hash.get(0..64).unwrap().to_ascii_uppercase();
        println!("message: {:?}", message);
        let private_key = &self.keypair.property.secret_key;
//        let private_key = util::get_keypair_from_secret(&self.secret).property.secret_key;
        let key = &hex_t::decode(private_key).unwrap()[1..];

        let msg = hex_t::decode(message).unwrap();
        let signed_hex_string = SignatureX::sign(&msg, &key);

        return signed_hex_string;
    }
}
