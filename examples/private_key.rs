
use ring::{digest};
extern crate num;

extern crate secp256k1;
use secp256k1::key::{ SecretKey};
use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::key::ONE_KEY;
use secp256k1::constants::*;    

extern crate crypto;
use crypto::ripemd160::Ripemd160 ;
use crypto::digest::Digest;

extern crate mylib;
use mylib::base::*;

fn main() {

    let mut seed =  vec![ 27, 160, 140, 35, 48, 34, 206, 80, 166, 40, 137, 17, 158, 180, 155, 221 ];

    let private_gen = util::scalar_multiple(&seed, None);
    // println!("private gen : {:?}", private_gen);
    let secp = Secp256k1::new();
    let mut secret_key = SecretKey::from_slice(&private_gen).expect("32 bytes, within curve order");
    let mut public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
    // println!("public gen : {:?}", public_gen);

    //derivePrivateKey return
    //secp256k1.ScalarMultiple(publicGen.encodeCompressed(), 0).add(privateGen).mod(order);
    let public_gen_output = util::scalar_multiple(public_gen.as_slice(), Some(0));
    println!("before add : {:?}", public_gen_output);
    let mut secret_key2 = SecretKey::from_slice(&public_gen_output).expect("32 bytes, within curve order");
    let x = secret_key2.add_assign(&secret_key[..]);
    println!("x : {:?}", secret_key2);

    //var privateKey = prefix + derivePrivateKey(entropy).toString(16, 64).toUpperCase();
    let private_key = "00".to_owned() + secret_key2.to_string().as_str();
    // println!("private_key : {}", private_key.to_ascii_uppercase());

    //////////////public key
    if let Ok(keyx) = hex::decode(private_key) {
        let mut secret_key = SecretKey::from_slice(&keyx[1..]).expect("32 bytes, within curve order");
        let mut public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
        let public_key = hex::encode(public_gen);
        println!("public key : {:?}", public_key);
    
        ////////////////////////address
        if let Ok(key) = hex::decode(public_key) {
            println!("address key : {:?}", key);
            let mut ctx = digest::Context::new(&digest::SHA256);
            ctx.update(&key);
            let mut key = [0u8; 32];
            key.copy_from_slice(ctx.finish().as_ref());

            let input = key;

            println!("address input : {:?}", input);

            let mut ripemd160x = Ripemd160::new();
            ripemd160x.input(&input);

            let mut ret: &mut [u8] = &mut [0u8;20];
            ripemd160x.result(ret);
            println!("ripemd160x ret : {:?}",  ret);
            
            let ripemd160x= ripemd160x.result_str();
            println!("ripemd160x : {}", ripemd160x);

            if let Ok(args) = hex::decode(ripemd160x) {

                let mut version: Vec<u8> = [0].to_vec();

                //4. concat args
                util::concat_args(&mut version, &ret.to_vec());

                //5. encodechecked.
                let mut checked: Vec<u8> = util::encode_checked(&mut version);

                //6. concat args
                util::concat_args(&mut version, &checked);

                let address = util::encode_raw(&mut version);
                println!("address : {}", address);

            }
        }
    
    }
}