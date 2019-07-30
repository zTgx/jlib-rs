//
// 请求账号信息
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::subscribe::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;

pub trait SubscribeI {
    fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
    where F: Fn(Result<SubscribeResponse, serde_json::error::Error>);
}

pub struct Subscribe {}
impl Subscribe {
    pub fn new() -> Self {
        Subscribe {
        }
    }
}

impl SubscribeI for Subscribe {
    fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
    where F: Fn(Result<SubscribeResponse, serde_json::error::Error>) {

        let ws_message = Rc::new(Cell::new("".to_string()));

        connect(config.addr, |out| {

            let cloned_ws_message = ws_message.clone();

            if let Ok(command) = SubscribeCommand::default().to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let text = msg.as_text()?;
                cloned_ws_message.set(text.to_string());

                out.close(CloseCode::Normal)
            }
        }).unwrap();

        let resp = downcast_to_string(ws_message);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            if let Ok(x) = serde_json::from_str(&x) as Result<SubscribeResponse, serde_json::error::Error> {
                println!("subscribe response : {:?}", x);

                op(Ok(x))
            }
        }
    }
}
