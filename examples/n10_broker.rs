extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::brokerage::*;
use jlib::message::query::brokerage::{RequestBrokerageResponse, BrokerageSideKick};

fn main() {
    let config = Config::new(TEST1, true);
    let account = "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string();
    Brokerage::new().request_brokerage(config.clone(), account, 1, "TES".to_string(), |x| match x {
        Ok(response) => {
            let res: RequestBrokerageResponse = response;
            println!("佣金设置信息: {:?}", &res);
        },

        Err(e) => {
            let err: BrokerageSideKick = e;
            println!("err: {:?}", err);
        }   
    });
}
