extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::order_book::*;
use jlib::message::query::order_book::{RequestOrderBookResponse, OrderBookItem, OrderBookSideKick};

fn main() {
    let config = Config::new(TEST1, true);
    let gets = OrderBookItem::with_params("SWT".to_string(), "".to_string());
    let pays = OrderBookItem::with_params("CNY".to_string(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string());
    OrderBook::new().request_order_book(config.clone(), gets, pays, |x| match x {
        Ok(response) => {
            let res: RequestOrderBookResponse = response;
            println!("挂单列表: {:?}",  &res);
        },

        Err(e) => {
            let err: OrderBookSideKick = e;
            println!("err: {:?}", err);
        }   
    });
}
    
