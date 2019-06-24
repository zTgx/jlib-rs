
use crate::base::brorand::*;
use crate::base::*;

use crate::base::config::*;
use crate::base::constants::PASSWORD_LEN;

use crate::base::seed::*;
use crate::base::keypair::*;
use crate::base::address::*;

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
        let seed = self.generate();//"sn37nYrQ6KPJvTFmaBYokS3FjXUWd";
        let seed_property = SeedProperty::new(&seed, 16);
        let seed = SeedBuilder::new(seed_property).build();

        //keypair
        let key_pair = KeypairBuilder::new(&seed).build();

        //address
        let address = WalletAddressBuilder::new(&key_pair).build();

        Wallet {
            key_type: self.config.key_type,
            address : address,//"jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string(), //test
            secret  : seed,
            keypair : Some(key_pair),
        }
    }

    //private method
    fn generate(&self) -> String {
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

        util::encode_raw(&mut version)
    }
}


#[derive(Debug)]
pub struct Wallet {
    pub key_type: KeyType, 
    pub address : WalletAddress,  //j开头的钱包地址
    pub secret  : Seed,  //secret seed
    pub keypair : Option<Keypair>, //公钥私钥对
}

impl Wallet {
    pub fn new(config: &WalletConfig) -> Self {
        WalletBuilder::new(config).build()
    }
}
