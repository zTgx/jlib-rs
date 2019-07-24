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
use crate::message::common::amount::Amount;
use RelationType;
use crate::api::query::account_info::*;
use crate::message::transaction::local_sign_tx::{LocalSignTx};
use crate::base::sign_tx::{SignTx};
use crate::base::util::{downcast_to_usize, downcast_to_string};

pub trait RelateI {
    fn set_relation<F>(&self, relation_type: RelationType, target: String, amount: Amount, op: F)
    where F: Fn(Result<RelationTxResponse, RelationSideKick>);
}

pub struct Relate {
    pub config : Box<Rc<Config>>,
    pub account: String,
    pub secret : String,
}
impl Relate {
    pub fn with_params(config: Box<Rc<Config>>, account: String, secret: String) -> Self {
        Relate {
            config : config,
            account: account,
            secret : secret,
        }
    }

    pub fn get_account_seq(&self) -> u32 {
        let seq_rc = Rc::new(Cell::new(0u64));

        let acc = String::from(self.account.as_str());
        AccountInfo::new().request_account_info(self.config.clone(), acc, |x| match x {
            Ok(response) => {
                let seq = seq_rc.clone();
                seq.set(response.sequence);
            },
            Err(_) => { }
        });

       downcast_to_usize(seq_rc)
    }
}

impl RelateI for Relate { 
    fn set_relation<F>(&self, rtype: RelationType, target: String, amount: Amount, op: F) 
    where F: Fn(Result<RelationTxResponse, RelationSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(String::from(self.account.as_str())));
        let secret_rc  = Rc::new(Cell::new(String::from(self.secret.as_str())));

        let rtype_rc  = Rc::new(Cell::new(rtype.get()));
        let target_rc = Rc::new(Cell::new(target));

        let amount_rc = Rc::new(Cell::new(amount));
        
        connect(self.config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let secret = secret_rc.clone();

            let rtype  = rtype_rc.clone();
            let target = target_rc.clone();
            let amount = amount_rc.clone();

            let tx_json = RelationTxJson::new(account.take(), target.take(), rtype.take(), amount.take());
            if self.config.local_sign {
                let sequence = self.get_account_seq();
                let blob = SignTx::with_params(sequence, &secret.take()).relate(&tx_json);
                if let Ok(command) = LocalSignTx::new(blob).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = RelationTx::new(secret.take(), tx_json).to_string() {
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
            let status = x["status"].to_string();
            if status == "\"success\"" {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<RelationTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<RelationSideKick, serde_json::error::Error> {
                    op(Err(v))
                } 
            }            
        }         
    }
}