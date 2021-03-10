extern crate jlib;

use jlib::api::subscription::data::SubscribeResponse;
use jlib::api::subscription::api::on;

use jlib::api::config::Config;
// static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";
static TEST_SERVER: &'static str = "ws://59.175.148.101:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    on(config, |x| {
        match x {
            Ok(response) => {
                let res: SubscribeResponse = response;
                println!("Response fee_base : {}", res.fee_base);
            },
            Err(err) => { println!("error: {}", err); }
        }
    });
}
