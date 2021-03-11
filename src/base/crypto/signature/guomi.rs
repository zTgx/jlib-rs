use libsm::sm3::hash::Sm3Hash; 
use hex;
use libsm::sm2::signature::{Pubkey, Seckey, Signature, SigCtx};

use crate::wallet::keypair::*;
use libsm::sm2::ecc::EccCtx;
use basex_rs::{BaseX, SKYWELL, Decode};

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
// impl <'a> SignatureI for SignatureX <'a> {
    /*
    @sign
    message: [u8]   /   message bytes needed to be sign.
    key    : [u8]   /   secret key bytes array.
    Output : signed hex string
    */
    pub fn sign(&self, message: &[u8]) -> String {

        let ctx = SigCtx::new();
        let private_key = hex::decode( &self.keypair.private_key ).unwrap();
        
        // let copy_pk = self.keypair.public_key.as_str().to_owned();
        // let public_key  = BaseX::new(SKYWELL).decode( copy_pk ).unwrap();
        
        let public_key = hex::decode( &self.keypair.public_key ).unwrap();
        // let public_key = hex::decode( "02DEA757E28F2E7456545AA2C17E2698BECABC884E66C79CA71C78EC240E0DC972").unwrap();
        let sk = Seckey::from_bytes_be(&private_key);

        let curve = EccCtx::new();
        println!("publick key: {:?}", public_key);
        let pk = curve.bytes_to_point(&public_key).unwrap();
        // let pk = ctx.load_pubkey(&public_key).unwrap();

        let signature = ctx.sign(message, &sk, &pk);

        {

            let b = ctx.verify(message, &pk, &signature);
            println!("验签：{}", b);
        }

        hex::encode_upper( signature.der_encode() ) //to_string().to_ascii_uppercase()
    }

    /*
    @verify
    message  : [u8]    /  raw message bytes.
    signature: [u8]    /  signed bytes array.
    Output   : bool    /  verify success or not.
    */
    pub fn verify(message: &[u8], signature: &[u8], key: &[u8]) -> bool {
        // let vrfy = Secp256k1::verification_only();
        // let sig: secp256k1::Signature = secp256k1::Signature::from_der(signature).expect("byte str decode");
        // let secp = Secp256k1::new();
        // let secret_key = SecretKey::from_slice(&key).expect("32 bytes, within curve order");
        // let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        // let msg = Message::from_slice(&message).unwrap();
        // if vrfy.verify(&msg, &sig, &public_key).is_ok() {
        //     return true;
        // }

        false
    }

    //Output Hex String
    pub fn sign_txn_signature(&self, so: &Vec<u8>) -> String {
        let mut digest = Vec::new();
        digest.extend_from_slice(&[83,84,88, 0]);
        digest.extend_from_slice(&so);
        let mut hash = Sm3Hash::new(digest.as_slice());
        let hash1 = hash.get_hash();

        let hash = hex::encode( &hash1 );
        let message = hash.get(0..64).unwrap().to_ascii_uppercase();

        let msg = hex::decode(message).unwrap();
        let signed_hex_string = self.sign(&msg);

        return signed_hex_string;
    }
}
