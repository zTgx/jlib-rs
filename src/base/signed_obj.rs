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
    TX_AMOUNT, TX_SEQUENCE, TX_TRANSACTION_TYPE,TX_FLAGS, TX_MEMOS, TX_MEMO, TX_MEMODATA, SignStreamType,
    TX_OFFER_SEQUENCE,
};
use crate::base::serialized_type::*;
use crate::base::amount::*;

//序列化接口
pub trait TxJsonSerializer {
    fn serialize_obj(&mut self, so: &mut Vec<u8>);
}

//builder接口
pub trait TxJsonBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer>;
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
            println!("header: {:?}", tmp);
        }

        let mut s = STInt32::serialize(self.value);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }
        
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

        println!("TxJsonOfferSequence so : {:?}", &so);
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
impl TxJsonBuilder for TxJsonOfferSequenceBuilder {
    fn build(&self) -> Box<dyn TxJsonSerializer> {
        Box::new( TxJsonOfferSequence::new(self.value) )
    }
}

//Amount
pub struct TxJsonAmount {
    pub name    : String,
    pub type_obj : Option<TypeObj>,
    pub value   : String,

    pub output: SignStreamType,
}

impl TxJsonAmount {
    pub fn new(value: String) -> Self {
        TxJsonAmount {
            name    : TX_AMOUNT.to_string(),
            type_obj : TypeObjBuilder::new(TX_AMOUNT).build(),
            value   : value,

            output: None,
        }
    }
}
impl TxJsonSerializer for TxJsonAmount {
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

        let amount = Amount::from_json(String::from(self.value.as_str()));
        let mut s = STAmount::serialize(amount);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }

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
    pub type_obj : Option<TypeObj>,
    pub value   : String,

    pub output: SignStreamType,
}

impl TxJsonFee {
    pub fn new(value: String) -> Self {
        TxJsonFee {
            name    : TX_FEE.to_string(),
            type_obj : TypeObjBuilder::new(TX_FEE).build(),
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

        let amount = Amount::from_json(String::from( self.value.as_str()));
        let mut s = STAmount::serialize(amount);
        tmp.append(&mut s);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }


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

        println!("TxJsonMemo so : {:?}", &so);
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

        println!("TxJsonMemos so : {:?}", &so);
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
impl TxJsonBuilder for TxJsonMemoBuilder {
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
            //get memo, not memos~~~~~~            
            // let mut s = TxJsonMemo::new(String::from( self.value[i].as_str() ));
            let s = &mut self.value[i];
            s.serialize_obj(&mut tmp);

            i += 1;
        }
        // let mut s = STArray::serialize(&self.value);
        // println!("array value hex : {:?}", &s);
        // tmp.append(&mut s);

        //Array ending marker
        let mut end_mark = STInt8::serialize(0xf1);
        tmp.append(&mut end_mark);

        self.output = Some(tmp);

        if let Some(x) = &self.output {
            so.extend_from_slice(&x);
        }

        println!("TxJsonArray so : {:?}", &so);
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
impl TxJsonBuilder for TxJsonMemosBuilder {
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

    pub fn serialize(&mut self) -> Vec<u8> {
        let mut so: Vec<u8> = vec![];
        for component in self.components.as_mut_slice() {
            component.serialize_obj(&mut so);
        }

        so
    }

    pub fn insert(&mut self, index: usize, item: Box<dyn TxJsonSerializer>) {
        self.components.insert(index, item);
    }
}
