use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::Value;

use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;
use crate::api::config::Config;
use api::server_info::data::ServerInfoCommand;
use crate::api::server_info::data::ServerInfoResponse;

pub fn request<F> (config: Config, op: F)
    where F: Fn(Result<ServerInfoResponse, serde_json::error::Error>) {

    let info = Rc::new(Cell::new(String::new()));

    connect(config.addr, |out| {
        let copy = info.clone();

        if let Ok(command) = ServerInfoCommand::default().to_string() {
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
                    let x: String = x["info"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<ServerInfoResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }
        }
    }
}
