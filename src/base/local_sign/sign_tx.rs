use crate::wallet::keypair::*;

use crate::base::serialize::signed_obj::{SignedTxJson, TxJsonTxnSignatureBuilder, TxJsonBuilder};

use crate::base::crypto::signature::traits::signature::SignatureI;
use crate::base::crypto::signature::builder::SignatureBuilder;

use crate::base::data::inverse_fields_map::INVERSE_FIELDS_MAP;

use crate::api::payment::data::{TxJson};
use crate::api::set_relation::data::{RelationTxJson};
use crate::api::create_offer::data::{OfferCreateTxJson};
use crate::api::cancel_offer::data::OfferCancelTxJson;

use crate::api::set_fee_rate::data::SetBrokerageTxJson;

use hex;

use crate::base::data::constants::TX_SIGNATURE;

use crate::base::local_sign::sign_pay::{SignTxPay};
use crate::base::local_sign::sign_relate::{SignTxRelate};
use crate::base::local_sign::sign_cancel_offer::{SignTxCancelOffer};
use crate::base::local_sign::sign_create_offer::{SignTxCreateOffer};
use crate::base::local_sign::sign_brokerage::{SignTxBrokerage};

use crate::address::types::seed::SeedBuilder;

use crate::address::traits::address::AddressI;
use crate::address::builder::AddressBuilder;
use crate::wallet::wallet::WalletType;
use crate::wallet::builder::WalletBuilder;

pub const PRE_FIELDS: [&'static str; 6] = ["Flags", "Fee", "TransactionType", "Account", "SigningPubKey", "Sequence"];

pub struct SignTx {
    pub sequence: u32, //account seq
    pub keypair: Keypair, 
}
impl SignTx {
    pub fn with_params(sequence: u32, secret: &str) -> Self {
        let seed = SeedBuilder::secret_to_seed(&secret.to_string());  
        let key_type: WalletType = WalletBuilder::get_wallet_type_from_seed(&secret);      
        let address = AddressBuilder::new(key_type, &seed);

        let keypair = Keypair {
            public_key: address.public_key_hex(),
            private_key: address.private_key(),
        };

        SignTx {
            sequence: sequence,
            keypair : keypair,
        }
    }
}

//Entrance
impl SignTx {
    pub fn pay(&self, tx_json: &TxJson) -> String {
        SignTxPay::with_params(&self.keypair, &tx_json, self.sequence).build(self)
    }

    pub fn relate(&self, tx_json: &RelationTxJson) -> String {
        SignTxRelate::with_params(&self.keypair, &tx_json, self.sequence).build(self)
    }

    pub fn create_offer(&self, tx_json: &OfferCreateTxJson) -> String {
        SignTxCreateOffer::with_params(&self.keypair, &tx_json, self.sequence).build(self)
    }

    pub fn cancel_offer(&self, tx_json: &OfferCancelTxJson) -> String {
        SignTxCancelOffer::with_params(&self.keypair, &tx_json, self.sequence).build(self)
    }

    pub fn set_rate(&self, tx_json: &SetBrokerageTxJson) -> String {
        SignTxBrokerage::with_params(&self.keypair, &tx_json, self.sequence).build(self)
    }
}

//common
impl SignTx {
    pub fn get_blob(&self, signed_tx_json: &mut SignedTxJson) -> String {
        let output: Vec<u8> = signed_tx_json.serialize();
        hex::encode(&output).to_ascii_uppercase()
    }

    pub fn get_txn_signature(&self, fields: &mut Vec<&str>, signed_tx_json: &mut SignedTxJson) {
        let output: Vec<u8> = signed_tx_json.serialize();

        let signature_builder = SignatureBuilder::new(WalletType::SM2P256V1, Keypair {
            private_key: self.keypair.private_key.to_owned(),
            public_key : self.keypair.public_key.to_owned()
        });
        let txn_signature = signature_builder.sign_txn_signature(&output);

        self.update(fields, TX_SIGNATURE);

        let mut index = 0; 
        for x in fields {
            if *x == TX_SIGNATURE {
                break;
            }

            index += 1;
        }

        let txn_signature = TxJsonTxnSignatureBuilder::new(txn_signature).build();
        signed_tx_json.insert(index, txn_signature);
    }

    pub fn update(&self, fields: &mut Vec<&str>, field: &'static str) {
        fields.push(field);
        SignTx::sort_fields(fields);
    }

    pub fn sort_fields(fields: &mut Vec<&str>) {
        fields.sort_by( |a, b| {
            let a_field_coordinates = INVERSE_FIELDS_MAP.get(a).unwrap();
            let a_type_bits = a_field_coordinates[0];
            let a_field_bits = a_field_coordinates[1];

            let b_field_coordinates = INVERSE_FIELDS_MAP.get(b).unwrap();
            let b_type_bits = b_field_coordinates[0];
            let b_field_bits = b_field_coordinates[1];

            // Sort by type id first, then by field id
            if a_type_bits != b_type_bits {

              //  a_type_bits - b_type_bits
              a_type_bits.cmp(&b_type_bits)

            } else {

              // a_field_bits - b_field_bits
              a_field_bits.cmp(&b_field_bits)

            }
        });
    }
}
