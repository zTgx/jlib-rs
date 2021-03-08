use crate::base::address::traits::address::AddressI;
use crate::WalletType;
use crate::base::misc::brorand::Brorand;
use crate::base::data::constants::PASSWORD_LEN;
use crate::base::wallet::{
    generate_str
};
use crate::base::address::guomi::Address;

pub struct AddressBuilder {
    seed_type : WalletType,
    address   : Box<dyn AddressI>,
}

impl AddressBuilder {
    pub fn new(seed_type: WalletType, seed: &Vec<u8>) -> Self {
        let address = AddressBuilder::build_address(seed_type, &seed);
        
        AddressBuilder {
            seed_type: seed_type,
            address  : address,
        }
    }

    fn build_address(seed_type: WalletType, seed: &Vec<u8>) -> Box<dyn AddressI> {
        match seed_type {
            WalletType::ED25519 => {
                return Box::new(Address::new(&seed));
            },

            WalletType::SECP256K1 => {
                return Box::new(Address::new(&seed));
            },
            WalletType::SM2P256V1 => {
                return Box::new(Address::new(&seed));
            }
        }
    }
}

impl AddressI for AddressBuilder {
    fn human_account_id(&self) -> String {
        self.address.human_account_id()
    }
    
    fn public_key(&self) -> String {
        self.address.public_key()
    }
    
    fn public_key_hex(&self) -> String {
        self.address.public_key_hex()
    }
}