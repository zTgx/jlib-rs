use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::message::query::account_tums::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;
use crate::Config;

pub trait AccountTumsI {
    fn request_account_tums<F>(&self, config: Config, account: String, op: F)
    where F: Fn(Result<RequestAccountTumsResponse, AccounTumSideKick>) ;
}

pub struct AccountTums {}
impl AccountTums {
    pub fn new() -> Self {
        AccountTums {
        }
    }
}

impl AccountTumsI for AccountTums {
        fn request_account_tums<F>(&self, config: Config, account: String, op: F)
        where F: Fn(Result<RequestAccountTumsResponse, AccounTumSideKick>) {

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
                if let Some(status) = x["status"].as_str() {
                    if status == "success" {
                        let x: String = x["result"].to_string();
                        if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountTumsResponse, serde_json::error::Error> {
                            op(Ok(v))
                        }
                    } else {
                        if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<AccounTumSideKick, serde_json::error::Error> {
                            op(Err(v))
                        }
                    }
                }
            }
    }
}
