//
// 取消挂单
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::transaction::offer_cancel::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

pub trait CancelOfferI {
    fn cancel_offer<F>(&self, config: Box<Rc<Config>>, account: String, offer_sequence: u64, 
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<OfferCancelTxResponse, serde_json::error::Error>);
}

pub struct CancelOffer {}
impl CancelOffer {
    pub fn new() -> Self {
        CancelOffer {
        }
    }
}

impl CancelOfferI for CancelOffer { 
    fn cancel_offer<F>(&self, config: Box<Rc<Config>>, account: String, offer_sequence: u64, 
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<OfferCancelTxResponse, serde_json::error::Error>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let offer_sequence_rc = Rc::new(Cell::new(offer_sequence));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let offer_sequence = offer_sequence_rc.clone();
            let secret = secret_rc.clone();

            if let Ok(command) = OfferCancelTx::new(secret.take(), OfferCancelTxJson::new(account.take(),  offer_sequence.take())).to_string() {
                out.send(command).unwrap()
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = downcast_to_string(info);
        println!("resp : {}", &resp);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            if let Ok(v) = serde_json::from_str(&x) as Result<OfferCancelTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }
}