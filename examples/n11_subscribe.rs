extern crate jlib;

use jlib::misc::config::*;
use jlib::message::query::subscribe::{SubscribeResponse};
use jlib::SUBSCRIBE;
use jlib::SubscribeI;

fn main() {
    let config = Config::new(TEST_SERVER, true);
    SUBSCRIBE.with_config(config.clone(), |x| {
        match x {
            Ok(response) => {
                let res: SubscribeResponse = response;
                println!("Response fee_base : {}", res.fee_base);
            },
            Err(err) => { println!("error: {}", err); }
        }
    });
}
