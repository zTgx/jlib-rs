use crate::base::misc::brorand::Brorand;
use crate::base::data::constants::PASSWORD_LEN;
use crate::WalletType;
use basex_rs::BaseX;
use crate::base::curve::sha256::JSha256;

static H_SECP256K1: &[u8] = &[33];
static H_ED25519: &[u8] = &[33];

pub struct Seed {}
impl Seed {
    pub fn build(wtype: &WalletType) -> String {
        //1. Generete PASSWORD_LEN random data
        let u: Vec<u8> = Brorand::brorand(PASSWORD_LEN);

        //2. dependen on type decide which curve to use secp256k1/de255119
        //3. encodeSeed function
        let mut version: Vec<u8>; //default secp256k1
        match wtype {
            &WalletType::ED25519 => {
                version = H_ED25519.to_vec();
            },

            &WalletType::SECP256K1 => {
                version = H_SECP256K1.to_vec();
            },
        }

        //4. concat args
        version.extend(u);

        //5. sha256
        let checked: Vec<u8> = JSha256::sha256(&version);

        //6. take 0..4
        let token = checked.get(..4).unwrap().to_vec();

        //7. concat args
        version.extend(token);

        //end. base58 encode
        BaseX::encode(version.as_mut_slice())
    }
}
