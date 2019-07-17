//
// 请求账号挂单
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::offer::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

pub trait AccountOfferI {
    fn request_account_offer<F>(&self, config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountOfferResponse, serde_json::error::Error>);
}

pub struct AccountOffer {}
impl AccountOffer {
    pub fn new() -> Self {
        AccountOffer {
        }
    }
}

impl AccountOfferI for AccountOffer { 
        fn request_account_offer<F>(&self, config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountOfferResponse, serde_json::error::Error>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));
            connect(config.addr, |out| { 
                let copy = info.clone();
                let account = account_rc.clone();
                if let Ok(command) = RequestAccountOfferCommand::with_params(account.take()).to_string() {
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
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountOfferResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }
}