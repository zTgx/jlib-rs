//
// 获得账号可接收和发送的货币
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::account_tums::*;
use crate::message::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

pub trait AccountTumsI {
    fn request_account_tums<F>(&self, config: Box<Rc<Config>>, account: String, op: F) 
    where F: Fn(Result<RequestAccountTumsResponse, serde_json::error::Error>) ;
}

pub struct AccountTums {}
impl AccountTums {
    pub fn new() -> Self {
        AccountTums {
        }
    }
}

impl AccountTumsI for AccountTums { 
        fn request_account_tums<F>(&self, config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountTumsResponse, serde_json::error::Error>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                if let Ok(command) = RequestAccountTumsCommand::with_params(account.take()).to_string() {
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
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountTumsResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }
}