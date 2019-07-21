
extern crate hex;
extern crate jlib;

use jlib::base::sign::*;
use jlib::base::util::*;
use jlib::base::brorand::*;

//ed25519_dalek
extern crate rand;
use rand::Rng;
use rand::OsRng;
extern crate sha2;
extern crate ed25519_dalek;
use sha2::Sha512;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use ed25519_dalek::PublicKey;


fn main () {
    let message = "11D06DFD3CDC4D8FE00214879A97B3E4B40B75F8DF82D89ECE72A96066A52583";
    let key = [ 154, 130, 174, 40, 14, 141, 200, 71, 58, 158, 235, 154, 10, 79, 66, 243,
                85, 228, 136, 132, 102, 248, 112, 40, 17,  16, 1, 139, 57, 145, 205, 182 ];

    if let Ok(msg) = hex::decode(message) {
        let signed_hex_string = SignatureX::sign(&msg, &key);
        // println!("signed_hex_string: {}", signed_hex_string);

        //verify
        if let Ok(sig) = hex::decode(signed_hex_string) {
            let _ret = SignatureX::verify(&msg, &sig, &key);
            // println!("verify result : {}", ret);
        }
    }

    let u: Vec<u8> = Brorand::brorand(64);
    println!("scalar: {:?}", &u);

    //sign
    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut csprng);
    println!("key pair: {:?}", keypair);

    let message: &[u8] = "This is a test of the tsunami alert system.".as_bytes();
    let signature: Signature = keypair.sign::<Sha512>(message);
    println!("signature: {:?}", signature);

    //verify
    let public_key: PublicKey = keypair.public;
    if public_key.verify::<Sha512>(message, &signature).is_ok() {
        println!("ok.");
    } else {
        println!("err.");
    }

}
