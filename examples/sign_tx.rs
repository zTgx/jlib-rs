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
      "Sequence": 13, 
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
            "TxnSignature".to_string(),
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

        let stint8 = STInt8::new(18u8);
        let type_name = stint8.type_name_of();
        println!("typename : {}", type_name);
        let x = stint8.serialize();
        println!("x : {:?}", x);
        println!("x.typename : {}", x.type_name_of());

      
        use mylib::base::GLOBAL_MAP;
        let key_from_value = GLOBAL_MAP.get_key_from_value(100);
        println!("key from global map : {:?}", key_from_value);
    }
}

fn serialize(tx_json: TxJson, keys: &Vec<String>) {
  let mut so = vec![0u8];
  for key in keys {
    let key = key.as_str();

    let field_coordinates = INVERSE_FIELDS_MAP.get(key).unwrap();
    let type_bits = field_coordinates[0];
    let field_bits = field_coordinates[1];
    // let tag_byte = (type_bits < 16 ? type_bits << 4 : 0) | (field_bits < 16 ? field_bits : 0);
    let tag_byte: u8 = if type_bits < 16 { type_bits << 4 } else { 0 } | if field_bits < 16 { field_bits } else { 0 };
    println!("tag_byte: {}", tag_byte);

    // if ('string' === typeof value) {
    //   if (field_name === 'LedgerEntryType') {
    //       value = get_ledger_entry_type(value);
    //   } else if (field_name === 'TransactionResult') {
    //       value = get_transaction_type(value);//binformat.ter[value];
    //   }
    // }
    // let stint8 = STInt8::new(tag_byte);
    // let s = stint8.serialize(tag_byte);
    // so.push(s);

    // if (type_bits >= 16) {
    //     let s = stint8.serialize(type_bits);
    //     so.push(s);
    // }

    // if (field_bits >= 16) {
    //     let x = stint8.serialize(field_bits);
    //     so.push(s);
    // }

    //Trait object~~~~~~~~
    // let mut serialized_object_type = "".to_string();
    // if key == "Memos" {
    //     // for Memo we override the default behavior with our STMemo serializer
    //     serialized_object_type = exports.STMemo;
    // } else {
    //     // for a field based on the type bits.
    //     serialized_object_type = exports[TYPES_MAP[type_bits]];
    // }

  //   match x {
  //     "Flags" => {

  //     },
  //     "Fee" => {

  //     },

  //     "TransactionType" => {

  //     },

  //     "Account" => {

  //     },

  //     "Amount" => {

  //     },

  //     "Destination" => {

  //     },

  //     "Sequence" => {


  //     },
      
  //     "SigningPubKey" => {

  //     },
        
  //     "TxnSignature" => {

  //     },
      
  //     "Memos" => {

  //     },

  //     _ => {}
  // }
}




  

  // if ('string' === typeof value) {
  //       if (field_name === 'LedgerEntryType') {
  //           value = get_ledger_entry_type(value);
  //       } else if (field_name === 'TransactionResult') {
  //           value = get_transaction_type(value);//binformat.ter[value];
  //       }
  //   }


  //1.
  // STInt8.serialize(so, tag_byte);

  // //2.
  // if (type_bits >= 16) {
  //     STInt8.serialize(so, type_bits);
  // }

  // //3.
  // if (field_bits >= 16) {
  //     STInt8.serialize(so, field_bits);
  // }

  //4
  // Get the serializer class (ST...)
    // var serialized_object_type;
    // if (field_name === 'Memo' && typeof value === 'object') {
    //     // for Memo we override the default behavior with our STMemo serializer
    //     serialized_object_type = exports.STMemo;
    // } else {
    //     // for a field based on the type bits.
    //     serialized_object_type = exports[TYPES_MAP[type_bits]];
    // }

    // try {
    //     serialized_object_type.serialize(so, value);
    // } catch (e) {
    //     e.message += ' (' + field_name + ')';
    //     throw e;
    // }

}
