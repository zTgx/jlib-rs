
use crate::base::misc::brorand::*;
use crate::base::misc::util::*;

use crate::base::wallet::config::*;
use crate::base::data::constants::PASSWORD_LEN;

use crate::base::wallet::keypair::*;
use crate::base::wallet::address::*;

use crate::WalletType;

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

        //seed
        let seed = self.generate();
        
        //keypair
        let key_pair = KeypairBuilder::new(&seed).build();

        //address
        let address = WalletAddressBuilder::new(&key_pair).build();

        Wallet {
            key_type: self.config.key_type,
            address : address,
            secret  : seed,
            keypair : key_pair,
        }
    }

    //private method
    fn generate(&self) -> String {
        //1. Generete 16 random data
        let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);

        //2. add secp256k1
        let _opt = "secp256k1";

        //3. encodeSeed function
        let mut version: Vec<u8> = [33].to_vec();

        //4. concat args
        concat_args(&mut version, &u);

        //5. encodechecked.
        let checked: Vec<u8> = encode_checked(&mut version);

        //6. concat args
        concat_args(&mut version, &checked);

        encode_raw(&mut version)
    }
}


#[derive(Debug)]
pub struct Wallet {
    pub key_type: WalletType,
    pub address : WalletAddress,   //j开头的钱包地址
    pub secret  : String,          //secret seed
    pub keypair : Keypair,         //公钥私钥对
}

impl Wallet {
    pub fn new(wtype: WalletType) -> Self {
        let config = WalletConfig::new(wtype);
        WalletBuilder::new(&config).build()
    }
}
