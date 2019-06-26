#[macro_use] 
extern crate typename;

extern crate mylib;

use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;

use mylib::base::inverse_fields_map::INVERSE_FIELDS_MAP;
use mylib::base::types_map::TYPES_MAP;
use mylib::base::serialized_type::*;

use typename::TypeName;
use mylib::base::amount::*;

// use mylib::transaction::TxJson;
#[derive(Serialize, Deserialize, Debug)]
pub struct TxJson {
    #[serde(rename="Flags")]
    pub flags: i32,

    #[serde(rename="Fee")]
    pub fee: f64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: f64,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pubKey: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="Memos")]
    pub memo: Option<Vec<String>>,  
}

fn main() {
    //var prefix = 0x53545800;
    // var hash = jser.from_json(self.tx_json).hash(prefix);

    // var signed_content = wt.signTx(hash);
    // console.log("content result : ", signed_content);
    // self.tx_json.TxnSignature = signed_content;


    let from_json = r#"
    { 
      "Flags": 0, 
      "Fee": 0.01, 
      "TransactionType": "Payment", 
      "Account": "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh", 
      "Amount": 0.5,
      "Destination": "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c", 
      "Sequence": 59, 
      "SigningPubKey":"0330E7FC9D56BB25D6893BA3F317AE5BCF33B3291BD63DB32654A313222F7FD020",
      "TxnSignature":"33333333"
      }"#;

    // println!("seride : {:?}", from_json);

    use serde_json::Value;
    if let Ok(x) = serde_json::from_str(&from_json) as Result<TxJson> {
        // println!("tx_json : {:?}", &x);

        //step 2 keys = sort_fields(keys);
        let mut keys = vec![
            "Flags".to_string(),
            "Fee".to_string(),
            "TransactionType".to_string(),
            "Account".to_string(),
            "Amount".to_string(),
            "Destination".to_string(),
            "Sequence".to_string(),
            "SigningPubKey".to_string(),
            ];

        use mylib::base::inverse_fields_map::INVERSE_FIELDS_MAP;
        keys.sort_by( |a, b| {
                                
            let a_field_coordinates = INVERSE_FIELDS_MAP.get(a.as_str()).unwrap();
            let a_type_bits = a_field_coordinates[0];
            let a_field_bits = a_field_coordinates[1];

            let b_field_coordinates = INVERSE_FIELDS_MAP.get(b.as_str()).unwrap();
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
        // println!("sorted : {:?}", keys);


        //serialize
        serialize(x, &keys);


      
    }
}

fn serialize(tx_json: TxJson, keys: &Vec<String>) {
  let mut so: Vec<u8> = vec![];
  for key in keys {
      let key = key.as_str();

      let field_coordinates = INVERSE_FIELDS_MAP.get(key).unwrap();
      let type_bits  = field_coordinates[0];
      let field_bits = field_coordinates[1];
      // let tag_byte = (type_bits < 16 ? type_bits << 4 : 0) | (field_bits < 16 ? field_bits : 0);
      let left = if type_bits < 16 { type_bits << 4 } else { 0 };
      let right = if field_bits < 16 { field_bits } else { 0 };
      let tag_byte: u8 = left | right;
      
      println!("type_name: {} / type_bits : {} / tag_byte: {}", key, type_bits, tag_byte);

      // if ('string' === typeof value) {
      //   if (field_name === 'LedgerEntryType') {
      //       value = get_ledger_entry_type(value);
      //   } else if (field_name === 'TransactionResult') {
      //       value = get_transaction_type(value);//binformat.ter[value];
      //   }
      // }

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

      //Trait object~~~~~~~~
      let mut serialized_object_type = "".to_string();
      if key == "Memos" {
          // for Memo we override the default behavior with our STMemo serializer
          // serialized_object_type = exports.STMemo;
      } else {
          // for a field based on the type bits.
          // serialized_object_type = TYPES_MAP[type_bits]].to_string();
      }

      // let mut y = STInt8::serialize()
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
            let value = tx_json.sequence;
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
            let value = "".to_owned() + tx_json.signing_pubKey.as_str();
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
            let value = "".to_owned() + tx_json.txn_signature.as_str();
            println!("value : {}", value);

            serialized_object_type = TYPES_MAP[type_bits as usize].to_string();
            if serialized_object_type.as_str() == "VL" {
            
                let mut s = STVL::serialize(value);
                so.append(&mut s);
                println!("TxnSignature : {:?}", &so);

                return;
            }
        },

        _ => {}
      }      
    }
}
