
/*
Deploy paras:
send message:  { id: 1,
  command: 'submit',
  secret: 'snoPBjXtMeMyMHUVTgbuqAfg1SUTb',
  tx_json:
   { Flags: 0,
     Fee: 10000,
     TransactionType: 'AlethContract',
     Account: 'jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh',
     Amount: 100000000,
     Method: 0,
     Payload:
      '363038303630343035323334383031353630306635373630303038306664356235303630393238303631303031653630303033393
      6303030663366653630383036303430353233343830313536303066353736303030383066643562353036303034333631303630343
      4353737633031303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303
      0303030303030303030363030303335303436336236653435366262383131343630343935373562363030303830666435623630346
      6363036313536356236303430383035313931383235323531393038313930303336303230303139306633356236303231393035366
      6656131363536323761376137323330353832303834373030326437366537623336636234306661396138383432663534303765616
      439343662386339633665653638323539633333383263346236303361383330303239' } }


InVoke params;
send message:  { id: 1,
  command: 'submit',
  secret: 'snoPBjXtMeMyMHUVTgbuqAfg1SUTb',
  tx_json:
   { Flags: 0,
     Fee: 10000,
     TransactionType: 'AlethContract',
     Account: 'jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh',
     Method: 1,
     Destination: 'jsQZVJXd3dvf9y1Lmghu3EAiNKgbpTAaKv',
     Amount: 0,
     Args: [ [Object] ] } }
*/


#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::commands::command_trait::CommandConversion;
use crate::transactions::transaction::*;

//////////////////////
/*
@合约部署请求
SolidityInitMessage 请求格式
id      : u64
command : String
secret  : String,
tx_json : TxJson Object
值分别为(固定值): 1, "submit"
*/
#[derive(Serialize, Deserialize)]
pub struct SolidityInitMessage {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="secret")]
    secret: String,

    #[serde(rename="tx_json")]
    tx_json: TxJson,
}

impl SolidityInitMessage {
    pub fn new(secret: String, tx_json: TxJson) -> Self {
        SolidityInitMessage {
          id: 1,
          command: "submit".to_string(),
          secret: secret,
          tx_json: tx_json,
        }
    }
}

impl CommandConversion for SolidityInitMessage {
    type T = SolidityInitMessage;
    fn to_string(&self) -> Result<String> {
        //https://crates.io/crates/serde_json
        // Serialize it to a JSON string.
        let j = serde_json::to_string(&self)?;

        // Print, write to a file, or send to an HTTP server.
        println!("{}", j);

        Ok(j)
    }
    
    fn box_to_raw(&self) -> &dyn Any {
        self
    }
}

pub trait ContractAPI {
    fn deploy(&self);
    fn invoke(&self);
}

pub struct Solidity {
}

impl ContractAPI for Solidity {
    fn deploy(&self) {

    }

    fn invoke(&self) {

    }
}
