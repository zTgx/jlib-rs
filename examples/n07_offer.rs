extern crate jlib;

use jlib::misc::config::*;
use jlib::api::account_offer::*;
use jlib::commands::offer::{RequestAccountOfferResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    AccountOffer::new().request_account_offer(config.clone(), account, |x| match x {
        Ok(response) => {
            let res: RequestAccountOfferResponse = response;
            println!("账号挂单: {:?}", &res);
        },

        Err(_) => {
            panic!("Error Message.");
        }   
    });    
}