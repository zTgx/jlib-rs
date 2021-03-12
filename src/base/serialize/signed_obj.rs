//
//
// 说明：
// 在序列化raw tx_json的过程中，通过把TxJson对象转化成待signed的特殊结构（SignedTxJson）来进行真正的序列化操作，
// SignedTxJson中包括了各个字段组成的对象如：TxJsonFlags等，这些对象都实现了TxJsonSerialize trait, (trait object).
//
//

use crate::base::serialize::type_obj::*;
use crate::base::data::constants::{
    TX_SIGNATURE, TX_DESTINATION, TX_ACCOUNT, TX_SIGNING_PUB_KEY, TX_FEE,
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE,TX_FLAGS, TX_MEMOS, TX_MEMO, TX_MEMODATA, SignStreamType,
    TX_OFFER_SEQUENCE, TX_LIMIT_AMOUNT, TX_TARGET, TX_RELATION_TYPE, TXTakerType,
    TX_RATE_DEN, TX_RATE_NUM, TX_FEE_ACCOUNT,
};
use crate::base::serialize::serialized_type::*;
use crate::base::misc::amount::*;
use crate::api::message::amount::Amount as RAmount;
use std::marker::PhantomData;

//序列化接口
pub trait TxJsonSerializer {
    fn serialize_obj(&mut self, so: &mut Vec<u8>);
}

//builder接口
pub trait TxJsonBuilder <'c> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'c>;
}



///Flags
pub struct TxJsonFlags {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : u32,

    //output
    pub output : SignStreamType,
}
impl TxJsonFlags {
    pub fn new(value: u32) -> Self {
        TxJsonFlags {
            name    : TX_FLAGS.to_string(),
            type_obj : TypeObjBuilder::new(TX_FLAGS).build(),
            value   : value,

            output  : None,
        }
    }
}
impl TxJsonSerializer for TxJsonFlags {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt32::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonFlagsBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonFlags::new(self.value) )
    }
}

//TransactionType
pub struct TxJsonTransactionType {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : u16,

    //output
    pub output: SignStreamType,
}

impl TxJsonTransactionType {
    pub fn new(value: u16) -> Self {
        TxJsonTransactionType {
            name    : TX_TRANSACTION_TYPE.to_string(),
            type_obj : TypeObjBuilder::new(TX_TRANSACTION_TYPE).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonTransactionType {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {

            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt16::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonTransactionTypeBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonTransactionType::new(self.value) )
    }
}

//Sequence
pub struct TxJsonSequence {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : u32,

    //output
    pub output: SignStreamType,
}

impl TxJsonSequence {
    pub fn new(value: u32) -> Self {
        TxJsonSequence {
            name    : TX_SEQUENCE.to_string(),
            type_obj : TypeObjBuilder::new(TX_SEQUENCE).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonSequence {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt32::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonSequenceBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonSequence::new(self.value) )
    }
}

//OfferSequence
pub struct TxJsonOfferSequence {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : u32,

    //output
    pub output: SignStreamType,
}

impl TxJsonOfferSequence {
    pub fn new(value: u32) -> Self {
        TxJsonOfferSequence {
            name    : TX_OFFER_SEQUENCE.to_string(),
            type_obj : TypeObjBuilder::new(TX_OFFER_SEQUENCE).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonOfferSequence {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt32::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonOfferSequenceBuilder {
    pub value: u32,
}
impl TxJsonOfferSequenceBuilder {
    pub fn new(value: u32) -> Self {
        TxJsonOfferSequenceBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder <'_> for TxJsonOfferSequenceBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonOfferSequence::new(self.value) )
    }
}

//RelationType
pub struct TxJsonRelationType {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : u32,

    //output
    pub output: SignStreamType,
}

impl TxJsonRelationType {
    pub fn new(value: u32) -> Self {
        TxJsonRelationType {
            name    : TX_RELATION_TYPE.to_string(),
            type_obj : TypeObjBuilder::new(TX_RELATION_TYPE).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonRelationType {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt32::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonRelationTypeBuilder {
    pub value: u32,
}
impl TxJsonRelationTypeBuilder {
    pub fn new(value: u32) -> Self {
        TxJsonRelationTypeBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder <'_> for TxJsonRelationTypeBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonRelationType::new(self.value) )
    }
}

//Amount
pub struct TxJsonAmount <'a> {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : &'a RAmount,

    pub output: SignStreamType,
}

impl <'a> TxJsonAmount <'a> {
    pub fn new(value: &'a RAmount) -> Self {
        TxJsonAmount {
            name    : TX_AMOUNT.to_string(),
            type_obj: TypeObjBuilder::new(TX_AMOUNT).build(),
            value   : value,

            output: None,
        }
    }
}
impl <'a> TxJsonSerializer for TxJsonAmount <'a> {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8>= vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let amount = Amount::from_ramount( self.value );
        let mut s = STAmount::serialize(amount);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonAmountBuilder <'a> {
    pub value   : &'a RAmount,
}
impl <'a> TxJsonAmountBuilder <'a> {
    pub fn new(value: &'a RAmount) -> Self {
        TxJsonAmountBuilder {
            value: value,
        }
    }
}
impl <'a> TxJsonBuilder <'a> for TxJsonAmountBuilder <'a> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'a> {
        Box::new( TxJsonAmount::new( self.value ))
    }
}

//Limit_Amount
pub struct TxJsonLimitAmount <'a> {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : &'a RAmount,

    pub output: SignStreamType,
}

impl <'a> TxJsonLimitAmount <'a> {
    pub fn new(value: &'a RAmount) -> Self {
        TxJsonLimitAmount {
            name    : TX_LIMIT_AMOUNT.to_string(),
            type_obj: TypeObjBuilder::new(TX_LIMIT_AMOUNT).build(),
            value   : value,

            output: None,
        }
    }
}
impl <'a> TxJsonSerializer for TxJsonLimitAmount <'a> {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8>= vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let amount = Amount::from_ramount( self.value );
        //let amount = Amount::from_json(String::from(self.value.as_str()));
        let mut s = STAmount::serialize(amount);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonLimitAmountBuilder <'a> {
    pub value   : &'a RAmount,
}
impl <'a> TxJsonLimitAmountBuilder <'a> {
    pub fn new(value: &'a RAmount) -> Self {
        TxJsonLimitAmountBuilder {
            value: value,
        }
    }
}
impl <'a> TxJsonBuilder <'a> for TxJsonLimitAmountBuilder <'a> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'a> {
        Box::new( TxJsonLimitAmount::new( &self.value ) )
    }
}

//TxJsonTakerBuilder
pub struct TxJsonTaker <'a> {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : &'a RAmount,

    pub output: SignStreamType,
}

impl <'a> TxJsonTaker <'a> {
    pub fn new(name: &TXTakerType, value: &'a RAmount) -> Self {
        TxJsonTaker {
            name    : String::from( name.get() ),
            type_obj: TypeObjBuilder::new( name.get() ).build(),
            value   : value,

            output: None,
        }
    }
}
impl <'a> TxJsonSerializer for TxJsonTaker <'a> {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8>= vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let amount = Amount::from_ramount(self.value);

        let mut s = STAmount::serialize(amount);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonTakerBuilder <'a> {
    pub name: TXTakerType,
    pub value: &'a RAmount,
}
impl <'a> TxJsonTakerBuilder <'a> {
    pub fn new(name: TXTakerType, value: &'a RAmount) -> Self {
        TxJsonTakerBuilder {
            name: name,
            value: value,
        }
    }
}
impl <'a> TxJsonBuilder <'a> for TxJsonTakerBuilder <'a> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'a> {
        Box::new( TxJsonTaker::new( &self.name, &self.value) )
    }
}

//Fee
pub struct TxJsonFee {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : u64,

    pub output: SignStreamType,
}

impl TxJsonFee {
    pub fn new(value: u64) -> Self {
        TxJsonFee {
            name    : TX_FEE.to_string(),
            type_obj: TypeObjBuilder::new(TX_FEE).build(),
            value   : value,

            output : None,
        }
    }
}
impl TxJsonSerializer for TxJsonFee {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let amount = Amount::from_value( self.value );
        let mut s = STAmount::serialize(amount);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonFeeBuilder {
    pub value   : u64,
}
impl TxJsonFeeBuilder {
    pub fn new(value: u64) -> Self {
        TxJsonFeeBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder <'_> for TxJsonFeeBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonFee::new( self.value ))
    }
}

//SigningPubKey
pub struct TxJsonSigningPubKey {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : String,

    pub output: SignStreamType,
}

impl TxJsonSigningPubKey {
    pub fn new(value: String) -> Self {
        TxJsonSigningPubKey {
            name    : TX_SIGNING_PUB_KEY.to_string(),
            type_obj : TypeObjBuilder::new(TX_SIGNING_PUB_KEY).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonSigningPubKey {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STVL::serialize(&self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonSigningPubKeyBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonSigningPubKey::new(String::from(self.value.as_str()) ))
    }
}

//Account
pub struct TxJsonAccount {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : String,

    //
    pub output: SignStreamType,
}

impl TxJsonAccount {
    pub fn new(value: String) -> Self {
        TxJsonAccount {
            name    : TX_ACCOUNT.to_string(),
            type_obj : TypeObjBuilder::new(TX_ACCOUNT).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonAccount {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STAccount::serialize(String::from(self.value.as_str()));
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonAccountBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonAccount::new(String::from(self.value.as_str())) )
    }
}

//FeeAccount
pub struct TxJsonFeeAccount <'a> {
    pub type_obj: Option<TypeObj>,
    pub value   : &'a String,

    pub output: SignStreamType,
}

impl <'a> TxJsonFeeAccount <'a> {
    pub fn new(value: &'a String) -> Self {
        TxJsonFeeAccount {
            type_obj: TypeObjBuilder::new(TX_FEE_ACCOUNT).build(),
            value   : value,

            output: None,
        }
    }
}
impl <'a> TxJsonSerializer for TxJsonFeeAccount <'a> {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STAccount::serialize( self.value.to_string() );
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonFeeAccountBuilder <'a> {
    pub value   : &'a String,
}
impl <'a> TxJsonFeeAccountBuilder <'a> {
    pub fn new(value: &'a String) -> Self {
        TxJsonFeeAccountBuilder {
            value: value,
        }
    }
}
impl <'a> TxJsonBuilder <'a> for TxJsonFeeAccountBuilder <'a> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'a> {
        Box::new( TxJsonFeeAccount::new( self.value ) )
    }
}

//Account
pub struct TxJsonTarget {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : String,

    pub output: SignStreamType,
}

impl TxJsonTarget {
    pub fn new(value: String) -> Self {
        TxJsonTarget {
            name    : TX_TARGET.to_string(),
            type_obj: TypeObjBuilder::new(TX_TARGET).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonTarget {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STAccount::serialize(String::from(self.value.as_str()));
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonTargetBuilder {
    pub value   : String,
}
impl TxJsonTargetBuilder {
    pub fn new(value: String) -> Self {
        TxJsonTargetBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder <'_> for TxJsonTargetBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonTarget::new( String::from( self.value.as_str() )) )
    }
}

//Destination
pub struct TxJsonDestination {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : String,

    pub output: SignStreamType,
}

impl TxJsonDestination {
    pub fn new(value: String) -> Self {
        TxJsonDestination {
            name    : TX_DESTINATION.to_string(),
            type_obj : TypeObjBuilder::new(TX_DESTINATION).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonDestination {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STAccount::serialize(String::from(self.value.as_str()));
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonDestinationBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonDestination::new(String::from(self.value.as_str())) )
    }
}

//TxnSignature
pub struct TxJsonTxnSignature {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : String,

    pub output: SignStreamType,
}

impl TxJsonTxnSignature {
    pub fn new(value: String) -> Self {
        TxJsonTxnSignature {
            name    : TX_SIGNATURE.to_string(),
            type_obj : TypeObjBuilder::new(TX_SIGNATURE).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonTxnSignature {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STVL::serialize(&self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
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
impl TxJsonBuilder <'_> for TxJsonTxnSignatureBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonTxnSignature::new( String::from(self.value.as_str())) )
    }
}

//memodata builder
pub struct TxJsonMemoData {
    pub name    : String,
    pub type_obj: Option<TypeObj>,
    pub value   : String,

    pub output  : SignStreamType,
}
impl TxJsonMemoData {
    pub fn new(value: String) -> Self {
        TxJsonMemoData {
            name        : TX_MEMODATA.to_string(),
            type_obj    : TypeObjBuilder::new(TX_MEMODATA).build(),
            value       : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonMemoData {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STMemo::serialize(&self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}

//memo
pub struct TxJsonMemo {
    pub name     : String,
    pub type_obj : Option<TypeObj>,
    pub value    : TxJsonMemoData,

    pub output: SignStreamType,
}

impl TxJsonMemo {
    pub fn new(value: TxJsonMemoData) -> Self {
        TxJsonMemo {
            name     : TX_MEMO.to_string(),
            type_obj : TypeObjBuilder::new(TX_MEMO).build(),
            value    : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonMemo {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        // let mut s = TxJsonMemoData::new(String::from( self.value.as_str() ));
        let s = &mut self.value;
        s.serialize_obj(&mut tmp);

        //Object ending marker
        let mut end_mark = STInt8::serialize(0xe1);
        tmp.append(&mut end_mark);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}

pub struct TxJsonMemoBuilder {
    pub value: String,
}
impl  TxJsonMemoBuilder  {
    pub fn new(value: String) -> Self {
        TxJsonMemoBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder <'_> for TxJsonMemoBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        let meme_data = TxJsonMemoData::new( String::from( self.value.as_str() ) );
        Box::new( TxJsonMemo::new( meme_data ) )
    }
}


//array builder
pub struct TxJsonMemos {
    pub name     : String,
    pub type_obj : Option<TypeObj>,
    pub value    : Vec<Box<dyn TxJsonSerializer>>,//TxJsonMemo>,

    pub output: SignStreamType,
}

impl TxJsonMemos {
    pub fn new(value: Vec<Box<dyn TxJsonSerializer>>) -> Self {
        TxJsonMemos {
            name     : TX_MEMOS.to_string(),
            type_obj : TypeObjBuilder::new(TX_MEMOS).build(),
            value    : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonMemos {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {

        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut i = 0;
        while i < self.value.len() {
            let s = &mut self.value[i];
            s.serialize_obj(&mut tmp);

            i += 1;
        }

        //Array ending marker
        let mut end_mark = STInt8::serialize(0xf1);
        tmp.append(&mut end_mark);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}

pub struct TxJsonMemosBuilder {
    pub value: Vec<String>,
}
impl TxJsonMemosBuilder {
    pub fn new(value: Vec<String>) -> Self {
        TxJsonMemosBuilder {
            value: value,
        }
    }
}
impl TxJsonBuilder <'_> for TxJsonMemosBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        let mut v: Vec<Box<dyn TxJsonSerializer>> = vec![];
        let mut i = 0;
        while i < self.value.len() {
            let s = TxJsonMemoBuilder::new( String::from( self.value[i].as_str() ) ).build();

            v.push( s );

            i += 1;
        }

        Box::new( TxJsonMemos::new( v ) )
    }
}

//Brokerage den
pub struct TxJsonBrokerageDen <'a> {
    pub type_obj: Option<TypeObj>,
    pub value   : u64,

    pub output: SignStreamType,

    phantom: PhantomData<&'a u64>,
}

impl <'a> TxJsonBrokerageDen <'a> {
    pub fn new(value: u64) -> Self {
        TxJsonBrokerageDen {
            type_obj: TypeObjBuilder::new( TX_RATE_DEN ).build(),
            value   : value,

            output : None,

            phantom: PhantomData,
        }
    }
}
impl <'a> TxJsonSerializer for TxJsonBrokerageDen <'a> {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt64::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonBrokerageDenBuilder <'a> {
    pub value   : u64,

    phantom: PhantomData<&'a u64>,
}
impl <'a> TxJsonBrokerageDenBuilder <'a> {
    pub fn new(value: u64) -> Self {
        TxJsonBrokerageDenBuilder {
            value: value,
            phantom: PhantomData,
        }
    }
}
impl <'a> TxJsonBuilder <'a> for TxJsonBrokerageDenBuilder <'a> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'a> {
        Box::new( TxJsonBrokerageDen::new( self.value ) )
    }
}

//Brokerage num
pub struct TxJsonBrokerageNum <'a> {
    pub type_obj: Option<TypeObj>,
    pub value   : u64,

    pub output: SignStreamType,

    phantom: PhantomData<&'a u64>,
}

impl <'a> TxJsonBrokerageNum <'a> {
    pub fn new(value: u64) -> Self {
        TxJsonBrokerageNum {
            type_obj: TypeObjBuilder::new( TX_RATE_NUM ).build(),
            value   : value,

            output : None,
            phantom: PhantomData,
        }
    }
}
impl <'a> TxJsonSerializer for TxJsonBrokerageNum <'a> {
    fn serialize_obj(&mut self, so: &mut Vec<u8>) {
        if self.output.is_some() {
            if let Some(x) = &self.output {
                so.extend_from_slice(&x);
            }

            return;
        }

        let mut tmp: Vec<u8> = vec![];
        //serialize header
        if let Some(raw) = &self.type_obj {
            raw.serialize_header(&mut tmp);
        }

        let mut s = STInt64::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
    }
}
pub struct TxJsonBrokerageNumBuilder <'a> {
    pub value   : u64,

    phantom: PhantomData<&'a u64>,
}
impl <'a> TxJsonBrokerageNumBuilder <'a> {
    pub fn new(value: u64) -> Self {
        TxJsonBrokerageNumBuilder {
            value: value,
            phantom: PhantomData,
        }
    }
}
impl <'a> TxJsonBuilder <'a> for TxJsonBrokerageNumBuilder <'a> {
    fn build(&self) -> Box<dyn TxJsonSerializer + 'a> {
        Box::new( TxJsonBrokerageNum::new( self.value ) )
    }
}

////////////////////////////////////////////////
//
pub struct SignedTxJson <'s> {
    pub components: Vec<Box<dyn TxJsonSerializer + 's>>,
}

impl <'s> SignedTxJson <'s> {
    pub fn new() -> Self {
        SignedTxJson {
            components: vec![],
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        let mut so: Vec<u8> = vec![];
        for component in self.components.as_mut_slice() {
            component.serialize_obj(&mut so);
        }

        so
    }

    pub fn insert(&mut self, index: usize, item: Box<dyn TxJsonSerializer + 's>) {
        self.components.insert(index, item);
    }
}
