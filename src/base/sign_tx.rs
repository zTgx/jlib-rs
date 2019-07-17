// #[macro_use] 
// extern crate typename;

// use serde_json::json;
// use serde_json::{Value};
// use serde::{Serialize};
// use serde_json::Result;

use crate::base::inverse_fields_map::INVERSE_FIELDS_MAP;
use crate::base::sign::*;
use ring::{digest};

use crate::message::transaction::transaction::{TxJson};
use crate::base::signed_obj::*;

use crate::base::constants::{
    TX_DESTINATION, TX_ACCOUNT, TX_SIGNING_PUB_KEY, TX_FEE, 
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE,TX_FLAGS,SignStreamType
};
use std::rc::Rc;
use crate::base::*;

const PRE_FIELDS: [&'static str; 7] = ["Flags", "Fee", "TransactionType", "Account", "Amount", "Destination", "Sequence"];

//等待数据去填充
pub struct SignTx {
    pub fields: Vec<&'static str>,
    pub secret: String,

    pub so_flags           : SignStreamType,
    pub so_fee             : SignStreamType,
    pub so_transaction_type: SignStreamType,
    pub so_account         : SignStreamType,
    pub so_amount          : SignStreamType,
    pub so_destination     : SignStreamType,
    pub so_sequence        : SignStreamType,
    pub so_public_key      : SignStreamType,
    pub so_txn             : SignStreamType,
    pub so_memo            : SignStreamType,
    pub so_blob            : SignStreamType,
}
impl Default for SignTx {
    fn default() -> Self {
        SignTx {
            fields: vec![],
            secret: "".to_string(),

            so_flags: None,
            so_fee: None,
            so_transaction_type: None,
            so_account: None,
            so_amount: None,
            so_destination: None,
            so_sequence: None,
            so_public_key: None,
            so_txn: None,
            so_memo: None,
            so_blob: None,
        }
    }
}
impl SignTx {
    pub fn prepare(&mut self, tx_json: TxJson, secret: String) -> Option<String> {
        self.secret = secret;

        //Step 1: Get Non-None field. [SigningPubKey] / [TxnSignature] / [Memos]
        // let mut fields: Vec<&str> = vec![];
        self.fields.extend_from_slice(&PRE_FIELDS);
        if tx_json.memo.is_some() {
            self.fields.push("Memos");
        }
        if tx_json.signing_pub_key.is_some() {
            self.fields.push("SigningPubKey");
        }
        if tx_json.txn_signature.is_some() {
            self.fields.push("TxnSignature");
        }

        //Step 2: sorted
        SignTx::sort_fields(&mut self.fields);

        //Step 3: serialize tx fields
        let mut tx: SignedTxJson = SignedTxJson::new();
        self.format_tx_obj(tx_json, &mut tx, &self.fields);

        self.calc_txn_sig(&mut tx);
        
        //Step 5 : blob
        let outp = self.calc_blob(&mut tx);
        println!("output blob: {:?}", outp);

        outp
    }


    pub fn calc_txn_sig(&mut self, signed_tx_json: &mut SignedTxJson) {
        //Step 4 : serialize
        let output: Vec<u8> = signed_tx_json.serialize();
        let txn_signature= self.update_txn_signature(&output);
        println!("txn_signature: {}", txn_signature);

        self.fields.push("TxnSignature");
        SignTx::sort_fields(&mut self.fields);
        let mut index = 0;
        for x in &self.fields {
            if *x == "TxnSignature" {
                break;
            }

            index += 1;
        }
        let txn_signature = TxJsonTxnSignatureBuilder::new(txn_signature).build();
        signed_tx_json.insert(index, txn_signature);
    }


    pub fn calc_blob(&self, signed_tx_json: &mut SignedTxJson) -> Option<String> {
        let output: Vec<u8> = signed_tx_json.serialize();
        Some(hex::encode(&output).to_ascii_uppercase())
    }  

    //Output Hex String
    pub fn update_txn_signature(&self, so: &Vec<u8>) -> String {
        let mut ctx = digest::Context::new(&digest::SHA512);
        ctx.update(&[83,84,88, 0]);
        ctx.update(&so);

        let hash = hex::encode(&ctx.finish().as_ref());
        let message = hash.get(0..64).unwrap().to_ascii_uppercase();
        
        //  let key = [26, 202, 174, 222, 206, 64, 91, 42, 149, 130, 18, 98, 158, 22, 242, 235, 70, 177, 83, 238, 233, 76, 221, 53, 15, 222, 255, 82, 121, 85, 37, 183];
        let private_key = util::get_public_key_from_secret(&self.secret).property.secret_key;
        let key = &hex::decode(private_key).unwrap()[1..];
        // println!("key : {:?}", key);
        let msg = hex::decode(message).unwrap();
        let signed_hex_string = SignatureX::sign(&msg, &key);
        return signed_hex_string;
    }

    pub fn format_tx_obj(&self, tx_json: TxJson, output: &mut SignedTxJson, fields: &Vec<&str>) {
        let tx_json_rc = Rc::new ( tx_json );

        let mut index = 0;
        for &key in fields {
            let tx_json = tx_json_rc.clone();
            println!("key : {} ", key);
            match key {
                TX_FLAGS => {
                    let value = tx_json.flags;
                    let flags = TxJsonFlagsBuilder::new(value).build();
                    output.insert(index, flags);
                },
                TX_FEE => {
                    let value = tx_json.fee;
                    let fee = TxJsonFeeBuilder::new(value.to_string()).build();
                    output.insert(index, fee);
                },
                TX_TRANSACTION_TYPE => {
                    //TRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPESTRANSACTION_TYPES
                    let value = 0u16;//tx_json.transaction_type;
                    let transaction_type = TxJsonTransactionTypeBuilder::new(value).build();
                    output.insert(index, transaction_type);
                },
                TX_ACCOUNT => {
                    let value = String::from(tx_json.account.as_str());
                    let account = TxJsonAccountBuilder::new(value).build();
                    output.insert(index, account);
                },
                TX_AMOUNT => {
                    let value = String::from(tx_json.amount.as_str());
                    let amount = TxJsonAmountBuilder::new(value).build();
                    output.insert(index, amount);
                },
                TX_DESTINATION => {
                    let value = String::from(tx_json.destination.as_str());
                    let destination = TxJsonDestinationBuilder::new(value).build();
                    output.insert(index, destination);
                },
                TX_SEQUENCE => {
                    let value = tx_json.sequence.unwrap();
                    let sequence = TxJsonSequenceBuilder::new(value).build();
                    output.insert(index, sequence);
                },

                TX_SIGNING_PUB_KEY => {
                    let _value = Rc::try_unwrap(tx_json).unwrap_err();
                    let value = util::get_public_key_from_secret(&self.secret).property.public_key;
                    // let value = String::from("0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020");
                    let signing_pub_key = TxJsonSigningPubKeyBuilder::new(value).build();
                    output.insert(index, signing_pub_key);
                },
                // TX_SIGNATURE => {
                //     let value: TxJson = Rc::try_unwrap(tx_json).unwrap();
                //     // let value = tx_json.txn_signature.unwrap();
                //     let value = String::from("3045022100A1625CA0BCB7EA68E1FCEF86209E2278AD334FFEB4E58A31739D028D868D2E93022011D90FC264BD8128C383A1F2596AFDBBB5104DBCD15BE8CB678A83447C97E6C1");
                //     let txn_signature = TxJsonTxnSignatureBuilder::new(value).build();
                //     output.insert(index, txn_signature);
                // },

                _ => {
                    panic!("pppppppppppppppppppppppnic.................");
                }
            }

            index += 1;
        }
    }

    //Refactor..........compare...!!!!!!!!
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