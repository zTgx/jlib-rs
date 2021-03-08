use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};
use crate::message::query::server_info::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;
use crate::api::config::Config;

pub trait ServerInfoI {
    fn request_server_info<F>(&self, config: Config, op: F)
        where
            F : Fn(Result<ServerInfoResponse, serde_json::error::Error>);
}

pub struct ServerInfo{}
impl ServerInfo {
    pub fn new() -> Self {
        ServerInfo {

        }
    }
}

impl ServerInfoI for ServerInfo {
    fn request_server_info<F> (&self, config: Config, op: F)
        where F: Fn(Result<ServerInfoResponse, serde_json::error::Error>) {

        let info = Rc::new(Cell::new("".to_string()));
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
