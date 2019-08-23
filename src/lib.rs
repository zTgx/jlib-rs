#[macro_use]
extern crate lazy_static;

extern crate typename;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate num;
extern crate void;
extern crate ws;
extern crate basex_rs;
extern crate cast_rs;

pub mod base;
pub mod message;
pub mod misc;
pub mod api;
pub mod contracts;

use serde_json::json;
use cast_rs::hexcast;
use misc::base_config::{CURRENCY};

pub use crate::base::wallet::wallet::Wallet as Wallet;

//Default` cannot be derived for enums, only structs
#[derive(Debug)]
pub enum RelationType {
    TRUST     = 0,
    AUTHORIZE = 1,
    FREEZE    = 3,
}
impl RelationType {
    pub fn get(&self) -> u32 {
        match *self {
            RelationType::TRUST     => { 0 },
            RelationType::AUTHORIZE => { 1 },
            RelationType::FREEZE    => { 3 },
        }
    }
}

//Offer Type
#[derive(PartialEq)]
pub enum OfferType {
    Sell,
    Buy,
}
impl OfferType {
    pub fn get(&self) -> &'static str {
        match *self {
            OfferType::Sell => { "Sell" },
            OfferType::Buy  => { "Buy"  },
        }
    }
}

//Generate Wallet
/*
Wallet DataStruct:
#[derive(Debug)]
pub struct Wallet {
    pub key_type: WalletType,
    pub address : String,    //starts with 'j'
    pub secret  : String,    //secret seed
    pub keypair : Keypair,   //public key & private key
}

Keypair DataStruct:
#[derive(Debug, Clone)]
pub struct Keypair {
    pub private_key: String, //hex string
    pub public_key: String,  //hex string
}
*/

#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
}

pub fn generate_wallet(wtype: WalletType) -> Wallet {
    Wallet::new(wtype)
}

//Subscribe
mod subscribe {

use ws::{connect, Handler, Sender, Handshake, Message, CloseCode};

use std::rc::Rc;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::subscribe::*;
use crate::message::common::command_trait::CommandConversion;

pub struct Client {
    out: Sender,
    op: Rc<dyn Fn(Result<SubscribeResponse, serde_json::error::Error>)>,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        if let Ok(command) = SubscribeCommand::default().to_string() {
            self.out.send(command).unwrap();
        }

        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        // Close the connection when we get a response from the server
        let resp = msg.into_text().unwrap();
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let result: String = x["result"].to_string();
            if let Ok(x) = serde_json::from_str(&result) as Result<SubscribeResponse, serde_json::error::Error> {
                //to call the function stored in `op`, surround the field access with parentheses
                (self.op)(Ok(x))
            } else if let Ok(x) = serde_json::from_str(&resp) as Result<SubscribeResponse, serde_json::error::Error> {
                (self.op)(Ok(x))
            }
        }

        // self.out.close(CloseCode::Normal)
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

pub trait SubscribeI {
    fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
    where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>);
}

pub struct Subscribe {}
impl Subscribe {
    pub fn new() -> Self {
        Subscribe {
        }
    }
}

impl SubscribeI for Subscribe {
    fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
    where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>) {

        let op_rc = Rc::new(op);

        connect(config.addr, |out| {

            let op = op_rc.clone();

            Client {
                out: out,
                op: op,
            }

        }).unwrap();
    }
}
} //end mod subscribe

lazy_static! {
    pub static ref SUBSCRIBE: subscribe::Subscribe = {
        subscribe::Subscribe::new()
    };
}
pub use subscribe::SubscribeI as SubscribeI;

///////////////////////////////////////////////////////////////////////////////////////
//
// Solidity contract APIs: deploy && invoke
//
///////////////////////////////////////////////////////////////////////////////////////

use crate::contracts::solidity::{
    SolidityInitMessage,
    SolidityInvokeMessage,
};
use std::rc::Rc;
use std::cell::Cell;
use ws::{connect, CloseCode};
use serde_json::Value;
use crate::misc::config::Config;
use message::common::command_trait::CommandConversion;
use serde::{Deserialize, Serialize};
use crate::base::misc::util::{downcast_to_string, check};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Args {
    #[serde(rename="Arg")]
    pub arg: Arg,
}
impl Args {
    pub fn new(arg: Arg) -> Self {
        Args {
            arg: arg,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Arg {
    #[serde(rename="Parameter")]
    pub parameter: String, //Hex String, 0xb6e456bb ===>getUnit 调用方法的（以太坊方式）十六进制处理。

    #[serde(rename="ContractParamsType")]
    pub contract_params_type: u8, //0 -> Address type; 1 -> general type.
}
impl Arg {
    pub fn new(parameter: String, contract_params_type: u8) -> Self {
        Arg {
            parameter: check(parameter),
            contract_params_type: contract_params_type,
        }
    }
}

//
// SolidityInitResponse
//
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInitTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Fee")]
    pub fee: String,

    #[serde(rename="Flags")]
    pub flags: u64,

    #[serde(rename="Payload")]
    pub payload: String,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="Method")]
    pub method: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Timestamp")]
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInitResponse {
    #[serde(rename="ContractState")]
    pub address: String,

    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: u64,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: SolidityInitTxJsonResponse,
}


/////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInvokeTxJsonResponse {
    #[serde(rename="Account")]
    pub account: String,

    #[serde(rename="Amount")]
    pub amount: String,

    #[serde(rename="Args")]
    pub args: Vec<Args>,

    #[serde(rename="ContractMethod")]
    pub method: String,

    #[serde(rename="Sequence")]
    pub sequence: u64,

    #[serde(rename="SigningPubKey")]
    pub signing_pub_key: String,

    #[serde(rename="Timestamp")]
    pub timestamp: u64,

    #[serde(rename="TransactionType")]
    pub transaction_type: String,

    #[serde(rename="TxnSignature")]
    pub txn_signature: String,

    #[serde(rename="hash")]
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SolidityInvokeResponse {
    #[serde(rename="ContractState")]
    pub contract_state: String,

    #[serde(rename="engine_result")]
    pub engine_result: String,

    #[serde(rename="engine_result_code")]
    pub engine_result_code: u64,

    #[serde(rename="engine_result_message")]
    pub engine_result_message: String,

    #[serde(rename="tx_blob")]
    pub tx_blob: String,

    #[serde(rename="tx_json")]
    pub tx_json: SolidityInvokeTxJsonResponse,
}

pub struct SolidityDeploy <'a> {
    pub config  : Box<Rc<Config>>,
    pub account : &'a String,
    pub secret  : &'a String,
}
impl <'a> SolidityDeploy <'a> {
    pub fn with_params(config: Box<Rc<Config>>, account: &'a String, secret: &'a String) -> Self {
        SolidityDeploy {
            config: config,
            account: account,
            secret: secret,
        }
    }

    pub fn deploy<F>(&self, payload: &'a String, op: F)
    where F: Fn(Result<SolidityInitResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account = Rc::new(Cell::new( String::from( self.account.as_str() )) );
        let secret = Rc::new(Cell::new( String::from( self.secret.as_str() )) );
        let payload = Rc::new(Cell::new( String::from( payload.as_str() )) );

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account.clone();
            let secret = secret.clone();
            let payload = payload.clone();

            let message = SolidityInitMessage::with_params(account.take(), secret.take(), payload.take());
            if let Ok(command) = message.to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<SolidityInitResponse, serde_json::error::Error> {
                    op(Ok(v))
                } else {
                    op(Err("deploy Error!"))
                }
        } else {
            op(Err("deploy Error!"))
        }
    }
}

pub struct SolidityCall <'a> {
    pub config: Box<Rc<Config>>,
    pub account : &'a String,
    pub secret  : &'a String,
    pub address : &'a String,
}
impl <'a> SolidityCall <'a> {
    pub fn with_params(config: Box<Rc<Config>>, account: &'a String, secret: &'a String, address: &'a String) -> Self {
        SolidityCall {
            config: config,
            account: account,
            secret: secret,
            address: address,
        }
    }

    pub fn call<F>(&self, method_name: &'a String, args: Vec<Arg>, op: F)
    where F: Fn(Result<SolidityInvokeResponse, &'static str>){
        let info = Rc::new(Cell::new("".to_string()));

        let account = Rc::new(Cell::new( String::from( self.account.as_str())) );
        let secret = Rc::new(Cell::new( String::from(self.secret.as_str())) );
        let address = Rc::new(Cell::new( String::from( self.address.as_str() )));
        let method_name = Rc::new(Cell::new( String::from( method_name.as_str() )) );
        let args = Rc::new(Cell::new(args));

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account.clone();
            let secret = secret.clone();
            let address = address.clone();
            let method_name = method_name.clone();
            let args = args.clone();

            let message = SolidityInvokeMessage::with_params(account.take(), secret.take(), address.take(), method_name.take(), args.take());
            if let Ok(command) = message.to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<SolidityInvokeResponse, serde_json::error::Error> {
                    op(Ok(v))
                } else {
                    op(Err("Err..."))
                }
        } else {
            op(Err("Err..."))
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
//processTx
/*
{"Account": "jKCQAZwwN2sQG3Mb56GmWVqxkgpLwwAZuR",
                "Fee": "10000",
                "Flags": 524288,
                "Sequence": 650,
                "SigningPubKey": "03E791056E6B4C62E26C0F1F3BB89317667AB74170B49339972716FC53FFCF007C",
                "TakerGets": "2000000000",
                "TakerPays":
                    { "currency": "CNY",
                      "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                      "value": "13.46" },
                "Timestamp": 611552863,
                "TransactionType": "OfferCreate",
                "TxnSignature": "3045022100B342C7159E1AD7FAA13452C1FB01F77A107263AFB06E93F9B4F307EF9DF9F98E0220690C964D89146250E879B9EA5ED311C3317631F1F6450360603E6818AFFB5FAF",
                "date": 611552870,
                "hash": "AB2A25557FF03911A8FC0A412293BE9D9FCB20CDD530EE05957A9859F8467C32",
                "inLedger": 12839308,
                "ledger_index": 12839308,
                "validated": true ,
"meta":
    { "AffectedNodes": [ { "ModifiedNode":
        { "FinalFields": { "Account": "j9x4pABowsWxmK1DhhWyK34u3boC6h3LHe",
            "BookDirectory": "51603377F758E3C8FA007C77312DDA06A737A1395CD5FC435D0547675A0517F6",
            "BookNode": "0000000000000000",
            "Flags": 0,
            "OwnerNode": "0000000000000000",
            "Sequence": 7031,
            "TakerGets":
                { "currency": "CNY",
                    "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                    "value": "1148.95233" },
            "TakerPays": "170721000000" },
            "LedgerEntryType": "Offer",
            "LedgerIndex": "020110B8BED1F151B9D3AF9E5D412D8627CB08232B388ADE1F4B0C68E7608BEC",
            "PreviousFields": { "TakerGets":
                { "currency": "CNY",
                    "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                    "value": "1162.41233" },
                "TakerPays": "172721000000" },
            "PreviousTxnID": "9CB6AEFA273C750242D5B8AF4299347E77F9A47C5D0B89EE5F6A4D5577E8C4A0",
            "PreviousTxnLgrSeq": 12839301 } },
        { "ModifiedNode":
            { "FinalFields": { "Account": "jEoSyfChhUMzpRDttAJXuie8XhqyoPBYvV",
                "Balance": "533983297806",
                "Flags": 0,
                "OwnerCount": 1,
                "Sequence": 34380818 },
                "LedgerEntryType": "AccountRoot",
                "LedgerIndex": "109E80FB8CC6D82D4F7F7D77248C2C3C116ECCD4520B3D2A88421FFF94A57B1E",
                "PreviousFields": { "Balance": "533983287806", "Sequence": 34380817 },
                "PreviousTxnID": "756338B8F9D4DCC8D88382B1092B13F75F65F330970278AFC7449496FF9875E9",
                "PreviousTxnLgrSeq": 12839308 } },
        { "ModifiedNode":
            { "FinalFields": { "Balance":
                { "currency": "CNY",
                    "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                    "value": "-6872.222452374449" },
                "Flags": 2228224,
                "HighLimit":
                    { "currency": "CNY",
                        "issuer": "jKCQAZwwN2sQG3Mb56GmWVqxkgpLwwAZuR",
                        "value": "10000000000" },
                "HighNode": "0000000000000000",
                "LowLimit":
                    { "currency": "CNY",
                        "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                        "value": "0" },
                "LowNode": "00000000000012A0" },
                "LedgerEntryType": "SkywellState",
                "LedgerIndex": "2600F8FCB87FEA15F74B0DB785016384C79AEA0730B62F597C1E576801BB813B",
                "PreviousFields": { "Balance":
                    { "currency": "CNY",
                        "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                        "value": "-6858.762452374449" } },
                "PreviousTxnID": "9B28F7958E729F0F904410B132D1F81481B38DD9F017790A82168CD38C995331",
                "PreviousTxnLgrSeq": 12838251 } },
        { "ModifiedNode":
            { "FinalFields":{ "Account": "j9x4pABowsWxmK1DhhWyK34u3boC6h3LHe",
                "Balance": "1496144192938",
                "Flags": 0,
                "OwnerCount": 8,
                "Sequence": 7032 },
                "LedgerEntryType": "AccountRoot",
                "LedgerIndex": "40A20BDD3C226C987579F6C821BF84492E1C6B6EFB62311481BA6B8CB1D7775A",
                "PreviousFields": { "Balance": "1494144192938" },
                "PreviousTxnID": "9CB6AEFA273C750242D5B8AF4299347E77F9A47C5D0B89EE5F6A4D5577E8C4A0",
                "PreviousTxnLgrSeq": 12839301 } },
        { "ModifiedNode":
            { "FinalFields": { "Account": "jKCQAZwwN2sQG3Mb56GmWVqxkgpLwwAZuR",
                "Balance": "500538133",
                "Flags": 0,
                "OwnerCount": 10,
                "Sequence": 651 },
                "LedgerEntryType": "AccountRoot",
                "LedgerIndex": "B39BD926378886F7EF4F81CEF862FC4D1E8E6D1265945AA9EC40FD85132DC629",
                "PreviousFields": { "Balance": "2500548133", "Sequence": 650 },
                "PreviousTxnID": "5BA24DE17EF64EDF942D99F247ED1495F5A61ED9260513FEDCA3E4BADBADFF3E",
                "PreviousTxnLgrSeq": 12839303 } },
        { "ModifiedNode":
            { "FinalFields": { "Balance":
                { "currency": "CNY",
                    "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                    "value": "1148.954817858577" },
                "Flags": 1114112,
                "HighLimit":
                    { "currency": "CNY",
                        "issuer": "jGa9J9TkqtBcUoHe2zqhVFFbgUVED6o9or",
                        "value": "0" },
                "HighNode": "000000000000172A",
                "LowLimit":
                    { "currency": "CNY",
                        "issuer": "j9x4pABowsWxmK1DhhWyK34u3boC6h3LHe",
                        "value": "10000000000" },
                "LowNode": "0000000000000000" },
                "LedgerEntryType": "SkywellState",
                "LedgerIndex": "E3E9FE1827E83B52F7017D3038F8C769F09343801BB073A993DE620756069137",
                "PreviousFields": { "Balance":
                    { "currency": "CNY",
                        "issuer": "jjjjjjjjjjjjjjjjjjjjBZbvri",
                        "value": "1162.414817858577" } },
                "PreviousTxnID": "9CB6AEFA273C750242D5B8AF4299347E77F9A47C5D0B89EE5F6A4D5577E8C4A0",
                "PreviousTxnLgrSeq": 12839301 } } ],
        "TransactionIndex": 3,
        "TransactionResult": "tesSUCCESS" }}
*/

///////////////////////////////////////////////////////
//constant
static OFFSET_SECOND: i64 = 946684800;

//
pub fn parse_amount(x: &Value) -> Value {
    let mut result: Value = json!({"value": "0"});

    println!("parse_amount: {:?}", &x);

    if x.is_string() { //SWT
        result["currency"] = json!("SWT");
        result["issuer"] = json!("");

        result["value"] = json!(x.as_str().unwrap().parse::<f32>().unwrap() / 1000000.0);
    } else {
        result["value"] = json!(x["value"].as_str().unwrap().parse::<f32>().unwrap());// x["value"].clone();
        result["currency"] = x["currency"].clone();
        result["issuer"] = x["issuer"].clone();
    }

    result
}

pub fn reverse_amount(x: &Value) -> Value {
    let mut result: Value = json!({"value": "0"});

    result["value"] = json!( "-".to_owned() + x["LimitAmount"]["value"].as_str().unwrap() );
    result["currency"] = x["LimitAmount"]["currency"].clone();
    result["issuer"] = x["Account"].clone();

    result
}

pub fn process_affect_node(an: &Value, result: &mut Value) {
    println!("an:--------------------------\n{:?}", an);
    if ! an["CreatedNode"].is_null() {
        result.as_object_mut().unwrap()["diffType"] = json!("CreatedNode");
    }

    if ! an["ModifiedNode"].is_null() {
        result.as_object_mut().unwrap()["diffType"] = json!("ModifiedNode");
    }

    if ! an["DeletedNode"].is_null() {
        result.as_object_mut().unwrap()["diffType"] = json!("DeletedNode");
    }

    if result["diffType"].is_null() {
        return;
    }

    let an = an[result["diffType"].clone().as_str().unwrap()].clone();

    result["entryType"] = an["LedgerEntryType"].clone();
    result["ledgerIndex"] = an["LedgerIndex"].clone();

    result["fields"] = an["FinalFields"].clone();

    result["fieldsPrev"] = an["PreviousFields"].clone();
    result["fieldsNew"] = an["NewFields"].clone();
    result["fieldsFinal"] = an["FinalFields"].clone();
    result["PreviousTxnID"] = an["PreviousTxnID"].clone();
}

pub fn is_amount_zero(amount: &Value) -> bool {
    if amount.is_null() {
        return false;
    }

    return amount.as_f64().unwrap() < std::f64::EPSILON;
}

pub fn amount_subtract(amount1: &Value, amount2: &Value) -> Value {
    if amount1.is_null() { return amount2.clone(); }
    if amount2.is_null() { return amount1.clone(); }

    let mut v: Value = amount1.clone();
    v["currency"] = amount1["currency"].clone();
    v["issuer"] = amount1["issuer"].clone();
    v["value"] = json!( amount1["value"].as_f64().unwrap() - amount2["value"].as_f64().unwrap() );

    v
}

pub fn amount_ratio(amount1: &Value, amount2: &Value) -> Value {
    println!("amount_ratio: 1: {:?} / 2: {:?}", &amount1, &amount2);
    let v1 = amount1["value"].clone().as_f64().unwrap();
    let v2 = amount2["value"].clone().as_f64().unwrap();

    let result: Value = json!( v1/v2 );

    result
}

pub fn get_price(effect: &Value, funded: bool) -> Value {
    println!("get price: {:?}", effect);

    let mut g = effect.clone();
    if effect["got"].is_null() {
        g = effect["pays"].clone();
    } else {
        g = effect["got"].clone();
    }

    println!("mid");

    let mut p = effect.clone();
    if effect["paid"].is_null() {
        p = effect["gets"].clone();
    } else {
        p = effect["paid"].clone();
    }
    println!("mid 1");

    if ! funded {
        return amount_ratio(&g, &p);
    } else {
        return amount_ratio(&p, &g);
    }

    println!("end");

}

//
pub fn txn_type(tx: &Value, account: &str) -> String {
    let tx_account = tx["Account"].as_str().unwrap();
    let tx_target = &tx["Target"];

    if tx_account == account ||
       ! tx_target.is_null() && tx_target.as_str().unwrap() == account ||
       ! tx["Destination"].is_null() && tx["Destination"].as_str().unwrap() == account ||
       ! tx["LimitAmount"].is_null() && tx["LimitAmount"]["issuer"].as_str().unwrap() == account {

        match tx["TransactionType"].as_str().unwrap() {
            "Payment" => {
                if tx["Account"].as_str().unwrap() == account {
                    if tx["Destination"].as_str().unwrap() == account {
                        return "convert".to_owned();
                    } else {
                        return "sent".to_owned();
                    }
                } else {
                    return "received".to_owned();
                }
            },

            "OfferCreate" => {
                return "offernew".to_owned();
            },

            "OfferCancel" => {
                return "offercancel".to_owned();
            },

            "TrustSet" => {
                if tx["Account"].as_str().unwrap() == account {
                    return "trusting".to_owned();
                }

                return "trusted".to_owned();
            },

            "RelationDel" | "AccountSet" | "SetRegularKey" | "RelationSet" | "SignSet" | "Operation" | "ConfigContract" | "AlethContract" | "Brokerage" => {
                return tx["TransactionType"].as_str().unwrap().to_ascii_lowercase();
            },

            _ => {
                return "unknown".to_owned();
            }
        }
    } else {
        return "offereffect".to_owned();
    }
}

pub fn type_result(x: &Value, result: &mut Value) {
    println!("type : {}", result["type"].to_string());
    match result["type"].as_str().unwrap() {
        "sent" => {
            result["counterparty"] = x["Destination"].clone();
            result["amount"] = parse_amount(&x["Amount"]);
        },

        "received" => {
            result["counterparty"] = x["Account"].clone();
            result["amount"] = parse_amount(&x["Amount"]);
        },

        "trusted" => {
            result["counterparty"] = x["Account"].clone();
            result["amount"] = reverse_amount(&x["LimitAmount"]);
        },

        "trusting" => {
            result["counterparty"] = x["LimitAmount"]["issuer"].clone();
            result["amount"] = x["LimitAmount"].clone();
        },

        "convert" => {
            result["spent"] = parse_amount(&x["SendMax"]);
            result["amount"] = parse_amount(&x["Amount"]);
        },

        "offernew" => {
            if x["Flags"].as_u64().unwrap() & 0x00080000 as u64 != 0 {
                result["offertype"] = json!("sell");
            } else {
                result["offertype"] = json!("buy");
            }

            result["gets"] = parse_amount(&x["TakerGets"]);
            result["pays"] = parse_amount(&x["TakerPays"]);

            result["seq"] = x["Sequence"].clone();

            let v1 = result["pays"]["value"].clone().as_f64().unwrap();
            let v2 = result["gets"]["value"].clone().as_f64().unwrap();

            if result["offertype"] == json!("sell") {
                result["price"] = json!( v1/v2 );
            } else {
                result["price"] = json!( v2/ v1);
            }
        },

        "offercancel" => {
            result["offerseq"] = x["Sequence"].clone();
        },

        "relationset" => {
            if x["Account"] == x["Target"] {
                result["counterparty"] = x["Account"].clone();
                result["isactive"] = json!(false);
            } else {
                result["counterparty"] = x["Target"].clone();
                result["isactive"] = json!(true);
            }

            if let Some(tp) = x["RelationType"].as_i64() {
                if tp == 3 {
                    result["relationtype"] = json!("freeze");
                } else {
                    result["relationtype"] = json!("authorize");
                }
            }

            result["amount"] = parse_amount(&x["LimitAmount"]);
        },

        "relationdel" => {
            if x["Account"] == x["Target"] {
                result["counterparty"] = x["Account"].clone();
                result["isactive"] = json!(false);
            } else {
                result["counterparty"] = x["Target"].clone();
                result["isactive"] = json!(true);
            }

            if let Some(tp) = x["RelationType"].as_i64() {
                if tp == 3 {
                    result["relationtype"] = json!("unfreeze");
                } else {
                    result["relationtype"] = json!("unknown");
                }
            }

            result["amount"] = parse_amount(&x["LimitAmount"]);
        },

        "configcontract" => {
            if x["Method"].as_i64() == Some(0) {
                result["method"] = json!("deploy");
                result["payload"] = x["Payload"].clone();
            } else if x["Method"].as_i64() == Some(1) {
                result["method"] = json!("call");
                result["destination"] = x["Destination"].clone();
            }
        },

        "alethcontract" => {
            if x["Method"].as_u64() == Some(0u64) {
                result["method"] = json!("deploy");
                result["seq"] = x["Sequence"].clone();
                result["payload"] = x["Payload"].clone();
            } else if x["Method"].as_u64() == Some(1u64) {
                result["method"] = json!("call");
                result["seq"] = x["Sequence"].clone();
                result["destination"] = x["Destination"].clone();
                result["amount"] = x["Amount"].clone();

                let method = hexcast::decode(x["MethodSignature"].clone().as_str().unwrap()).unwrap();
                let method = String::from_utf8_lossy(&method);
                let v: Vec<_>  = method.match_indices("(").collect();
                let v1: Vec<_>  = method.match_indices(")").collect();
                let idx1 = v[0].0;
                let idx2 = v1[0].0;

                result["func"] = json!( method.get(0..idx1).unwrap() );

                let ret= method.get(idx1+1..idx2).unwrap();
                let v: Vec<&str> = ret.split(',').collect();
                if v.len() == 0 {
                    result["func_parms"] = json!([]);
                } else {
                    let mut idx = 0;
                    while idx < v.len() {
                        result["func_parms"][idx] = json!( v[idx] );

                        idx += 1;
                    }
                }
            }
        },

        "brokerage" => {
            result["feeAccount"] = x["FeeAccountID"].clone();

            result["mol"] = x["OfferFeeRateNum"].clone();
            result["den"] = x["OfferFeeRateDen"].clone();

            result["amount"] = parse_amount(&x["Amount"]);
            result["seq"] = x["Sequence"].clone();
        },

        _ => {
            // TODO parse other type
        }
    }
}
///
pub fn process_tx(tx: &str) -> Value {
    let mut result: Value = json!({"An": "Object"});

    if let Ok(x) = serde_json::from_str(tx) as Result<Value, serde_json::error::Error> {
        //account
        let account = x["Account"].as_str().unwrap();

        //date
        if ! x["date"].is_null() {
            let timestamp = x["date"].as_i64().unwrap() + OFFSET_SECOND;
            result["date"] = json!(timestamp);
        } else if ! x["timestamp"].is_null() {
            let timestamp = x["timestamp"].as_i64().unwrap() + OFFSET_SECOND;
            result["date"] = json!(timestamp);
        }

        //hash
        if ! x["hash"].is_null() {
            result["hash"] = json!(x["hash"]);
        }

        //type
        result["type"] = json!(txn_type(&x, &account));

        //fee
        result["fee"] = json!(x["Fee"].as_str().unwrap().parse::<f64>().unwrap() / 1000000.0);

        //result
        if ! x["meta"].is_null() {
            result["result"] = json!(x["meta"]["TransactionResult"]);
        } else {
            result["result"] = json!("failed");
        }

        //type result
        type_result(&x, &mut result);

        //memos
        if x["Meta"].is_array() && x["Memos"].as_array().unwrap().len() > 0 {
            let mut m = 0;
            while m < x["Memos"].as_array().unwrap().len() {
                let memo_s = x["Memos"][m]["Memo"].clone();
                let mut memo = memo_s;
                result["memos"][m] = memo;

                m += 1;
            }
        }

        result["effects"] = json!([]);
        // no effect, return now
        if x["meta"].is_null() || x["meta"]["TransactionResult"] != json!("tesSUCCESS") {
            return result;
        }

        let mut cos: Vec<Value> = vec![];
        let mut gets_value = 0;
        let mut pays_value = 0;
        let mut total_rate = 0;

        if ! result["gets"].is_null() {
            cos.push( result["gets"]["currency"].clone());
            cos.push( result["pays"]["currency"].clone());
        }

        result["balances"] = json!([]);
        result["balancesPrev"] = json!([]);

        let affected_nodes = x["meta"]["AffectedNodes"].clone();
        let mut n = 0;
        while n < affected_nodes.as_array().unwrap().len() {
            println!("--------------------------------------------------------------------------new loop----------------------");
            let node_src = affected_nodes[n].clone();
            let mut node = json!({"diffType": "node"});
            process_affect_node(&node_src, &mut node);
            let mut effect = json!({});

            println!("nod: {:?}", node.clone());

            if ! node.is_null() && node["entryType"] == json!("Offer") {

                let field_set = node["fields"].clone();
                let mut sell = 0;
                if ! node["fields"]["Flags"].is_null() {
                    sell = node["fields"]["Flags"].clone().as_u64().unwrap() & 0x00080000;
                }

                println!("---------------------------------pin................");
                println!("fields.account: {:?}", node["fields"]["Account"]);
                println!("account: {}", &account);
                if node["fields"]["Account"] == account {
                    if node["diffType"] == json!("ModifiedNode") ||
                       ( node["diffType"] == json!("DeletedNode") &&
                       ! node["fieldsPrev"]["TakerGets"].is_null() &&
                       ! is_amount_zero(&parse_amount(&node["fieldsFinal"]["TakerGets"])) ) {

                        effect["effect"] = json!("offer_partially_funded");
                        effect["counterparty"]["account"] = x["Account"].clone();
                        effect["counterparty"]["seq"] = x["Sequence"].clone();
                        effect["counterparty"]["hash"] = x["hash"].clone();

                        if node["diffType"] != json!("DeletedNode") {
                            effect["remaining"] = json!( ! is_amount_zero(&parse_amount(&node["fields"]["TakerGets"])) );
                        } else {
                            effect["cancelled"] = json!(true);
                        }

                        effect["gets"] = parse_amount(&field_set["TakerGets"]);
                        effect["pays"] = parse_amount(&field_set["TakerPays"]);
                        effect["got"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerPays"]), &parse_amount(&node["fields"]["TakerPays"]));
                        effect["paid"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerGets"]), &parse_amount(&node["fields"]["TakerGets"]));

                        if sell != 0 {
                            effect["type"] = json!("sold");
                        } else {
                            effect["type"] = json!("bought");
                        }

                        if ! node["fields"]["OfferFeeRateNum"].is_null() {
                            effect["platform"] = node["fields"]["Platform"].clone();

                            let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                            let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                            effect["rate"] = json!( v1 / v2 );
                        }
                    } else {
                        if node["diffType"] == json!("CreatedNode") {
                            effect["effect"] = json!("offer_created");
                        } else {
                            if node["fieldsPrev"]["TakerPays"].is_null() {
                                effect["effect"] = json!("offer_cancelled");
                            } else {
                                effect["effect"] = json!("offer_funded");
                            }
                        }

                        // 2. offer_funded
                        if effect["effect"] == json!("offer_funded") {
                            let field_set = node["fieldsPrev"].clone();

                            effect["counterparty"]["account"] = x["Account"].clone();
                            effect["counterparty"]["seq"] = x["Sequence"].clone();
                            effect["counterparty"]["hash"] = x["hash"].clone();

                            effect["got"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerPays"]), &parse_amount(&node["fields"]["TakerPays"]));
                            effect["paid"] = effect["got"].clone();
                            if sell != 0 {
                                effect["type"] = json!("sold");
                            } else {
                                effect["type"] = json!("bought");
                            }

                            if ! node["fields"]["OfferFeeRateNum"].is_null() {
                                effect["platform"] = node["fields"]["Platform"].clone();

                                let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                                let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                                effect["rate"] = json!( v1 / v2 );
                            }
                        }
                        // 3. offer_created
                        if effect["effect"] == json!("offer_created") {
                            effect["gets"] = parse_amount(&field_set["TakerGets"]);
                            effect["pays"] = parse_amount(&field_set["TakerPays"]);
                            if sell != 0 {
                                effect["type"] = json!("sell");
                            } else {
                                effect["type"] = json!("buy");
                            }

                            if ! field_set["OfferFeeRateNum"].is_null() {
                                effect["platform"] = field_set["Platform"].clone();

                                let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                                let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                                effect["rate"] = json!( v1 / v2 );
                            }
                        }
                        // 4. offer_cancelled
                        if effect["effect"] == json!("offer_cancelled") {
                            effect["hash"] = node["fields"]["PreviousTxnID"].clone();
                            // collect data for cancel transaction type
                            if result["type"] == json!("offercancel") {
                                result["gets"] = parse_amount(&field_set["TakerGets"]);
                                result["pays"] = parse_amount(&field_set["TakerPays"]);
                            }

                            effect["gets"] = parse_amount(&field_set["TakerGets"]);
                            effect["pays"] = parse_amount(&field_set["TakerPays"]);

                            if sell != 0 {
                                effect["type"] = json!("sell");
                            } else {
                                effect["type"] = json!("buy");
                            }

                            if !field_set["OfferFeeRateNum"].is_null() {
                                effect["platform"] = field_set["Platform"].clone();
                                let v1 = field_set["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                                let v2 = field_set["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                                effect["rate"] = json!( v1 / v2 );
                            }
                        }
                    }

                    effect["seq"] = node["fields"]["Sequence"].clone();
                }
                // 5. offer_bought
                else if x["Account"] == account && ! node["fieldsPrev"].is_null() {

                    effect["effect"] = json!("offer_bought");

                    effect["counterparty"]["account"] = node["Account"].clone();
                    effect["counterparty"]["seq"] = node["Sequence"].clone();
                    effect["counterparty"]["hash"] = node["hash"].clone();

                    effect["paid"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerPays"]), &parse_amount(&node["fields"]["TakerPays"]));
                    effect["got"] = amount_subtract(&parse_amount(&node["fieldsPrev"]["TakerGets"]), &parse_amount(&node["fields"]["TakerGets"]));

                    if result["offertype"] == json!("buy") && sell != 0 || result["offertype"] == json!("sell") && !(sell != 0 ) as bool {
                        if sell != 0 {
                            effect["type"] = json!("bought");
                        } else {
                            effect["type"] = json!("sold");
                        }

                    }else {
                        if sell != 0 {
                            effect["type"] = json!("sold");
                        } else {
                            effect["type"] = json!("bought");
                        }
                    }
                }
                // add price
                if ! effect["gets"].is_null() && ! effect["pays"].is_null() || ! effect["got"].is_null() && ! effect["paid"].is_null() {

                    let mut created = false;
                    if effect["effect"] == json!("offer_created") && effect["type"] == json!("buy") {
                        created = true;
                    }

                    let mut funded = false;
                    if effect["effect"] == json!("offer_funded") && effect["type"] == json!("bought") {
                        funded = true;
                    }

                    let mut cancelled = false;
                    if effect["effect"] == json!("offer_cancelled") && effect["type"] == json!("buy") {
                        cancelled = true;
                    }

                    let mut bought = false;
                    if effect["effect"] == json!("offer_bought") && effect["type"] == json!("bought") {
                        bought = true;
                    }

                    let mut partially_funded = false;
                    if effect["effect"] == json!("offer_partially_funded") && effect["type"] == json!("bought") {
                        partially_funded = true;
                    }

                    effect["price"] = get_price(&effect, created || funded || cancelled || bought ||  partially_funded );

                    println!("------------price");
                }
            }

            println!("result type: {}", result["type"]);
            println!("node entryType: {}", node["entryType"]);

            if result["type"] == json!("offereffect") && ! node.is_null() && node["entryType"] == json!("AccountRoot") {

                if node["fields"]["RegularKey"] == account {
                    effect["effect"] = json!("set_regular_key");
                    effect["type"] = json!("null");
                    effect["account"] = node["fields"]["Account"].clone();
                    effect["regularkey"] = json!(account);
                }
            }

            if ! node.is_null() && node["entryType"] == json!("Brokerage") {
                result["platform"] = node["fields"]["Platform"].clone();

                let v1 = node["fields"]["OfferFeeRateNum"].clone().as_f64().unwrap();
                let v2 = node["fields"]["OfferFeeRateDen"].clone().as_f64().unwrap();

                result["rate"] = json!( v1 / v2 );
            }

            if ! node.is_null() && node["entryType"] == json!("SkywellState") {//其他币种余额
                println!("node: {:?}", node);

                if node["fields"]["HighLimit"]["issuer"] == account || node["fields"]["LowLimit"]["issuer"] == account {
                    if let Some(cny) = node["fields"]["Balance"]["currency"].clone().as_str() {
                        let cny = json!( {cny: node["fields"]["Balance"]["value"].clone().as_str().unwrap().parse::<f64>().unwrap().abs() } );
                        result["balances"].as_array_mut().unwrap().push( cny.clone() );
                    }

                    println!("skkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk1111111111111");
                    if ! node["fieldsPrev"]["Balance"].is_null() {

                        if let Some(cnn) = node["fieldsPrev"]["Balance"]["currency"].clone().as_str() {
                            let v = json!( {cnn: node["fieldsPrev"]["Balance"]["value"].clone().as_str().unwrap().parse::<f64>().unwrap().abs() } );
                            result["balancesPrev"].as_array_mut().unwrap().push( v.clone() );
                        }

                    } else if ! node["fieldsNew"]["Balance"].is_null() {//新增币种

                        if let Some(cnn) = node["fields"]["Balance"]["currency"].clone().as_str() {
                            result["balancesPrev"].as_array_mut().unwrap().push( json!({cnn: 0}) );
                        }
                    } else {
                    }
                }
            }

            if !node.is_null() && node["entryType"] == json!("AccountRoot") {//基础币种余额
                  if node["fields"]["Account"] == account {

                      println!("基础货币: {:?}", node);
                      if let Ok(x) = node["fields"]["Balance"].clone().as_str().unwrap().parse::<f64>() {
                          let v = json!( { CURRENCY : x/1000000.0 } );
                          result["balances"].as_array_mut().unwrap().push( v );

                      } else {
                          let v = json!( { CURRENCY : 0 } );
                          result["balances"].as_array_mut().unwrap().push( v );
                      }
                      println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA bbb");

                      if ! node["fieldsPrev"]["Balance"].is_null() {
                          if let Ok(x) =  node["fieldsPrev"]["Balance"].clone().as_str().unwrap().parse::<f64>() {
                              let v = json!( {CURRENCY: x / 1000000.0} );
                              result["balancesPrev"].as_array_mut().unwrap().push( v );
                          } else {
                              let v = json!( {CURRENCY: 0.0} );
                              result["balancesPrev"].as_array_mut().unwrap().push( v );
                          }

                      } else if ! node["fieldsNew"]["Balance"].is_null() {

                          result["balancesPrev"].as_array_mut().unwrap().push( json!( {CURRENCY: 0 } ));

                      } else {
                      }
                  }
            }

            // add effect
            if ! effect.is_null() {

                println!("AccountRootAccountRootAccountRootAccountRootAccountRootAccountRootAccountRootAccountRootAccountRootAccountRootAccountRootAccountRoot");

                if node["diffType"] == json!("DeletedNode") && effect["effect"] != json!("offer_bought") {
                    effect["deleted"] = json!(true);
                }

                result["effects"].as_array_mut().unwrap().push(  effect.clone() );
            }

            println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

            if result["type"] == json!("offernew") && ! effect["got"].clone().is_null() {
                println!("offfffffffffffffffffffffnewwwwwwwwwwwwwwwwwww");
                    // println!("result: {:?}", result);
                    println!("effect: {:?}", effect);
                    if result["gets"]["currency"] == effect["paid"]["currency"] {
                        gets_value = ( effect["paid"]["value"].clone().as_f64().unwrap() + gets_value as f64 ) as u64;
                    }

                    // let mut pays_value1 = 0;
                    if result["pays"]["currency"] == effect["got"]["currency"] {
                        gets_value = ( effect["got"]["value"].clone().as_f64().unwrap() + pays_value as f64 ) as u64;
                    }

                    if result["gets"]["currency"] != effect["paid"]["currency"] || result["pays"]["currency"] != effect["got"]["currency"] {
                        if ! cos.contains(&effect["got"]["currency"]) {
                            cos.push(effect["got"]["currency"].clone());
                        }

                        if ! cos.contains(&effect["paid"]["currency"]) {
                            cos.push(effect["paid"]["currency"].clone());
                        }
                    }
                }

                n += 1;
        } // end while

    } else {
        panic!("Error input.");
    }

    result
}
