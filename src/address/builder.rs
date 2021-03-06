use crate::wallet::wallet::WalletType;

use crate::address::traits::address::AddressI;
use crate::address::traits::address::AddressCheckI;
use crate::address::types::sm2p256v1::AddressSM2P256V1;

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
                return Box::new(AddressSM2P256V1::new(&seed));
            },

            WalletType::SECP256K1 => {
                return Box::new(AddressSM2P256V1::new(&seed));
            },
            WalletType::SM2P256V1 => {
                return Box::new(AddressSM2P256V1::new(&seed));
            }
        }
    }
}

// ----------------------------------------------------------------------------------------------------------
// AddressBuilder 对 trait AddressI 的实现。
// ----------------------------------------------------------------------------------------------------------
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

    fn private_key(&self) -> String {
        self.address.private_key()
    }
}

// ----------------------------------------------------------------------------------------------------------
// AddressBuilder 对 trait AddressCheckI 的实现。
// ----------------------------------------------------------------------------------------------------------
impl AddressCheckI for AddressBuilder {
    fn check(&self, _address: &String) -> bool {
        true
    }
}