
use std::fmt; //fmt METHOD
use serde::{Deserialize, Serialize};

use crate::transaction::TxJson;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Amount {
    #[serde(rename="currency")]
    pub currency: String,//'USD',

    #[serde(rename="value")]
    pub value: f64,   //0.5

    #[serde(rename="issuer")]
    pub issuer: String,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',
}

impl Amount {
    pub fn new(currency: String, value: f64, issuer: String) -> Self {
        Amount {
            currency: currency,
            value: value,
            issuer: issuer,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AmountTest {

    #[serde(rename="value")]
    pub value: String,   //0.5

    #[serde(rename="currency")]
    pub currency: String,//'USD',

    #[serde(rename="issuer")]
    pub issuer: String,  //'jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS',
}

impl AmountTest {
    pub fn new(currency: String, value: String, issuer: String) -> Self {
        AmountTest {
                        value: value,

            currency: currency,
            issuer: issuer,
        }
    }
}

////////////////////Flags 相关
// Universal flags can apply to any transaction type
#[derive(Serialize, Deserialize, Debug)]
pub enum Universal {
    FullyCanonicalSig = 0x80000000
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AccountSet {
    RequireDestTag  = 0x00010000,
    OptionalDestTag = 0x00020000,
    RequireAuth     = 0x00040000,
    OptionalAuth    = 0x00080000,
    DisallowSWT     = 0x00100000,
    AllowSWT        = 0x00200000,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TrustSet {
    SetAuth         =  0x00010000,
    ClearNoSkywell  =  0x00040000,
    SetFreeze       =  0x00100000,
    ClearFreeze     =  0x00200000,

    // NoSkywell       =  0x00020000,
    // SetNoSkywell    =  0x00020000,
    
    NoSkywell,
    SetNoSkywell,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OfferCreate {
    Passive             =  0x00010000,
    ImmediateOrCancel   =  0x00020000,
    FillOrKill          =  0x00040000,
    Sell                =  0x00080000,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Payment {
    NoSkywellDirect  =    0x00010000,
    PartialPayment   =    0x00020000,
    LimitQuality     =    0x00040000,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RelationSet {
    Authorize   =    0x00000001,
    Freeze      =    0x00000011,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Flags { 
    Universal  { name: Universal   },
    AccountSet { name: AccountSet  },
    TrustSet   { name: TrustSet    },
    OfferCreate{ name: OfferCreate },
    Payment    { name: Payment     },
    RelationSet{ name: RelationSet },
    Other,
}

//Get enum as string
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl Flags {
    pub fn get(&self) -> i32 {
        match *self {
            //Universal
            Flags::Universal { ref name } => { match name {
                FullyCanonicalSig => { Universal::FullyCanonicalSig as i32 }
            }}

            //AccountSet
            Flags::AccountSet { ref name } => { match name {
                RequireDestTag  => { AccountSet::RequireDestTag   as i32 },
                OptionalDestTag => { AccountSet::OptionalDestTag  as i32 },
                RequireAuth     => { AccountSet::RequireAuth      as i32 },
                OptionalAuth    => { AccountSet::OptionalAuth     as i32 },
                DisallowSWT     => { AccountSet::DisallowSWT      as i32 },
                AllowSWT        => { AccountSet::AllowSWT         as i32 },
            }}

            //TrustSet
            Flags::TrustSet { ref name } => { match name {
                SetAuth         =>  { TrustSet::SetAuth         as i32 },
                ClearNoSkywell  =>  { TrustSet::ClearNoSkywell  as i32 },
                SetFreeze       =>  { TrustSet::SetFreeze       as i32 },
                ClearFreeze     =>  { TrustSet::ClearFreeze     as i32 },

                // NoSkywell       =>  { TrustSet::NoSkywell       as i32 },
                // SetNoSkywell    =>  { TrustSet::SetNoSkywell    as i32 },
                NoSkywell       =>  { 0x00020000    as i32 },
                SetNoSkywell    =>  { 0x00020000    as i32 },
            }}

            //OfferCreate
            Flags::OfferCreate { ref name } => { match name {
                Passive             =>  { OfferCreate::Passive              as i32 },
                ImmediateOrCancel   =>  { OfferCreate::ImmediateOrCancel    as i32 },
                FillOrKill          =>  { OfferCreate::FillOrKill           as i32 },
                Sell                =>  { OfferCreate::Sell                 as i32 },
            }}

            //Payment
            Flags::Payment { ref name } => { match name {
                NoSkywellDirect  =>  { Payment::NoSkywellDirect      as i32 },
                PartialPayment   =>  { Payment::PartialPayment       as i32 },
                LimitQuality     =>  { Payment::LimitQuality         as i32 },
            }}

            //RelationSet
            Flags::RelationSet { ref name } => { match name {
                Authorize   =>  { RelationSet::Authorize      as i32 },
                Freeze      =>  { RelationSet::Freeze         as i32 },
            }}

            Flags::Other => { 0 },
        }
    }
}
    // let fl = Flags::Universal{ name: Universal::FullyCanonicalSig };
    // let n = fl.get();
    // println!("n : {}", n);
//////////////////////////////Flags End

/*
工具方法
*/
pub fn string_to_hex(s: &String) -> String {
    let mut ss = String::from("");
    for x in s.as_bytes() {
        let hexs = format!("{:x}", x);
        ss.push_str(&hexs);
    }
    
    ss
}

// fn sigining(tx_json: &TxJson) -> TxJson {

// }

// //Reference tx_json, and generate one signed tx_json.
// pub fn sign(tx_json: &TxJson) -> TxJson {
//     if tx_json.sequence.is_some() {
//         signing(self, callback);
//     } else {
//         //Get Account Sequence
//         Remote::request_account_info(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), |x| match x {
//             Ok(response) => {
//                 println!("Sequence: {}", response.Sequence);

//                 self.tx_json.Sequence = data.account_data.Sequence;
//                 signing(self, callback)
//             },

//             Err(_) => {}
//         });
//     }
// }