use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::base::misc::util::downcast_to_string;
use crate::api::config::Config;

use crate::api::nth_tx::data::{
    RequestTxCommand,
    RequestTxResponse,
    SpecTxSideKick
};

pub fn request<F>(config: Config, hash: String,  op: F)
    where F: Fn(Result<RequestTxResponse, SpecTxSideKick>) {

    let info = Rc::new(Cell::new(String::new()));

    let hash_rc = Rc::new(Cell::new(hash));

    connect(config.addr, |out| {
        let copy = info.clone();

        let hash = hash_rc.clone();

        if let Ok(command) = RequestTxCommand::with_params(hash.take()).to_string() {
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
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<SpecTxSideKick, serde_json::error::Error> {
                    op(Err(v))
                }
            }
        }
    }
}

