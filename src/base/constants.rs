pub static ALPHABET: &[u8; 58] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";
pub const PASSWORD_LEN: usize = 16;

/// The order of the secp256k1 curve
// pub const CURVE_ORDER: [u8; 32] = [
pub const CURVE_ORDER: &[u8] = &[
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b,
    0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41
];

pub const CURVE_ZERO: &[u8] = &[ 0x0, 0x0, 0x0, 0x0 ];


pub type SignStreamType = Option<Vec<u8>>;

//Obj type
pub const  TX_SIGNATURE         : &'static str = "TxnSignature";
pub const  TX_DESTINATION       : &'static str = "Destination";
pub const  TX_ACCOUNT           : &'static str = "Account";
pub const  TX_SIGNING_PUB_KEY   : &'static str = "SigningPubKey";
pub const  TX_FEE               : &'static str = "Fee";
pub const  TX_AMOUNT            : &'static str = "Amount";
pub const  TX_SEQUENCE          : &'static str = "Sequence";
pub const  TX_TRANSACTION_TYPE  : &'static str = "TransactionType";
pub const  TX_FLAGS             : &'static str = "Flags";
pub const  TX_MEMOS             : &'static str = "Memos";
pub const  TX_MEMO              : &'static str = "Memo";
pub const  TX_MEMODATA          : &'static str = "MemoData";
pub const  TX_OFFER_SEQUENCE    : &'static str = "OfferSequence";
pub const  TX_LIMIT_AMOUNT      : &'static str = "LimitAmount";
pub const  TX_TARGET            : &'static str = "Target";
pub const  TX_RELATION_TYPE     : &'static str = "RelationType";

