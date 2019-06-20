
use ring::{digest};
extern crate num;

extern crate secp256k1;
use secp256k1::key::{ SecretKey};
use secp256k1::key::PublicKey;
use secp256k1::Secp256k1;
use secp256k1::key::ONE_KEY;
use secp256k1::constants::*;      

fn scalar_multiple(bytes: &[u8], discrim: Option<u8>) -> Vec<u8> {
    let mut i = 0u32;
    while i <= 0xFFFFFFFF  {
        // We hash the bytes to find a 256 bit number, looping until we are sure it
        // is less than the order of the curve.
        let mut ctx = digest::Context::new(&digest::SHA512);
        ctx.update(&bytes);
        if let Some(x) = discrim {
            //as i32
            ctx.update(&(x as i32).to_be_bytes());
        }
        ctx.update(&i.to_be_bytes());

        let mut key = [0u8; 64];
        key.copy_from_slice(ctx.finish().as_ref());
        for x in key.iter() {
            println!("{}", x );
        }
        let mut key = key.to_vec();
        key.truncate(32);
        
        // let finish = ctx.finish();
        // let xx: String = finish.as_ref().iter().map(|c| {
        //     let x = format!("{:x}", c);
        //     x 
        // }).collect();
        // let key = xx.get(0..32).unwrap().to_string();

        if key.as_slice() < &CURVE_ORDER && key.as_slice() > &0i32.to_be_bytes() {

            println!("scalar key : {:?}", key);
            // let mut key = key.to_vec();
            // key.truncate(32);
            return key;
        }

        i += 1;
    } // end while

    //never get this
    vec![0]
}

fn main() {

    // let secret_key = SecretKey::from_slice("ssLGemsXGu9rKnhnotfhT2bbfvMYY".to_string().as_bytes()).expect("32 bytes, within curve order");
    // println!("secret key : {:?}", secret_key);

    let mut seed =  vec![ 27, 160, 140, 35, 48, 34, 206, 80, 166, 40, 137, 17, 158, 180, 155, 221 ];

    let private_gen = scalar_multiple(&seed, None);
    // println!("private gen : {:?}", private_gen);
    let secp = Secp256k1::new();
    let mut secret_key = SecretKey::from_slice(&private_gen).expect("32 bytes, within curve order");
    let mut public_gen = PublicKey::from_secret_key(&secp, &secret_key).serialize().to_vec();
    // println!("public gen : {:?}", public_gen);

    //derivePrivateKey return
    //secp256k1.ScalarMultiple(publicGen.encodeCompressed(), 0).add(privateGen).mod(order);
    let public_gen_output = scalar_multiple(public_gen.as_slice(), Some(0));
    println!("before add : {:?}", public_gen_output);
    let mut secret_key2 = SecretKey::from_slice(&public_gen_output).expect("32 bytes, within curve order");
    let x = secret_key2.add_assign(&secret_key[..]);
    println!("x : {:?}", secret_key2);

    //var privateKey = prefix + derivePrivateKey(entropy).toString(16, 64).toUpperCase();
    let private_key = "00".to_owned() + secret_key2.to_string().as_str();
    println!("private_key : {}", private_key.to_ascii_uppercase());
    
    return;


    //1. order
    let order: [u8; 32] = [
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
                    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b,
                    0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41];
	println!("order : {:?}", order);
    println!("seed  : {:?}", seed);

    {
        //ScalarMultiple
        let mut i = 0u32;
        while i <= 0xFFFFFFFF  {
            // We hash the bytes to find a 256 bit number, looping until we are sure it
            // is less than the order of the curve.
            // var hasher = new Sha512().add(seed);

            {
                let mut ctx = digest::Context::new(&digest::SHA512);
                ctx.update(&seed);
                ctx.update(&[0,0,0,0]);
                let multi_part = ctx.finish();
                let xx: String = multi_part.as_ref().iter().map(|c| {
                    let x = format!("{:x}", c);
                    x 
                }).collect();
                let key = xx.get(0..32).unwrap().to_string();

                let xx_order: String = order.iter().map(|c| {
                    let x = format!("{:x}", c);
                    x 
                }).collect();
                let order = xx_order.get(0..32).unwrap().to_string();

                if key < order && key > "0".to_string() {

                    ////[1]
                    println!("privateGen : {:?}", key);
                }
                //---> key == privateGen

                //
                {
                    // use num::bigint::BigInt;

                    // let low = BigInt::parse_bytes(b"e74ff93c819ff0a1ca781617b4de7b92", 16);
                    // println!("looooooooooooooooow: {:?}", low);

                    // let xy = b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
                    // let yy = b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
                    // let g = BigInt::parse_bytes(xy, 16).unwrap();
                    // println!("g : {:?}", g);

                    // let ret = g.checked_mul(&low.unwrap());
                    // println!("low : {:?}", ret);

                    // // println!("to big endian: {:?}", &ret.unwrap().to_bytes_be());
                    // println!("to little endian: {:?}", &ret.unwrap().to_bytes_le());

                    // return;
                }
                

                // {
                //     use secp256k1::ffi::*;

                //     unsafe {
                //         let mut secret_key = SecretKey::from_slice(key.as_bytes()).expect("32 bytes, within curve order");
                //         let context = secp256k1_context_create(SECP256K1_START_NONE);
                //         let mut public_key = PublicKey::new();
                //         secp256k1_ec_pubkey_create(context, &mut public_key, );
                //         println!("Publickey raw : {:?}", public_key);
                //     }

                //     return;
                // }



                extern crate hex;
                if let Ok(keyx) = hex::decode("e74ff93c819ff0a1ca781617b4de7b921678073e07b18a6830900e5370a69f03") {

                     println!("keyx : {:?}", keyx);
                    let secp = Secp256k1::new();
                    let mut secret_key = SecretKey::from_slice(&keyx).expect("32 bytes, within curve order");
                    let mut public_key = PublicKey::from_secret_key(&secp, &secret_key);
                    println!("public key : {:?}", public_key.serialize().to_vec());
                }

                return;
                let secp = Secp256k1::new();
                let mut secret_key = SecretKey::from_slice(key.as_bytes()).expect("32 bytes, within curve order");


                
                

                // secret_key.mul_assign(&GENERATOR_X);
                // println!("pk1 : {:?}", secret_key);
                // secret_key.mul_assign(&GENERATOR_Y);
                // println!("pk2 : {:?}", secret_key);

                // return;

                // println!("pubk : {:?}", secret_key);


                //compressed
                let mut public_key = PublicKey::from_secret_key(&secp, &secret_key);
                println!("public_key: {}", public_key.to_string());
                let public_key_seris = public_key.serialize().to_vec();
                // println!("seris : {:#?}", public_key_seris);

                //ScalarMultiple again.
                {

                    let mut ctx = digest::Context::new(&digest::SHA512);
                    ctx.update(&public_key_seris);
                    ctx.update(&[0,0,0,0]);
                    ctx.update(&[0,0,0,0]);
                    let multi_part = ctx.finish();
                    // println!("multi_part: {:?}", multi_part.as_ref());
                    let xx: String = multi_part.as_ref().iter().map(|c| {
                        let x = format!("{:x}", c);
                        x 
                    }).collect();
                    let key1 = xx.get(0..32).unwrap().to_string();
                    println!("after public key compress : {:?}", key1);

                    //add privategen mod order


                    let mut secret_key1 = SecretKey::from_slice(key1.as_bytes()).expect("32 bytes, within curve order");
                    secret_key1.add_assign(key.as_bytes());
                    println!("secret_key1 : {:?}", secret_key1);
                    // let xx: String = secret_key1.as_ref().iter().map(|c| {
                    //     let x = format!("{:x}", c);
                    //     x 
                    // }).collect();
                    // let key = xx.get(..).unwrap().to_string().to_ascii_uppercase();



                    let privateKey = "00".to_owned() + secret_key1.to_string().to_ascii_uppercase().as_str();
                    println!("lasssssssssssssssst: {:?}", privateKey);
                    let public_key = PublicKey::from_secret_key(&secp, &secret_key1);
                    // let public_key = PublicKey::from_str()
                    let pubbbbb = public_key.serialize();
                    let xx: String = pubbbbb.iter().map(|c| {
                        let x = format!("{:x}", c);
                        x 
                    }).collect();
                    let key = xx.get(..).unwrap().to_string().to_ascii_uppercase();
                    println!("lasssssssssssssssssssst pub key : {:?}", key);




                    // let mut ctx = digest::Context::new(&digest::SHA512);
                    // ctx.update(key.as_bytes());
                    // ctx.update(&[0,0,0,0]);
                    // ctx.update(&[0,0,0,0]);
                    // let multi_part = ctx.finish();
                    // println!("multi_part: {:?}", multi_part.as_ref());
                    // let xx: String = multi_part.as_ref().iter().map(|c| {
                    //     let x = format!("{:x}", c);
                    //     x 
                    // }).collect();
                    // let key1 = xx.get(0..32).unwrap().to_string();
                    // println!("after public key compress : {:?}", key1);

                    println!("\n\n");

                extern crate crypto;
                use crypto::ripemd160::Ripemd160 ;
                use crypto::digest::Digest;

                let publickey = vec![ 2,
  167,
  80,
  56,
  21,
  217,
  185,
  138,
  191,
  143,
  112,
  199,
  11,
  220,
  55,
  213,
  239,
  59,
  177,
  89,
  231,
  32,
  254,
  153,
  13,
  223,
  175,
  205,
  240,
  92,
  210,
  15,
  189 ];
// 
                // var hash256 = hashjs.sha256().update(bytes).digest();
                println!("bytes: {:?}", publickey);
                let mut ctx = digest::Context::new(&digest::SHA256);
                ctx.update(&publickey);
                let hash256 = ctx.finish();
                let xx: String = hash256.as_ref().iter().map(|c| {
                    let x = format!("{:x}", c);
                    x 
                }).collect();
                let hash256 = xx.get(0..32).unwrap().to_string();
                println!("hash256 : {:?}", hash256);

                let input = vec! [ 239,
  175,
  56,
  10,
  109,
  25,
  147,
  68,
  81,
  57,
  75,
  188,
  168,
  165,
  110,
  53,
  212,
  206,
  172,
  27,
  209,
  62,
  130,
  165,
  31,
  99,
  69,
  147,
  218,
  186,
  42,
  80 ];
                let mut ripemd160x = Ripemd160::new();
                ripemd160x.input(&input);
                let ripemd160x= ripemd160x.result_str();
                println!("ripemd160x : {}", ripemd160x);
                }
                

                let accountid= vec! [ 16,
  128,
  12,
  194,
  94,
  11,
  79,
  93,
  113,
  241,
  33,
  182,
  90,
  57,
  114,
  27,
  88,
  168,
  57,
  46 ];


        //concat args
        let args = vec! [ 0,
  16,
  128,
  12,
  194,
  94,
  11,
  79,
  93,
  113,
  241,
  33,
  182,
  90,
  57,
  114,
  27,
  88,
  168,
  57,
  46 ];



{
let ac = digest::digest(&digest::SHA256, &args);
        let checked = digest::digest(&digest::SHA256, &ac.as_ref());
        let xx: Vec<u8> = checked.as_ref().iter().map(|c| {
            let x = format!("{:x}", c);
            x.as_str().chars().nth(0).unwrap() as u8
        }).collect::<Vec<u8>>();
        //println!("checked : {:?}", xx.get(..4));

        let ret = xx.get(..4).unwrap().to_vec();
        println!("ret : {:?}", ret);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
}





                return;

                // secp256k1.ScalarMultiple(publicGen.encodeCompressed(), 0).add(privateGen).mod(order);



            }

            i += 1;
        }





    }
}