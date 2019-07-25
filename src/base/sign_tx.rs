

use crate::base::keypair::*;
use crate::base::util::{get_keypair_from_secret};
use crate::base::signed_obj::{SignedTxJson, TxJsonTxnSignatureBuilder, TxJsonBuilder};
use cast_rs::hex_t;
use crate::base::sign::SignatureX;
use crate::base::inverse_fields_map::INVERSE_FIELDS_MAP;

use crate::message::transaction::transaction::{TxJson};
use crate::message::transaction::relation::{RelationTxJson};
use crate::message::transaction::offer_create::{OfferCreateTxJson};
use crate::message::transaction::offer_cancel::{OfferCancelTxJson};

use crate::base::constants::{
    TX_SIGNATURE, 
};

use crate::base::sign_pay::{SignTxPay};
use crate::base::sign_relate::{SignTxRelate};
use crate::base::sign_cancel_offer::{SignTxCancelOffer};
use crate::base::sign_create_offer::{SignTxCreateOffer};

pub const PRE_FIELDS: [&'static str; 6] = ["Flags", "Fee", "TransactionType", "Account", "SigningPubKey", "Sequence"];

pub struct SignTx {
    pub sequence: u32, //account seq
    pub keypair: Keypair, 
}
impl SignTx {
    pub fn with_params(sequence: u32, secret: &str) -> Self {
        SignTx {
            sequence: sequence,
            keypair : get_keypair_from_secret(&secret.to_string()),
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
}

//common
impl SignTx {
    pub fn get_blob(&self, signed_tx_json: &mut SignedTxJson) -> String {
        let output: Vec<u8> = signed_tx_json.serialize();
        hex_t::encode(&output).to_ascii_uppercase()
    }

    pub fn get_txn_signature(&self, fields: &mut Vec<&str>, signed_tx_json: &mut SignedTxJson) {
        let output: Vec<u8> = signed_tx_json.serialize();

        let signature_x = SignatureX::new(&self.keypair);
        let txn_signature = signature_x.sign_txn_signature(&output);
        println!("txn_signature: {}", txn_signature);

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

        println!("sorted: {:?}", &fields);
    }
}