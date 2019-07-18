//
// 账号交易列表
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::brokerage::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

pub trait BrokerageI {
    fn request_brokerage<F>(&self, config: Box<Rc<Config>>, issuer: String, app: u64, currency: String, op: F) 
    where F: Fn(Result<RequestBrokerageResponse, serde_json::error::Error>) ;
}

pub struct Brokerage {}
impl Brokerage {
    pub fn new() -> Self {
        Brokerage {
        }
    }
}

impl BrokerageI for Brokerage { 
    fn request_brokerage<F>(&self, config: Box<Rc<Config>>, issuer: String, app: u64, currency: String, op: F) 
    where F: Fn(Result<RequestBrokerageResponse, serde_json::error::Error>) {

        let info = Rc::new(Cell::new("".to_string()));

        let issuer_rc = Rc::new(Cell::new(issuer));
        let app_rc = Rc::new(Cell::new(app));
        let currency_rc = Rc::new(Cell::new(currency));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let issuer = issuer_rc.clone();
            let app = app_rc.clone();
            let currency = currency_rc.clone();

            if let Ok(command) = RequestBrokerageCommand::with_params(issuer.take(), app.take(), currency.take()).to_string() {
                out.send(command).unwrap();
            }

            //返回一个Handler类型(trait)，等待epoll调用。
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
            if let Ok(v) = serde_json::from_str(&x) as Result<RequestBrokerageResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }       

        /*
        resp : {"error":"unknownCmd","error_code":30,"error_message":"Unknown method.","id":1,"request":{"app_type":1,"command":"Fee_Info","currency":"TES","id":1,"issuer":"jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS","ledger_index":"validated"},"status":"error","type":"response"}

        */  
    }
}