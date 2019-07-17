
use serde::{Deserialize, Serialize};
use crate::misc::message::Amount;

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