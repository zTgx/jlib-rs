//
// 请求账号信息
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::account_info::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;

pub trait AccountInfoI {
    fn request_account_info<F>(&self, config: Box<Rc<Config>>, account: String, op: F)
    where F: Fn(Result<RequestAccountInfoResponse, AccounInfoSideKick>);
}

pub struct AccountInfo {}
impl AccountInfo {
    pub fn new() -> Self {
        AccountInfo {
        }
    }
}

impl AccountInfoI for AccountInfo {
        fn request_account_info<F>(&self, config: Box<Rc<Config>>, account: String, op: F)
        where F: Fn(Result<RequestAccountInfoResponse, AccounInfoSideKick>) {

            let info = Rc::new(Cell::new("".to_string()));
            let account_rc = Rc::new(Cell::new(account));

            connect(config.addr, |out| {
                let copy = info.clone();

                let account = account_rc.clone();
                if let Ok(command) = RequestAccountInfoCommand::with_params(account.take()).to_string() {
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
                if let Some(status) = x["status"].as_str() {
                    if status == "success" {
                        let x: String = x["result"].to_string();
                        if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                            let x: String = x["account_data"].to_string();
                            if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountInfoResponse, serde_json::error::Error> {
                                op(Ok(v))
                            }
                        }
                    } else {
                        if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<AccounInfoSideKick, serde_json::error::Error> {
                            op(Err(v))
                        }
                    }
                }
            }
    }
}
