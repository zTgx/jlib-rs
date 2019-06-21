
use crate::base::brorand::*;
use crate::base::*;

use crate::base::config::*;
use crate::base::constants::PASSWORD_LEN;

use crate::base::seed::*;

// 钱包生成器
#[derive(Debug)]
pub struct WalletBuilder <'a> {
    pub config: &'a WalletConfig,
}

impl <'a> WalletBuilder <'a> {
    pub fn new(config: &'a WalletConfig) -> Self {
        WalletBuilder {
            config: config,
        }
    }

    pub fn build(&self) -> Wallet {

        let seed: &'a str = "sn37nYrQ6KPJvTFmaBYokS3FjXUWd";
        let seed_property = SeedProperty::new(seed, 16);
        let seed = SeedBuilder::new(seed_property).build();

        Wallet {
            key_type: self.config.key_type,
            address : "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string(), //test
            secret  : seed,
            keypair : None, //test
        }
    }

    //private method
    fn generate_seed() -> String {
        //1. Generete 16 random data
        let mut u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);

        //2. add secp256k1
        let opt = "secp256k1";

        //3. encodeSeed function
        let mut version: Vec<u8> = [33].to_vec();

        //4. concat args
        util::concat_args(&mut version, &u);

        //5. encodechecked.
        let mut checked: Vec<u8> = util::encode_checked(&mut version);

        //6. concat args
        util::concat_args(&mut version, &checked);

        // let secret: String = util::encode_raw(&mut version);
        // println!("secret : {}", secret);

        util::encode_raw(&mut version)
    }
}


#[derive(Debug)]
pub struct Keypair {
    pub secret_key: String, //私钥
    pub public_key: String, //公钥
}
impl Keypair{}


#[derive(Debug, Copy, Clone)]
pub struct WalletAddress <'a> {
    pub address: &'a str,
}
impl <'a> WalletAddress <'a> {
    pub fn new() -> Self {
        WalletAddress {
            address: "default",
        }
    }

    // pub fn update(&mut self, bytes: &[u8]) {
    //     self.address = bytes;
    // }

    pub fn get(&self) -> String {
        self.address.to_owned()
    }
}


#[derive(Debug)]
pub struct Wallet {
    pub key_type: KeyType, 
    pub address : String,  //j开头的钱包地址
    pub secret  : Seed,  //secret seed
    pub keypair : Option<Keypair>, //公钥私钥对
}

impl Wallet {
    pub fn new(config: &WalletConfig) -> Self {
        WalletBuilder::new(config).build()
    }
}
