
// #[macro_use]
// extern crate typename;

pub mod sign;
pub mod brorand;
pub mod util;
pub mod constants;
pub mod base_data;
pub mod wallet;
pub mod config;
pub mod seed;
pub mod keypair;
pub mod address;

pub mod inverse_fields_map;
pub mod types_map;
pub mod serialized_type;

pub mod amount;
pub mod sign_tx;
pub mod type_obj;
pub mod signed_obj;

//Serialize
use std::collections::HashMap;

//针对key和value都是唯一的HashMap方法
pub trait TWHashMap {
    fn get_value_from_key(&self, s: &str) -> Option<&i32>;
    fn get_key_from_value(&self, i: i32) -> Option<&'static str>;
}

/*
 * return the transaction type in string
 * Data defined in the TRANSACTION_TYPES
*/
pub struct TransactionTypeMap {
    pub m: HashMap<&'static str, i32>,
}
impl TransactionTypeMap {
    pub fn new() -> Self {
        let map: HashMap<&'static str, i32> =
                                [
                                    ("Payment"         , 0),
                                    ("AccountSet"      , 3),
                                    ("SetRegularKey"   , 5),
                                    ("OfferCreate"     , 7),
                                    ("OfferCancel"     , 8),
                                    ("Contract"        , 9),
                                    ("RemoveContract"  , 10),
                                    ("TrustSet"        , 20),
                                    ("RelationSet"     , 21),
                                    ("RelationDel"     , 22),
                                    ("ConfigContract"  , 30),
                                    ("EnableFeature"   , 100),
                                    ("SetFee"          , 101)
                                ]
                                .iter().cloned().collect();

        TransactionTypeMap {
            m: map,
        }
    }
}
impl TWHashMap for TransactionTypeMap {
    fn get_value_from_key(&self, key: &str) -> Option<&i32> {
        self.m.get(key)
    }

    fn get_key_from_value(&self, value: i32) -> Option<&'static str> {
        let mut k = None;
        for (key, val) in self.m.iter() {

            if *val == value {
                k = Some(*key);

                return k;
            }
        }

        k
    }
}


/*
 * return the transaction result in string
 * Data defined in the TRANSACTION_RESULTS
 *  tesSUCCESS               : 0,
  tecCLAIM                 : 100,
  tecPATH_PARTIAL          : 101,
  tecUNFUNDED_ADD          : 102,
  tecUNFUNDED_OFFER        : 103,
  tecUNFUNDED_PAYMENT      : 104,
  tecFAILED_PROCESSING     : 105,
  tecDIR_FULL              : 121,
  tecINSUF_RESERVE_LINE    : 122,
  tecINSUF_RESERVE_OFFER   : 123,
  tecNO_DST                : 124,
  tecNO_DST_INSUF_SWT      : 125,
  tecNO_LINE_INSUF_RESERVE : 126,
  tecNO_LINE_REDUNDANT     : 127,
  tecPATH_DRY              : 128,
  tecMASTER_DISABLED       : 130,
  tecNO_REGULAR_KEY        : 131,
  tecOWNERS                : 132,
  tecNO_ISSUER             : 133,
  tecNO_AUTH               : 134,
  tecNO_LINE               : 135,
  tecINSUFF_FEE            : 136,
  tecFROZEN                : 137,
  tecNO_TARGET             : 138,
  tecNO_PERMISSION         : 139,
  tecNO_ENTRY              : 140,
  tecINSUFFICIENT_RESERVE  : 141
*/
#[derive(Debug, Default)]
pub struct TransactionResultMap {
    pub m: HashMap<&'static str, i32>,
}
impl TransactionResultMap {
    pub fn new() -> Self {
        let map: HashMap<&'static str, i32> = [
                                        ("tesSUCCESS"                , 0  ),
                                        ("tecCLAIM"                  , 100),
                                        ("tecPATH_PARTIAL"           , 101),
                                        ("tecUNFUNDED_ADD"           , 102),
                                        ("tecUNFUNDED_OFFER"         , 103),
                                        ("tecUNFUNDED_PAYMENT"       , 104),
                                        ("tecFAILED_PROCESSING"      , 105),
                                        ("tecDIR_FULL"               , 121),
                                        ("tecINSUF_RESERVE_LINE"     , 122),
                                        ("tecINSUFFICIENT_RESERVE"   , 141),
                                    ].iter().cloned().collect();

        TransactionResultMap {
            m: map,
        }
    }
}
impl TWHashMap for TransactionResultMap {
    fn get_value_from_key(&self, key: &str) -> Option<&i32> {
        self.m.get(key)
    }

    fn get_key_from_value(&self, value: i32) -> Option<&'static str> {
        let mut k = None;
        for (key, val) in self.m.iter() {
            if *val == value {
                k = Some(*key);

                return k;
            }
        }

        k
    }
}


/*
 * return the transaction type in string
 * Data defined in the ledger entry:
  AccountRoot: [97].concat(sleBase,[
  Contract: [99].concat(sleBase,[
  DirectoryNode: [100].concat(sleBase,[
  EnabledFeatures: [102].concat(sleBase,[
  FeeSettings: [115].concat(sleBase,[
  GeneratorMap: [103].concat(sleBase,[
  LedgerHashes: [104].concat(sleBase,[
  Nickname: [110].concat(sleBase,[
  Offer: [111].concat(sleBase,[
  SkywellState: [114].concat(sleBase,[

  TODO: add string input handles
*/
#[derive(Debug, Default)]
pub struct LedgerEntryTypeMap {
    pub m: HashMap<&'static str, i32>,
}
impl LedgerEntryTypeMap {
    pub fn new() -> Self {
        let map: HashMap<&'static str, i32> =
                                    [
                                        ("AccountRoot"      , 97),
                                        ("Contract"         , 99),
                                        ("DirectoryNode"    , 100),
                                        ("EnabledFeatures"  , 102),
                                        ("FeeSettings"      , 115),
                                        ("GeneratorMap"     , 103),
                                        ("LedgerHashes"     , 104),
                                        ("Nickname"         , 110),
                                        ("Offer"            , 111),
                                        ("SkywellState"     , 114),
                                    ]
                                    .iter().cloned().collect();

        LedgerEntryTypeMap {
            m: map,
        }
    }
}
impl TWHashMap for LedgerEntryTypeMap {
    fn get_value_from_key(&self, key: &str) -> Option<&i32> {
        self.m.get(key)
    }

    fn get_key_from_value(&self, value: i32) -> Option<&'static str> {
        let mut k = None;
        for (key, val) in self.m.iter() {
            if *val == value {
                k = Some(*key);

                return k;
            }
        }

        k
    }
}

lazy_static! {
    pub static ref G_TRANSACTION_TYPE_MAP: TransactionTypeMap = {
        let map = TransactionTypeMap::new();
        map
    };

    pub static ref G_TRANSACTION_RESULT_MAP: TransactionResultMap = {
        let map = TransactionResultMap::new();
        map
    };

    pub static ref G_LEDGER_ENTRY_TYPE_MAP: LedgerEntryTypeMap = {
        let map = LedgerEntryTypeMap::new();
        map
    };
}
