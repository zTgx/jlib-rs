
extern crate rand;
use rand::Rng;

extern crate mylib;
use mylib::base::brorand::*;
use mylib::base::*;
use mylib::base::constants::{ALPHABET, PASSWORD_LEN};

//Keypairs
pub struct Keypairs {
}

impl Keypairs {
    pub fn generateSeed() {
        //1. Generete 16 random data
        let mut u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);
        println!("u: {:?}", u);

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

        let secret: String = util::encode_raw(&mut version);
        println!("secret : {}", secret);
    }
}

////////////////////////////////////////////////////////////////////////////////////////
//Wallet

//钱包属性：地址长度，加密算法等
pub struct WalletConfig {
    pub key_type: String,  //key的加密算法：ed25519 / secp256k1
}

impl WalletConfig {}

//
pub struct WalletBuilder {
    pub config: WalletConfig,
}

impl WalletBuilder {
    pub fn new(config: WalletConfig) -> Self {
        WalletBuilder {
            config: config,
        }
    }

    pub fn build(&self) -> Wallet {

    }
}

#[derive(Debug)]
pub struct Keypair {
    pub secret_key: String, //私钥
    pub public_key: String, //公钥
}

impl Keypair{}

pub struct Wallet {
    pub address : String,  //j开头的钱包地址
    pub secret  : String,  //secret seed
    pub keypair : Keypair, //公钥私钥对

    builder: WalletBuilder, // 钱包生成器
}

impl Wallet {
    pub fn new(config: WalletConfig) -> Self {
        WalletBuilder::new(config).build()
    }
}

fn main() {
    // Wallet::generate();

    let ret = util::entropy("ssndDcNoc4FvwVPveY3KbfWh8fNh3".to_string());
    println!("entropy : {:?}", ret);
}