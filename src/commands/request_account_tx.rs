#![allow(unused)]

use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::any::Any;

use crate::commands::command_trait::CommandConversion;
use crate::misc::message::Amount;

/*
@4.12获得账号交易列表 
RequestAccountTxCommand 请求格式
id: u64,              //(固定值): 1
command: String,      //(固定值): account_tx
account: String,      //需要用户传递的参数，钱包的地址
ledger_index_min: i32 //(固定值): 0
ledger_index_max: i32 //(固定值): -1
limit: Option<u64>    //需要用户传递的参数，限定返回多少条记录，默认200
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTxCommand {
    #[serde(rename="id")]
    id: u64,

    #[serde(rename="command")]
    command: String,

    #[serde(rename="account")]
    account: String,

    #[serde(rename="ledger_index_min")]
    ledger_index_min: i32,

    #[serde(rename="ledger_index_max")]
    ledger_index_max: i32,

    #[serde(rename="limit")]
    limit: Option<u64>,
}

impl RequestAccountTxCommand {
    pub fn with_params(account: String, limit: Option<u64>) -> Box<Self> {
        let mut n = Some(200);
        if limit.is_some() {
            n = limit;
        }

        Box::new( 
            RequestAccountTxCommand {
                id: 1,
                command: "account_tx".to_string(),
                account: account,
                ledger_index_min: 0,
                ledger_index_max: -1,
                limit: n,
            }
        )
    }
}

impl CommandConversion for RequestAccountTxCommand {
    type T = RequestAccountTxCommand;
    fn to_string(&self) -> Result<String> {
        // let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
        // let compact = format!("{}", json);

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

    // fn to_concrete<T>(&self) -> T {
    //     let def: Box<dyn CommandConversion> = self;
    //     let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //         Some(b) => b,
    //         None => panic!("&a isn't a B!"),
    //     };
        
    //     b
    // }
}

//实现default方法, 此command不提供default方法~
// impl Default for RequestLedgerCommand {
//     fn default() -> Self {
//         RequestLedgerCommand { 
//             id: 1,
//             command: "ledger".to_string(),
//         }
//     }
// }

/////////////////////////
/*
RequestAccountTxResponse 数据返回格式
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Marker {
    #[serde(rename="ledger")]
    pub ledger: u64,

    #[serde(rename="seq")]
    pub seq: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FinalFields {

    #[serde(rename="Flags")]
    pub flags: Option<u64>,

    #[serde(rename="Owner")]
    pub owner: Option<String>,

    #[serde(rename="RootIndex")]
    pub root_index: Option<String>,

    #[serde(rename="Account")]
    pub account: Option<String>,

    #[serde(rename="Balance")]
    pub balance: Option<String>,

    #[serde(rename="OwnerCount")]
    pub owner_count: Option<u64>,

    #[serde(rename="Sequence")]
    pub sequence: Option<u64>,
    
    #[serde(rename="TakerGetsCurrency")]
    pub taker_gets_currency: Option<String>,

    #[serde(rename="TakerGetsIssuer")]
    pub taker_gets_issuer: Option<String>,

    #[serde(rename="TakerPaysCurrency")]
    pub taker_pays_currency: Option<String>,

    #[serde(rename="TakerPaysIssuer")]
    pub taker_pays_issuer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousFields {
    #[serde(rename="Balance")]
    pub balance: String,

    #[serde(rename="Sequence")]
    pub sequence: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tx {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Destination")]
    pub destination: String,

    #[serde(rename="Fee")]
    pub fee:  String,

    #[serde(rename="Flags")]
    pub flags: u64,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="date")]
    pub date: u64,
    
    #[serde(rename="hash")]
    pub hash: String,

    #[serde(rename="inLedger")]
    pub in_ledger: u64,

    #[serde(rename="ledger_index")]
    pub ledger_index: u64,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ModifiedNode {
    #[serde(rename="FinalFields")]
    pub final_fields: FinalFields,
    
    #[serde(rename="LedgerEntryType")]
    pub ledger_entry_type: String,

    #[serde(rename="LedgerIndex")]
    pub ledger_index: String,

    #[serde(rename="PreviousFields")]
    pub previous_fields: Option<PreviousFields>,

    #[serde(rename="PreviousTxnID")]
    pub previous_txn_id: Option<String>,

    #[serde(rename="PreviousTxnLgrSeq")]
    pub previous_txn_lgr_seq: Option<u64>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    #[serde(rename="AffectedNodes")]
    pub modified_node: Vec<AffectedNodes>,
    
    #[serde(rename="TransactionIndex")]
    pub transaction_index: u64,

    #[serde(rename="TransactionResult")]
    pub transaction_result: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AffectedNodes {
    #[serde(rename="ModifiedNode")]
    pub modified_node: Option<ModifiedNode>,
    
    #[serde(rename="CreatedNode")]
    pub create_node: Option<CreatedNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatedNode {
    #[serde(rename="LedgerEntryType")]
    pub ledger_entry_type: String, 
    
    #[serde(rename="LedgerIndex")]
    pub ledger_index: String, 
    
    #[serde(rename="NewFields")]
    pub new_field: NewFields, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewFields {
    #[serde(rename="Account")]
    account: String,
    
    #[serde(rename="BookDirectory")]
    book_directory: String,
    
    #[serde(rename="Flags")]
    flags: u64,
    
    #[serde(rename="Sequence")]
    sequence: u32,
    
    #[serde(rename="TakerGets")]
    taker_gets: Amount,
    
    #[serde(rename="TakerPays")]
    taker_pays: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename="meta")]
    pub meta: Meta,

    #[serde(rename="tx")]
    pub tx: Tx,

    #[serde(rename="validated")]
    pub validated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAccountTxResponse {
    #[serde(rename="account")]
    pub account: String,   

    #[serde(rename="ledger_index_max")]
    pub ledger_index_max: u64,

    #[serde(rename="ledger_index_min")]
    pub ledger_index_min: u64,

    #[serde(rename="marker")]
    pub marker: Marker, 

    #[serde(rename="limit")]
    pub limit: u64, 

    #[serde(rename="transactions")]
    pub transactions: Vec<Transaction>,
}