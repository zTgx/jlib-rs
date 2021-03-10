extern crate jlib;

use jlib::api::fee_info::api::request;
use jlib::api::fee_info::data::{RequestBrokerageResponse, BrokerageSideKick};

use jlib::api::config::Config;

static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    
    request(config, account, |x| match x {
        Ok(response) => {
            let res: RequestBrokerageResponse = response;
            println!("brokerage response: {:?}", &res);
        },
        Err(e) => {
            let err: BrokerageSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
