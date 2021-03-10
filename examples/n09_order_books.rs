extern crate jlib;

use jlib::api::order_books::api::request;
use jlib::api::order_books::data::{
    RequestOrderBookResponse, OrderBookItem, OrderBookSideKick
};

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    
    let gets = OrderBookItem::with_params("SWT".to_string(), "".to_string());
    let pays = OrderBookItem::with_params("CNY".to_string(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string());

    request(config, gets, pays, |x| match x {
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
    
