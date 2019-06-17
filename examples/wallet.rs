
extern crate rand;

static ALPHABET: &[u8] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";


////////////////////////////////////////////////////////////////////////////////////////
//Keypairs
pub struct Keypairs {

}

impl Keypairs {
    pub fn generateSeed() {
        //addressCodec.encodeSeed(brorand(16), "secp256k1")

        use rand::Rng;
        // const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        //                         abcdefghijklmnopqrstuvwxyz\
        //                         0123456789";
        const PASSWORD_LEN: usize = 16;
        let mut rng = rand::thread_rng();

        // let password: String = (0..PASSWORD_LEN)
        //     .map(|_| {
        //         let idx = rng.gen_range(0, ALPHABET.len());
        //         // This is safe because `idx` is in range of `CHARSET`
        //         char::from(unsafe { *ALPHABET.get_unchecked(idx) })
        //     })
        //     .collect();

        // println!("{:?}", password);

        //1. Generete 16bytes random data
        let x: String = (0..PASSWORD_LEN).map(|_| {
            let idx: u8 = rand::thread_rng().gen();
            println!("idx : {}", idx);
            let hexs = format!("{:x}", idx);
            println!("hexs : {}", hexs);

            hexs
        }).collect();

        println!("x : {:?}", x);
        //2. add secp256k1
        let opt = "secp256k1";

        //3. encodeSeed function
        //encodeVersioned(bytes, version, opts.expectedLength)
        let version = "3";

        //4. concat args
        Keypairs::concat_args(version, &x);
        // console.log("concatArgs : ", x);




    }

    pub fn concat_args(version: &'static str, bytes: &String) {
        let mut new_string = version.to_owned() + bytes;

        unsafe {
        let hexs: String = new_string.as_mut_vec().iter().map(|c| {
            let x = format!("{:x}", c);

            x
        }).collect();
        println!("new_string : {:?}", hexs);

        //5. encodechecked.
        Keypairs::encode_checked(hexs);


        }
    }

    pub fn encode_checked(x: String) {
        use ring::{digest, test};

        // let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
        // let expected: Vec<u8> = test::from_hex(expected_hex).unwrap();
        let vv: &[u8] = &[ 33, 228, 98, 120, 229, 208, 105, 36, 76, 162, 155, 0, 178, 95, 45, 115, 89 ];
        // let actual = digest::digest(&digest::SHA256, x.as_bytes());
        // let check = digest::digest(&digest::SHA256, &actual.as_ref());
        // let xx: Vec<u8> = check.as_ref().to_vec();
        {
            let ac = digest::digest(&digest::SHA256, vv);
            let checked = digest::digest(&digest::SHA256, &ac.as_ref());
            let xx: String = checked.as_ref().iter().map(|c| {
                let x = format!("{:x}", c);

                x
            }).collect();
            println!("new_string : {:?}", xx.get(..8));



            
        }

        let raw: &[u8] =  &[ 33,
  228,
  98,
  120,
  229,
  208,
  105,
  36,
  76,
  162,
  155,
  0,
  178,
  95,
  45,
  115,
  89,
  57,
  48,
  147,
  236 ];
        Keypairs::encode_raw(raw);//Keypairs::concat_args(buffer, check));
    }

    pub fn encode_raw(x: &[u8]) {
            Keypairs::encode(x);
    }

    pub fn encode(source: &[u8]) {
            // if (source.length === 0) return ''

            println!("source : {:?} and len : {}", source, source.len());
            
    // let source = &source[..4];
    let BASE = ALPHABET.len() as u16;

    let mut digits: Vec<u16> = vec![0u16; 1];
    
    let mut i = 0;
    while i < source.len() {

        let mut j = 0;
        let mut carry: u16 = source[i] as u16;
        
        let digits_len = digits.len();
        while j < digits_len {
            let y = &digits.as_slice()[j];
            println!("\n\ndigits[j] : {}", y);
            println!("digits[j] << 8 : {}", y << 8);

            println!("before add carray : {}", carry);
            carry += digits.as_slice()[j] << 8;
            println!("after add carray : {}", carry);

            //digits.insert(j, carry % (BASE as u16));
            digits.as_mut_slice()[j] = carry % (BASE as u16);
            println!("digits : {:?}", digits);

            carry = (carry / (BASE as u16)) | 0;
            println!("carray : {}", carry);

            j += 1;
        }

        // let s = [10, 40, 30];
        // let x = s.to_vec();
        // Here, `s` and `x` can be modified independently.
        while carry > 0 {
            digits.push(carry % (BASE as u16));
            println!("-digits : {:?}", digits);
            carry = (carry / BASE) | 0;
            println!("-carray : {}\n\n", carry);
        }

        i += 1;
    }

    //println!("digits : {:?}", digits);
    let mut string = "".to_string();

    // deal with leading zeros
    let mut k = 0;
    while source[k] == 0 && k < source.len() - 1 {

        string += ALPHABET[0].to_string().as_str();

        k += 1;
    }        
    // convert digits to a string
    let mut q: i32 = (digits.len() - 1) as i32;
    while q >= 0 {

        // string += ALPHABET[digits[q as usize] as usize].to_string().as_str();

        let uu: u8 = ALPHABET[digits[q as usize] as usize];
        let xx = uu as char;
        string.push( xx );

        // println!("string : {}", xx);
        q -= 1;
    }

    println!("string : {}", string);

    }
}

////////////////////////////////////////////////////////////////////////////////////////
//Wallet

pub struct Wallet {
    pub keypairs: String, //...
    pub secret: String,//
}

impl Wallet {
    //generate
    pub fn generate() {
        let secret = Keypairs::generateSeed();        
        // let keypair = Keypairs::deriveKeyPair(secret);
        // let address = Keypairs::deriveAddress(keypair.publicKey);

        // GenReturn {
        //     secret: secret,
        //     address: address
        // }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
    Wallet::generate();
}