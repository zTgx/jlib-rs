use crate::base::address::traits::address::AddressI;
use crate::WalletType;
use crate::base::address::guomi::AddressGuomi;

pub struct AddressBuilder {
    address   : Box<dyn AddressI>,
}

impl AddressBuilder {
    pub fn new(seed_type: WalletType, seed: &Vec<u8>) -> Self {
        let address = AddressBuilder::build_address(seed_type, &seed);
        
        AddressBuilder {
            address  : address,
        }
    }

    fn build_address(seed_type: WalletType, seed: &Vec<u8>) -> Box<dyn AddressI> {
        match seed_type {
            WalletType::ED25519 => {
                return Box::new(AddressGuomi::new(&seed));
            },

            WalletType::SECP256K1 => {
                return Box::new(AddressGuomi::new(&seed));
            },
            WalletType::SM2P256V1 => {
                return Box::new(AddressGuomi::new(&seed));
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