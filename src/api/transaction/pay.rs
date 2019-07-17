//
// 请求账号信息
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::transaction::transaction::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;
use crate::message::common::memo::*;
use crate::base::sign_tx::*;
use crate::message::common::amount::Amount;
use crate::base::util::{string_to_hex};

pub trait PaymentI {
    fn payment<F>(&self, config: Box<Rc<Config>>, from: String, to: String, amount: Amount, 
                                                    memo: Option<String>, 
                                                    sequence: Option<u32>,
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<TransactionTxResponse, serde_json::error::Error>);
}

pub struct Payment {}
impl Payment {
    pub fn new() -> Self {
        Payment {
        }
    }
}

impl PaymentI for Payment { 
    fn payment<F>(&self, config: Box<Rc<Config>>, from: String, to: String, amount: Amount, 
                                                    memo: Option<String>, 
                                                    sequence: Option<u32>,
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<TransactionTxResponse, serde_json::error::Error>) {
        //params check
        // var tx = new Transaction(this);
        // if (options === null || typeof options !== 'object') {
        //     tx.tx_json.obj = new Error('invalid options type');
        //     return tx;
        // }
        // var src = options.source || options.from || options.account;
        // var dst = options.destination || options.to;
        // var amount = options.amount;
        // if (!utils.isValidAddress(src)) {
        //     tx.tx_json.src = new Error('invalid source address');
        //     return tx;
        // }
        // if (!utils.isValidAddress(dst)) {
        //     tx.tx_json.dst = new Error('invalid destination address');
        //     return tx;
        // }
        // if (!utils.isValidAmount(amount)) {
        //     tx.tx_json.amount = new Error('invalid amount');
        //     return tx;
        // }

        let info = Rc::new(Cell::new("".to_string()));

        let from_rc = Rc::new(Cell::new(from));
        let to_rc = Rc::new(Cell::new(to));
        let amount_rc = Rc::new(Cell::new(amount));
        let memo_rc = Rc::new(Cell::new(None));
        if memo.is_some() {
            let mut v: Vec<Memo> = Vec::new();
            v.push(Memo::new(MemoData::new(string_to_hex(&memo.unwrap()))));

            memo_rc.set(Some(v));
        }

        let sequence_rc = Rc::new(Cell::new(None));
        if sequence.is_some() {
            sequence_rc.set(sequence);
        }

        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let from = from_rc.clone();
            let to   = to_rc.clone();
            let amount = amount_rc.clone();
            let memo   = memo_rc.clone();
            let sequence = sequence_rc.clone();

            let secret = secret_rc.clone();

            //txjson
            use crate::base::*;
            let mut signing_pub_key: Option<String> = None;

            let mut d_secret = "".to_string();
            if let Some(x) = secret.take() {
                signing_pub_key = Some(util::get_public_key_from_secret(&x).property.public_key);
                d_secret = String::from(x.as_str());
            }
            let tx_json = TxJson::new(from.take(), to.take(), amount.take(), memo.take(), sequence.take(), signing_pub_key);

            if config.local_sign {
                use crate::message::transaction::local_sign_tx::*;
                let mut local_sign = SignTx::default();

                let submit = local_sign.prepare(tx_json, d_secret);

                if let Ok(command) = LocalSignTx::new(secret.take(), submit.unwrap()).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = TransactionTx::new(Some(d_secret), tx_json).to_string() {
                    out.send(command).unwrap()
                }
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
            if let Ok(v) = serde_json::from_str(&x) as Result<TransactionTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         
    }
}