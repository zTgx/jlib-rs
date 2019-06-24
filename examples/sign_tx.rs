// extern crate mylib;
use serde_json::json;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use serde_json::Result;

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
        println!("tx_json : {:?}", x);

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
        println!("sorted : {:?}", keys);
    }


















}