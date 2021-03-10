use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::base::misc::util::downcast_to_string;
use crate::api::config::Config;

use crate::api::order_books::data::{
    RequestOrderBookCommand,
    RequestOrderBookResponse,
    OrderBookSideKick,
    OrderBookItem
};

pub fn request<F>(config: Config, gets: OrderBookItem, pays: OrderBookItem, op: F)
    where F: Fn(Result<RequestOrderBookResponse, OrderBookSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let gets_rc = Rc::new(Cell::new(gets));
        let pays_rc = Rc::new(Cell::new(pays));
        connect(config.addr, |out| {
            let copy = info.clone();

            let gets = gets_rc.clone();
            let pays = pays_rc.clone();

            if let Ok(command) = RequestOrderBookCommand::with_params(gets.take(), pays.take()).to_string() {
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
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestOrderBookResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                } else {
                    if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<OrderBookSideKick, serde_json::error::Error> {
                        op(Err(v))
                    }
                }
            }
        }
}
