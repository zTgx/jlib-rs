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
use crate::base::misc::util::downcast_to_string;

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

            let status: String = x["status"].to_string();
            if status == "\"success\"" {
                let result = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&result) as Result<RequestAccountTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }

            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<AccounTxSideKick, serde_json::error::Error> {
                    op(Err(v))
                }
            }
        }
    }
}
