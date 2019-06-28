// #[macro_use] 
// extern crate typename;

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::base::inverse_fields_map::INVERSE_FIELDS_MAP;
use crate::base::types_map::TYPES_MAP;
use crate::base::serialized_type::*;

use typename::TypeName;
use crate::base::amount::*;
use crate::base::sign::*;
use ring::{digest};

use crate::transaction::{TxJson};
use crate::base::type_obj::*;
use crate::base::signed_obj::*;

use crate::base::constants::{
    TX_SIGNATURE, TX_DESTINATION, TX_ACCOUNT, TX_SIGNING_PUB_KEY, TX_FEE, 
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE,TX_FLAGS
};
use std::rc::Rc;

const PRE_FIELDS: [&str; 7] = ["Flags", "Fee", "TransactionType", "Account", "Amount", "Destination", "Sequence"];


pub struct SignTx {
}
impl SignTx {

    pub fn prepare(tx_json: TxJson) {
        //Step 1: Get Non-None field. [SigningPubKey] / [TxnSignature] / [Memos]
        let mut fields: Vec<&str> = vec![];
        fields.extend_from_slice(&PRE_FIELDS);
        if tx_json.memo.is_some() {
            fields.push("Memos");
        }
        if tx_json.signing_pubKey.is_some() {
            fields.push("SigningPubKey");
        }
        if tx_json.txn_signature.is_some() {
            fields.push("TxnSignature");
        }

        //Step 2: sorted
        SignTx::sort_fields(&mut fields);
        // println!("sorted fields: {:?}", fields);

        //Step 3: serialize tx fields
        let mut tx: SignedTxJson = SignedTxJson::new();
        // let tx_json = tx_json.copy();
        SignTx::format_tx_obj(tx_json, &mut tx, &fields);

        //Step 4 : serialize
        let output: Vec<u8> = tx.serialize();
        let txn_signature= SignTx::update_txn_signature(&output);
        println!("txn_signature: {}", txn_signature);
    }

    //Hex String sign(tx_json) => blob
    pub fn serialize_all_fields(so: &Vec<u8>) -> String {
        hex::encode(&so).to_ascii_uppercase()
    }

    //Output Hex String
    pub fn update_txn_signature(so: &Vec<u8>) -> String {
        let mut ctx = digest::Context::new(&digest::SHA512);
        ctx.update(&[83,84,88, 0]);
        ctx.update(&so);

        let mut hash = hex::encode(&ctx.finish().as_ref());
        let mut message = hash.get(0..64).unwrap().to_ascii_uppercase();
        message
        
        // let key = [26, 202, 174, 222, 206, 64, 91, 42, 149, 130, 18, 98, 158, 22, 242, 235, 70, 177, 83, 238, 233, 76, 221, 53, 15, 222, 255, 82, 121, 85, 37, 183];

        // let msg = hex::decode(message).unwrap();
        // let mut signed_hex_string = SignatureX::sign(&msg, &key);
        // return signed_hex_string;
    }

    pub fn format_tx_obj(tx_json: TxJson, output: &mut SignedTxJson, fields: &Vec<&str>) {
        let tx_json_rc = Rc::new ( tx_json );
        for &key in fields {
            let tx_json = tx_json_rc.clone();
            println!("key : {}", key);
            match key {
                TX_FLAGS => {
                    println!("flags");
                    let value = tx_json.flags;
                    let flags = TxJsonFlagsBuilder::new(value).build();
                    output.insert(flags);
                },
                TX_FEE => {
                    println!("fee");
                    let value = tx_json.fee;
                    let fee = TxJsonFeeBuilder::new(value.to_string()).build();
                    output.insert(fee);
                },
                TX_TRANSACTION_TYPE => {
                    println!("transaction type");
                    //TRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPES
                    let value = 0u16;//tx_json.transaction_type;
                    let transaction_type = TxJsonTransactionTypeBuilder::new(value).build();
                    output.insert(transaction_type);
                },
                TX_ACCOUNT => {
                    println!("account");
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    output.insert(account);
                },
                TX_AMOUNT => {
                    println!("amount");
                    let value = String::from(tx_json.amount.as_str());
                    let amount = TxJsonAmountBuilder::new(value).build();
                    output.insert(amount);
                },
                TX_DESTINATION => {
                    println!("destination");
                    let value = String::from(tx_json.destination.as_str());
                    let destination = TxJsonDestinationBuilder::new(value).build();
                    output.insert(destination);
                },
                TX_SEQUENCE => {
                    println!("sequence");
                    let value = tx_json.sequence.unwrap();
                    let sequence = TxJsonSequenceBuilder::new(value).build();
                    output.insert(sequence);
                },

                TX_SIGNING_PUB_KEY => {
                    println!("public key");
                    let value = Rc::try_unwrap(tx_json).unwrap_err();
                    println!("--- {:#?}", value);
                    let value = String::from("0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020");
                    let signing_pubKey = TxJsonSigningPubKeyBuilder::new(value).build();
                    output.insert(signing_pubKey);
                    println!("inserrrrrrrrrrt public key");
                },
                TX_SIGNATURE => {
                    println!("siguration");
                    let value: TxJson = Rc::try_unwrap(tx_json).unwrap();
                    // let value = tx_json.txn_signature.unwrap();
                    let value = String::from("value.as_str()");
                    let txn_signature = TxJsonTxnSignatureBuilder::new(value).build();
                    output.insert(txn_signature);
                },

                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                }
            }
        }
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