//
// 关系
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::transaction::relation::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;
use crate::message::common::amount::Amount;

pub trait RelateI {
    fn set_relation<F>(&self, config: Box<Rc<Config>>, account: String, target: String, relation_type: u64, amount: Amount, 
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<RelationTxResponse, serde_json::error::Error>);
}

pub struct Relate {}
impl Relate {
    pub fn new() -> Self {
        Relate {
        }
    }
}

impl RelateI for Relate { 
    fn set_relation<F>(&self, config: Box<Rc<Config>>, account: String, target: String, relation_type: u64, amount: Amount, 
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<RelationTxResponse, serde_json::error::Error>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let target_rc = Rc::new(Cell::new(target));
        let relation_type_rc = Rc::new(Cell::new(relation_type));
        let amount_rc = Rc::new(Cell::new(amount));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let target   = target_rc.clone();
            let relation_type = relation_type_rc.clone();
            let amount = amount_rc.clone();
            let secret = secret_rc.clone();

            if let Ok(command) = RelationTx::new(secret.take(), RelationTxJson::new(account.take(), target.take(), relation_type.take(), amount.take())).to_string() {
                out.send(command).unwrap()
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
            if let Ok(v) = serde_json::from_str(&x) as Result<RelationTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         
    }
}