extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::subscribe::*;
use jlib::message::query::subscribe::{SubscribeResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    Subscribe::new().with_config(config.clone(), |x| { 
        match x {
            Ok(response) => { 
                let res: SubscribeResponse = response;
                println!("Response fee_base : {}", res.fee_base);
            }

            Err(err) => { println!("error: {}", err); }
        }
    });
}
