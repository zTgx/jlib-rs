//
// 获取市场挂单列表
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::order_book::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;

pub trait OrderBookI {
    fn request_order_book<F>(&self, config: Box<Rc<Config>>, gets: OrderBookItem, pays: OrderBookItem, op: F)
    where F: Fn(Result<RequestOrderBookResponse, OrderBookSideKick>) ;
}

pub struct OrderBook {}
impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
        }
    }
}

impl OrderBookI for OrderBook {
    fn request_order_book<F>(&self, config: Box<Rc<Config>>, gets: OrderBookItem, pays: OrderBookItem, op: F)
    where F: Fn(Result<RequestOrderBookResponse, OrderBookSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let gets_rc = Rc::new(Cell::new(gets));
        let pays_rc = Rc::new(Cell::new(pays));
        connect(config.addr, |out| {
            let copy = info.clone();

            let gets = gets_rc.clone();
            let pays = pays_rc.clone();

            //使用take（）的对象要实现Default trait，因为 take（） 调用后，原始值会调用default（）
            //OrderBookItem, add #[derive(Default)] or impl default trait.
            if let Ok(command) = RequestOrderBookCommand::with_params(gets.take(), pays.take()).to_string() {
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
            let status = x["status"].to_string();
            if status == "\"success\"" {
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
