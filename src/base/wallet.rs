
use crate::base::brorand::*;
use crate::base::*;
use crate::base::constants::{ALPHABET, PASSWORD_LEN};



// 钱包生成器
pub struct WalletBuilder {
    pub config: WalletConfig,
}

impl WalletBuilder {
    // pub fn new(config: WalletConfig) -> Self {
    //     WalletBuilder {
    //         config: config,
    //     }
    // }

    pub fn build(config: WalletConfig) -> Wallet {
        Wallet {
            key_type: config.key_type,
            address : "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string(), //test
            secret  : WalletBuilder::generate_seed(),
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

#[derive(Debug)]
//key的加密算法：ed25519 / secp256k1
pub enum KeyType {
    SECP256K1,
    ED25519,
}

//钱包属性：地址长度，加密算法等
#[derive(Debug)]
pub struct WalletConfig {
    pub key_type: KeyType,
}
impl WalletConfig {
    pub fn new(key_type: KeyType) -> Self {
        WalletConfig {
            key_type: key_type,
        }
    }
}

#[derive(Debug)]
pub struct Wallet {
    pub key_type: KeyType, 
    pub address : String,  //j开头的钱包地址
    pub secret  : String,  //secret seed
    pub keypair : Option<Keypair>, //公钥私钥对
}

impl Wallet {
    pub fn new(config: WalletConfig) -> Self {
        WalletBuilder::build(config)
    }
}
