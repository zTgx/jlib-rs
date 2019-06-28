//
//
// 说明：
// 在序列化raw tx_json的过程中，通过把TxJson对象转化成待signed的特殊结构（SignedTxJson）来进行真正的序列化操作，
// SignedTxJson中包括了各个字段组成的对象如：TxJsonFlags等，这些对象都实现了TxJsonSerialize trait, (trait object).
//
//

use crate::base::type_obj::*;
use crate::base::constants::{
    TX_SIGNATURE, TX_DESTINATION, TX_ACCOUNT, TX_SIGNING_PUB_KEY, TX_FEE, 
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE,TX_FLAGS
};
use crate::base::serialized_type::*;
use crate::base::amount::*;

//序列化接口
pub trait TxJsonSerializer {
    fn serialize_obj(&self, so: &mut Vec<u8>);
}

//builder接口
pub trait TxJsonBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer>;
}



///Flags
pub struct TxJsonFlags {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : u32,
}
impl TxJsonFlags {
    pub fn new(value: u32) -> Self {
        TxJsonFlags {
            name    : TX_FLAGS.to_string(),
            typeObj : TypeObjBuilder::new(TX_FLAGS).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonFlags {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
        //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STInt32::serialize(self.value);
        so.append(&mut s);

        println!("TxJsonFlags so : {:?}", &so);
    }
}
pub struct TxJsonFlagsBuilder {
    pub value: u32,
}
impl TxJsonFlagsBuilder {
    pub fn new(value: u32) -> Self {
        TxJsonFlagsBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonFlagsBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonFlags::new(self.value) )
    }
}

//TransactionType
pub struct TxJsonTransactionType {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : u16,
}

impl TxJsonTransactionType {
    pub fn new(value: u16) -> Self {
        TxJsonTransactionType {
            name    : TX_TRANSACTION_TYPE.to_string(),
            typeObj : TypeObjBuilder::new(TX_TRANSACTION_TYPE).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonTransactionType {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
        //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STInt16::serialize(self.value);
        so.append(&mut s);

        println!("TxJsonTransactionType so : {:?}", &so);
    }
}


pub struct TxJsonTransactionTypeBuilder {
    pub value: u16,
}
impl TxJsonTransactionTypeBuilder {
    pub fn new(value: u16) -> Self {
        TxJsonTransactionTypeBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonTransactionTypeBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonTransactionType::new(self.value) )
    }
}

//Sequence
pub struct TxJsonSequence {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : u32,
}

impl TxJsonSequence {
    pub fn new(value: u32) -> Self {
        TxJsonSequence {
            name    : TX_SEQUENCE.to_string(),
            typeObj : TypeObjBuilder::new(TX_SEQUENCE).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonSequence {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STInt32::serialize(self.value);
        so.append(&mut s);

        println!("TxJsonSequence so : {:?}", &so);
    }
}
pub struct TxJsonSequenceBuilder {
    pub value: u32,
}
impl TxJsonSequenceBuilder {
    pub fn new(value: u32) -> Self {
        TxJsonSequenceBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonSequenceBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonSequence::new(self.value) )
    }
}

//Amount
pub struct TxJsonAmount {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : String,
}

impl TxJsonAmount {
    pub fn new(value: String) -> Self {
        TxJsonAmount {
            name    : TX_AMOUNT.to_string(),
            typeObj : TypeObjBuilder::new(TX_AMOUNT).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonAmount {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let amount = Amount::from_json(String::from(self.value.as_str()));
        let mut s = STAmount::serialize(amount);
        so.append(&mut s);

        println!("TxJsonAmount so : {:?}", &so);
    }
}
pub struct TxJsonAmountBuilder {
    pub value   : String,
}
impl TxJsonAmountBuilder {
    pub fn new(value: String) -> Self {
        TxJsonAmountBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonAmountBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonAmount::new(String::from(self.value.as_str()) ))
    }
}

//Fee
pub struct TxJsonFee {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : String,
}

impl TxJsonFee {
    pub fn new(value: String) -> Self {
        TxJsonFee {
            name    : TX_FEE.to_string(),
            typeObj : TypeObjBuilder::new(TX_FEE).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonFee {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let amount = Amount::from_json(String::from(self.value.as_str()));
        let mut s = STAmount::serialize(amount);
        so.append(&mut s);

        println!("TxJsonFee so : {:?}", &so);
    }
}
pub struct TxJsonFeeBuilder {
    pub value   : String,
}
impl TxJsonFeeBuilder {
    pub fn new(value: String) -> Self {
        TxJsonFeeBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonFeeBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonFee::new(String::from(self.value.as_str()) ))
    }
}

//SigningPubKey
pub struct TxJsonSigningPubKey {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : String,
}

impl TxJsonSigningPubKey {
    pub fn new(value: String) -> Self {
        TxJsonSigningPubKey {
            name    : TX_SIGNING_PUB_KEY.to_string(),
            typeObj : TypeObjBuilder::new(TX_SIGNING_PUB_KEY).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonSigningPubKey {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STVL::serialize(&self.value);
        so.append(&mut s);

        println!("TxJsonSigningPubKey so : {:?}", &so);
    }
}
pub struct TxJsonSigningPubKeyBuilder {
    pub value   : String,
}
impl TxJsonSigningPubKeyBuilder {
    pub fn new(value: String) -> Self {
        TxJsonSigningPubKeyBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonSigningPubKeyBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonSigningPubKey::new(String::from(self.value.as_str()) ))
    }
}

//Account
pub struct TxJsonAccount {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : String,
}

impl TxJsonAccount {
    pub fn new(value: String) -> Self {
        TxJsonAccount {
            name    : TX_ACCOUNT.to_string(),
            typeObj : TypeObjBuilder::new(TX_ACCOUNT).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonAccount {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STAccount::serialize(String::from(self.value.as_str()));
        so.append(&mut s);

        println!("TxJsonAccount so : {:?}", &so);
    }
}
pub struct TxJsonAccountBuilder {
    pub value   : String,
}
impl TxJsonAccountBuilder {
    pub fn new(value: String) -> Self {
        TxJsonAccountBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonAccountBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonAccount::new(String::from(self.value.as_str())) )
    }
}

//Destination
pub struct TxJsonDestination {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : String,
}

impl TxJsonDestination {
    pub fn new(value: String) -> Self {
        TxJsonDestination {
            name    : TX_DESTINATION.to_string(),
            typeObj : TypeObjBuilder::new(TX_DESTINATION).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonDestination {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STAccount::serialize(String::from(self.value.as_str()));
        so.append(&mut s);

        println!("TxJsonDestination so : {:?}", &so);
    }
}
pub struct TxJsonDestinationBuilder {
    pub value   : String,
}
impl TxJsonDestinationBuilder {
    pub fn new(value: String) -> Self {
        TxJsonDestinationBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonDestinationBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonDestination::new(String::from(self.value.as_str())) )
    }
}

//TxnSignature
pub struct TxJsonTxnSignature {
    pub name    : String,
    pub typeObj : Option<TypeObj>,
    pub value   : String,
}

impl TxJsonTxnSignature {
    pub fn new(value: String) -> Self {
        TxJsonTxnSignature {
            name    : TX_SIGNATURE.to_string(),
            typeObj : TypeObjBuilder::new(TX_SIGNATURE).build(),
            value   : value,
        }
    }
}
impl TxJsonSerializer for TxJsonTxnSignature {
    fn serialize_obj(&self, so: &mut Vec<u8>) {
                //serialize header
        if let Some(raw) = &self.typeObj {
            raw.serialize_header(so);
            println!("header: {:?}", so);
        }

        let mut s = STVL::serialize(&self.value);
        so.append(&mut s);

        println!("TxJsonTxnSignature so : {:?}", &so);
    }
}
pub struct TxJsonTxnSignatureBuilder {
    pub value   : String,
}
impl TxJsonTxnSignatureBuilder {
    pub fn new(value: String) -> Self {
        TxJsonTxnSignatureBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder for TxJsonTxnSignatureBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonTxnSignature::new( String::from(self.value.as_str())) )
    }
}

////////////////////////////////////////////////
//
pub struct SignedTxJson {
    pub components: Vec<Box<dyn TxJsonSerializer>>,
}
impl SignedTxJson {
    pub fn new() -> Self {
        SignedTxJson {
            components: vec![],
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut so: Vec<u8> = vec![];
        for component in self.components.iter() {
            component.serialize_obj(&mut so);
        }

        so
    }

    pub fn insert(&mut self, item: Box<dyn TxJsonSerializer>) {
        self.components.push(item);
    }
}
