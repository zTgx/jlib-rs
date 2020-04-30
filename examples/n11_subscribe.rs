extern crate jlib;
use jlib::api::subscribe::{SUBSCRIBE, SubscribeI};
use jlib::api::subscribe::message::SubscribeResponse;

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    SUBSCRIBE.with_config(config, |x| {
        match x {
            Ok(response) => {
                let res: SubscribeResponse = response;
                println!("Response fee_base : {}", res.fee_base);
            },
            Err(err) => { println!("error: {}", err); }
        }
    });
}
