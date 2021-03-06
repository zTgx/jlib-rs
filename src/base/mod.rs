extern crate secp256k1;
extern crate crypto;
extern crate ring;

pub mod data;
pub mod local_sign;
pub mod misc;
pub mod serialize;
pub mod wallet;
pub mod curve;
pub mod xcodec;
pub mod base_config;
pub mod parse_tx;

pub mod seed;

//Serialize
use std::collections::HashMap;

pub trait TWHashMap {
    fn get_value_from_key(&self, s: &str) -> Option<&u16>;
    fn get_key_from_value(&self, i: u16) -> Option<&'static str>;
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

/*
 * return the transaction type in string
 * Data defined in the TRANSACTION_TYPES
*/
pub struct TransactionTypeMap {
    pub m: HashMap<&'static str, u16>,
}
impl TransactionTypeMap {
    pub fn new() -> Self {
        let map: HashMap<&'static str, u16> =
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
                                    ("SetFee"          , 101),
                                    ("Brokerage"       , 205),
                                ]
                                .iter().cloned().collect();

        TransactionTypeMap {
            m: map,
        }
    }
}
impl TWHashMap for TransactionTypeMap {
    fn get_value_from_key(&self, key: &str) -> Option<&u16> {
        self.m.get(key)
    }

    fn get_key_from_value(&self, value: u16) -> Option<&'static str> {
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

#[derive(Debug, Default)]
pub struct TransactionResultMap {
    pub m: HashMap<&'static str, u16>,
}
impl TransactionResultMap {
    pub fn new() -> Self {
        let map: HashMap<&'static str, u16> = [
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
    fn get_value_from_key(&self, key: &str) -> Option<&u16> {
        self.m.get(key)
    }

    fn get_key_from_value(&self, value: u16) -> Option<&'static str> {
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

#[derive(Debug, Default)]
pub struct LedgerEntryTypeMap {
    pub m: HashMap<&'static str, u16>,
}
impl LedgerEntryTypeMap {
    pub fn new() -> Self {
        let map: HashMap<&'static str, u16> =
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
    fn get_value_from_key(&self, key: &str) -> Option<&u16> {
        self.m.get(key)
    }

    fn get_key_from_value(&self, value: u16) -> Option<&'static str> {
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
