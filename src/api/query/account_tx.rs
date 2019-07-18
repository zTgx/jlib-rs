//
// 账号交易列表
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::account_tx::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;
use crate::misc::error::AccounTxSideKick;

pub trait AccountTxI {
    fn request_account_tx<F>(&self, config: Box<Rc<Config>>, account: String, limit: Option<u64>, op: F) 
    where F: Fn(Result<RequestAccountTxResponse, AccounTxSideKick>);
}

pub struct AccountTx {}
impl AccountTx {
    pub fn new() -> Self {
        AccountTx {
        }
    }
}

impl AccountTxI for AccountTx { 
    fn request_account_tx<F>(&self, config: Box<Rc<Config>>, account: String, limit: Option<u64>, op: F) 
    where F: Fn(Result<RequestAccountTxResponse, AccounTxSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let limit_rc = Rc::new(Cell::new(limit));
        connect(config.addr, |out| { 
            let copy = info.clone();
            let account = account_rc.clone();
            let limit = limit_rc.clone();
            if let Ok(command) = RequestAccountTxCommand::with_params(account.take(), limit.take()).to_string() {
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
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let result: String = x["result"].to_string();
            let status: String = x["status"].to_string();

            println!("status: {}", &status);
            println!("result: {}", &result.len());
            //check status
            if status == "success".to_owned() && result.len() != 0 {
                println!("success.");
                //parse success data
                if let Ok(v) = serde_json::from_str(&result) as Result<RequestAccountTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }

            } else {
                println!("error.");
                // println!("error: {:?}", x.to_string());
                //parse error data
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<AccounTxSideKick, serde_json::error::Error> {
                    op(Err(v))
                }
            }

            
            // else {
            //     // let y: &'static ServerInfoSideKick = &AccountInfoSideKick{};
            //     // let e = SuperError { side: y };
            //     // println!("Error: {}", e.description());
            //     // println!("caused by : {}", e.source().unwrap());

            // }
        }         

        /*
        resp: "{\"error\":\"actMalformed\",\"error_code\":33,\"error_message\":\"Account malformed.\",\"id\":1,\"request\":{\"account\":\"jB8rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ\",\"command\":\"account_tx\",\"id\":1,\"ledger_index_max\":-1,\"ledger_index_min\":0,\"limit\":1},\"status\":\"error\",\"type\":\"response\"}\n"

        */
    }
}