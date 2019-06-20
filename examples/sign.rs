    
    
    extern crate secp256k1;

    use secp256k1::key::{SecretKey, PublicKey};
    use secp256k1::constants;
    use secp256k1::{Secp256k1, Signature, Message};
    use secp256k1::Error::{InvalidMessage, IncorrectSignature, InvalidSignature};

fn main () {
    let sign = Secp256k1::signing_only();
    // let vrfy = Secp256k1::verification_only();
    // let full = Secp256k1::new();

    let mut key = [
  154,
  130,
  174,
  40,
  14,
  141,
  200,
  71,
  58,
  158,
  235,
  154,
  10,
  79,
  66,
  243,
  85,
  228,
  136,
  132,
  102,
  248,
  112,
  40,
  17,
  16,
  1,
  139,
  57,
  145,
  205,
  182 ];

    

    // Try key generation
    // let (sk, pk) = full.generate_keypair(&mut thread_rng());

    // Try signing
    if let Ok(message) = hex::decode("11D06DFD3CDC4D8FE00214879A97B3E4B40B75F8DF82D89ECE72A96066A52583") {
        println!("message : {:?}", message);
        let message = Message::from_slice(&message).unwrap();

        let mut secret_key = SecretKey::from_slice(&key).expect("32 bytes, within curve order");

        let signature = sign.sign(&message, &secret_key);
        println!("tx_json signed : {:?}", signature.to_string().to_ascii_uppercase());
    };

    // let sig = full.sign(&msg, &sk);

    // Try verifying
    // assert!(vrfy.verify(&msg, &sig, &pk).is_ok());
    // assert!(full.verify(&msg, &sig, &pk).is_ok());
}