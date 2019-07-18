//
// 获取某一账本具体信息
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::spec_ledger::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

pub trait SpecLedgerI {
    fn request_ledger<F>(&self, config: Box<Rc<Config>>, ledger_index: Option<u64>, ledger_hash: Option<String>, transactions: bool, op: F) 
    where F: Fn(Result<RequestLedgerResponse, SpecLedgerSideKick>);
}

pub struct SpecLedger {}
impl SpecLedger {
    pub fn new() -> Self {
        SpecLedger {
        }
    }
}

impl SpecLedgerI for SpecLedger { 
        fn request_ledger<F>(&self, config: Box<Rc<Config>>, ledger_index: Option<u64>, ledger_hash: Option<String>, transactions: bool, op: F) 
        where F: Fn(Result<RequestLedgerResponse, SpecLedgerSideKick>) {

            let info = Rc::new(Cell::new("".to_string()));

            let ledger_index_rc = Rc::new(Cell::new(None));
            if ledger_index.is_some() {
                ledger_index_rc.set(ledger_index);
            }
            let ledger_hash_rc = Rc::new(Cell::new(None));
            if ledger_hash.is_some() {
                ledger_hash_rc.set(ledger_hash);
            }
            let transactions_rc = Rc::new(Cell::new(transactions));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let index = ledger_index_rc.clone();
                let hash = ledger_hash_rc.clone();
                let trans = transactions_rc.clone();
                if let Ok(command) = RequestLedgerCommand::with_params(index.take(), hash.take(), trans.take()).to_string() {
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
                let status = x["status"].to_string().get(1..8).unwrap().to_string();
                if status == "success" {
                    let x: String = x["result"].to_string();
                    if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                        let x: String = x["ledger"].to_string();
                        if let Ok(v) = serde_json::from_str(&x) as Result<RequestLedgerResponse, serde_json::error::Error> {
                            op(Ok(v))
                        }
                    }
                } else  {
                    println!("err");
                    if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<SpecLedgerSideKick, serde_json::error::Error> {
                        op(Err(v))
                    }
                }
            }         
    }
}