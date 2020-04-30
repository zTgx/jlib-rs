extern crate jlib;
use jlib::api::query::order_book::*;
use jlib::message::query::order_book::{RequestOrderBookResponse, OrderBookItem, OrderBookSideKick};

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let gets = OrderBookItem::with_params("SWT".to_string(), "".to_string());
    let pays = OrderBookItem::with_params("CNY".to_string(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string());
    OrderBook::new().request_order_book(config, gets, pays, |x| match x {
        Ok(response) => {
            let res: RequestOrderBookResponse = response;
            println!("order list: {:?}",  &res);
        },
        Err(e) => {
            let err: OrderBookSideKick = e;
            println!("err: {:?}", err);
        }   
    });
}
    
