#![allow(unused)]
#![allow(non_snake_case)]

use std::fmt; //fmt METHOD
use serde::{Deserialize, Serialize};
// use serde::de::{self, Visitor, MapAccess};


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
    pub fn get(&self) -> u32 {
        match *self {
            //Universal
            Flags::Universal { ref name } => { match name {
                FullyCanonicalSig => { Universal::FullyCanonicalSig as u32 }
                _ => 0,
            }}

            //AccountSet
            Flags::AccountSet { ref name } => { match name {
                RequireDestTag  => { AccountSet::RequireDestTag   as u32 },
                OptionalDestTag => { AccountSet::OptionalDestTag  as u32 },
                RequireAuth     => { AccountSet::RequireAuth      as u32 },
                OptionalAuth    => { AccountSet::OptionalAuth     as u32 },
                DisallowSWT     => { AccountSet::DisallowSWT      as u32 },
                AllowSWT        => { AccountSet::AllowSWT         as u32 },
                _ => 0,
            }}

            //TrustSet
            Flags::TrustSet { ref name } => { match name {
                SetAuth         =>  { TrustSet::SetAuth         as u32 },
                ClearNoSkywell  =>  { TrustSet::ClearNoSkywell  as u32 },
                SetFreeze       =>  { TrustSet::SetFreeze       as u32 },
                ClearFreeze     =>  { TrustSet::ClearFreeze     as u32 },

                // NoSkywell       =>  { TrustSet::NoSkywell       as i32 },
                // SetNoSkywell    =>  { TrustSet::SetNoSkywell    as i32 },
                NoSkywell       =>  { 0x00020000    as u32 },
                SetNoSkywell    =>  { 0x00020000    as u32 },

                _ => 0,
            }}

            //OfferCreate
            Flags::OfferCreate { ref name } => { match name {
                Passive             =>  { OfferCreate::Passive              as u32 },
                ImmediateOrCancel   =>  { OfferCreate::ImmediateOrCancel    as u32 },
                FillOrKill          =>  { OfferCreate::FillOrKill           as u32 },
                Sell                =>  { OfferCreate::Sell                 as u32 },

                _ => 0,
            }}

            //Payment
            Flags::Payment { ref name } => { match name {
                NoSkywellDirect  =>  { Payment::NoSkywellDirect      as u32 },
                PartialPayment   =>  { Payment::PartialPayment       as u32 },
                LimitQuality     =>  { Payment::LimitQuality         as u32 },

                _ => 0,
            }}

            //RelationSet
            Flags::RelationSet { ref name } => { match name {
                Authorize   =>  { RelationSet::Authorize      as u32 },
                Freeze      =>  { RelationSet::Freeze         as u32 },

                _ => 0,
            }}

            Flags::Other => { 0 as u32 },

            _ => 0,
        }
    }
}
    // let fl = Flags::Universal{ name: Universal::FullyCanonicalSig };
    // let n = fl.get();
    // println!("n : {}", n);
//////////////////////////////Flags End



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