extern crate jlib;
use jlib::api::query::brokerage::*;
use jlib::message::query::brokerage::{RequestBrokerageResponse, BrokerageSideKick};

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    Brokerage::new().request_brokerage(config, account, |x| match x {
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
