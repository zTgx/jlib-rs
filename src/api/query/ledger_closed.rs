use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::message::query::ledger_closed::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;
use crate::Config;

pub trait LedgerClosedI {
    fn request_ledger_closed<F>(&self, config: Config, op: F)
        where
            F : Fn(Result<LedgerClosedResponse, LedgerClosedSideKick>);
}

pub struct LedgerClosed {}
impl LedgerClosed {
    pub fn new() -> Self {
        LedgerClosed {
        }
    }
}

impl LedgerClosedI for LedgerClosed {
    fn request_ledger_closed<F>(&self, config: Config, op: F)
        where F: Fn(Result<LedgerClosedResponse, LedgerClosedSideKick>) {
            let info = Rc::new(Cell::new("".to_string()));

            connect(config.addr, |out| {
                let copy = info.clone();

                if let Ok(command) = LedgerClosedCommand::default().to_string() {
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
                        if let Ok(x) = serde_json::from_str(&x) as Result<LedgerClosedResponse, serde_json::error::Error> {
                            op(Ok(x));
                        }
                    }
                } else {
                    if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<LedgerClosedSideKick, serde_json::error::Error> {
                        op(Err(v))
                    }
                }
            }
    }
}
