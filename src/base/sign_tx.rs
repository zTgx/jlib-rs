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

        //Step 3: serialize tx fields
        let mut so = SignTx::serialize_tx_fields(&tx_json, &fields);
    }

    //Hex String sign(tx_json) => blob
    pub fn serialize_all_fields(so: &Vec<u8>) -> String {
        hex::encode(&so).to_ascii_uppercase()
    }

    //Output Hex String
    pub fn update_txn_signature(so: &Vec<u8>) -> String {
        let mut ctx = digest::Context::new(&digest::SHA512);
        // let prefix = hex::decode("53545800").unwrap();
        ctx.update(&[83,84,88, 0]);
        ctx.update(&so);

        let mut hash = hex::encode(&ctx.finish().as_ref());
        let mut message = hash.get(0..64).unwrap().to_ascii_uppercase();
        
        let key = [26, 202, 174, 222, 206, 64, 91, 42, 149, 130, 18, 98, 158, 22, 242, 235, 70, 177, 83, 238, 233, 76, 221, 53, 15, 222, 255, 82, 121, 85, 37, 183];

        let msg = hex::decode(message).unwrap();
        let mut signed_hex_string = SignatureX::sign(&msg, &key);
        return signed_hex_string;
    }

    pub fn serialize_tx_fields(tx_json: &TxJson, fields: &Vec<&str>) -> Vec<u8> {
        let mut so: Vec<u8> = vec![];
        for &key in fields {
            // let key = key.as_str();

            let field_coordinates = INVERSE_FIELDS_MAP.get(key).unwrap();
            let type_bits  = field_coordinates[0];
            let field_bits = field_coordinates[1];
            let left = if type_bits < 16 { type_bits << 4 } else { 0 };
            let right = if field_bits < 16 { field_bits } else { 0 };
            let tag_byte: u8 = left | right;

            let mut s8 = STInt8::serialize(tag_byte);
            so.append(&mut s8);

            if (type_bits >= 16) {
                let mut s = STInt8::serialize(type_bits);
                so.append(&mut s);
            }

            if (field_bits >= 16) {
                let mut x = STInt8::serialize(field_bits);
                so.append(&mut x);
            }

            let mut serialized_object_type = "".to_string();
            match key {
                "TransactionType" => {
                    let value = tx_json.flags;
                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "Int16" {
                        let mut s = STInt16::serialize(value as u16);
                        so.append(&mut s);

                        println!("so : {:?}", &so);
                    }
                },

                "Flags" => {
                let value = tx_json.flags;
                serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                if serialized_object_type.as_str() == "Int32" {
                    let mut s = STInt32::serialize(value as u32);
                    so.append(&mut s);

                    println!("so : {:?}", &so);
                    }
                },

                "Sequence" => {
                    let value = tx_json.sequence.unwrap();
                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "Int32" {
                        let mut s = STInt32::serialize(value as u32);
                        so.append(&mut s);

                        println!("so : {:?}", &so);
                    }
                },

                "Amount" => {

                    let value = tx_json.amount;
                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "Amount" {
                        println!("raw value : {}", value);
                        let amount = Amount::from_json(value.to_string());
                        let mut s = STAmount::serialize(amount);
                        so.append(&mut s);
                        println!("so : {:?}", &so);
                    }
                },

                "Fee" => {
                    let value = tx_json.fee;
                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "Amount" {
                    
                        println!("raw value : {}", value);
                        let amount = Amount::from_json(value.to_string());
                        let mut s = STAmount::serialize(amount);
                        so.append(&mut s);
                        println!("so : {:?}", &so);
                    }
                },

                "SigningPubKey" => {
                    let value = "".to_owned() + tx_json.signing_pubKey.unwrap().as_str();
                    println!("signing pub key : {}", value);

                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "VL" {
                    
                        let mut s = STVL::serialize(value);
                        so.append(&mut s);
                        println!("so : {:?}", &so);
                    }
                },

                "Account" => {
                    let value = "".to_owned() + tx_json.account.as_str();
                    println!("value : {}", value);

                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "Account" {
                    
                        let mut s = STAccount::serialize(value);
                        so.append(&mut s);
                        println!("Account : {:?}", &so);
                    }
                },

                "Destination" => {
                    let value = "".to_owned() + tx_json.destination.as_str();
                    println!("value : {}", value);

                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "Account" {
                    
                        let mut s = STAccount::serialize(value);
                        so.append(&mut s);
                        println!("Account : {:?}", &so);

                    }
                },

                "TxnSignature" => {
                    let value = "".to_owned() + tx_json.txn_signature.unwrap().as_str();
                    println!("TxnSignature value : {}", value);

                    serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
                    if serialized_object_type.as_str() == "VL" {
                    
                        let mut s = STVL::serialize(value);
                        so.append(&mut s);
                        println!("TxnSignature : {:?}", &so);

                    }
                },

                _ => {}
            }      
        }

        so
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