pub mod wallet;
pub mod config;
pub mod keypair;
pub mod address;
pub mod seed;

use basex_rs::BaseX;
use crate::base::curve::sha256::JSha256;

//generate seed && address
pub fn generate_str(version: &mut Vec<u8>, so: &Vec<u8>) -> String {
    //4. concat args
    version.extend(so);

    //5. sha256
    let checked: Vec<u8> = JSha256::sha256(&version);

    //6. take 0..4
    let token = checked.get(..4).unwrap().to_vec();

    //7. concat args
    version.extend(token);

    //end. base58 encode
    BaseX::encode(version.as_mut_slice())
}

pub fn check_address(address: &String) -> bool {

    true
}

pub fn check_secret(secret: &String) -> bool {

    true
}
