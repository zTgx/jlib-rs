use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::base::misc::util::downcast_to_string;
use crate::api::config::Config;

use crate::api::account_offers::data::{
    RequestAccountOfferCommand,
    RequestAccountOfferResponse, 
    AccountOffersSideKick
};

pub fn request<F>(config: Config, account: String, op: F)
    where F: Fn(Result<RequestAccountOfferResponse, AccountOffersSideKick>) {

        let info = Rc::new(Cell::new(String::new()));

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
            if let Some(status) = x["status"].as_str() {
                if status == "success" {
                    let x: String = x["result"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountOfferResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                } else {
                    if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<AccountOffersSideKick, serde_json::error::Error> {
                        op(Err(v))
                    }
                }
            }
        }
}
