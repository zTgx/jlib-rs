#![allow(unused)]
#![allow(non_snake_case)]

use std::fmt; //fmt METHOD
use serde::{Deserialize, Serialize};

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
    }
}

impl Flags {
    pub fn get(&self) -> u32 {
        match *self {
            //Universal
            Flags::Universal { ref name } => {
                 match name {
                     &Universal::FullyCanonicalSig => { Universal::FullyCanonicalSig as u32
                }
            }}

            //AccountSet
            Flags::AccountSet { ref name } => {
                match name {
                    &AccountSet::RequireDestTag  => { AccountSet::RequireDestTag   as u32 },
                    &AccountSet::OptionalDestTag => { AccountSet::OptionalDestTag  as u32 },
                    &AccountSet::RequireAuth     => { AccountSet::RequireAuth      as u32 },
                    &AccountSet::OptionalAuth    => { AccountSet::OptionalAuth     as u32 },
                    &AccountSet::DisallowSWT     => { AccountSet::DisallowSWT      as u32 },
                    &AccountSet::AllowSWT        => { AccountSet::AllowSWT         as u32 },
                }
            }

            //TrustSet
            Flags::TrustSet { ref name } => {
                match name {
                    &TrustSet::SetAuth         =>  { TrustSet::SetAuth         as u32 },
                    &TrustSet::ClearNoSkywell  =>  { TrustSet::ClearNoSkywell  as u32 },
                    &TrustSet::SetFreeze       =>  { TrustSet::SetFreeze       as u32 },
                    &TrustSet::ClearFreeze     =>  { TrustSet::ClearFreeze     as u32 },

                    // NoSkywell       =>  { TrustSet::NoSkywell       as i32 },
                    // SetNoSkywell    =>  { TrustSet::SetNoSkywell    as i32 },
                    &TrustSet::NoSkywell       =>  { 0x00020000    as u32 },
                    &TrustSet::SetNoSkywell    =>  { 0x00020000    as u32 },
                }
            }

            //OfferCreate
            Flags::OfferCreate { ref name } => {
                match name {
                    &OfferCreate::Passive             =>  { OfferCreate::Passive              as u32 },
                    &OfferCreate::ImmediateOrCancel   =>  { OfferCreate::ImmediateOrCancel    as u32 },
                    &OfferCreate::FillOrKill          =>  { OfferCreate::FillOrKill           as u32 },
                    &OfferCreate::Sell                =>  { OfferCreate::Sell                 as u32 },
                }
            }

            //Payment
            Flags::Payment { ref name } => {
                match name {
                    &Payment::NoSkywellDirect  =>  { Payment::NoSkywellDirect      as u32 },
                    &Payment::PartialPayment   =>  { Payment::PartialPayment       as u32 },
                    &Payment::LimitQuality     =>  { Payment::LimitQuality         as u32 },
                }
            }

            //RelationSet
            Flags::RelationSet { ref name } => {
                match name {
                    &RelationSet::Authorize   =>  { RelationSet::Authorize      as u32 },
                    &RelationSet::Freeze      =>  { RelationSet::Freeze         as u32 },
                }
            }

            Flags::Other => { 0 as u32 },

            _ => 0,
        }
    }
}
