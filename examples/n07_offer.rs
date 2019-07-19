extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_offer::*;
use jlib::message::query::offer::{RequestAccountOfferResponse, AccountOffersSideKick};

fn main() {
    let config = Config::new(TEST1, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    AccountOffer::new().request_account_offer(config.clone(), account, |x| match x {
        Ok(response) => {
            let res: RequestAccountOfferResponse = response;
            println!("账号挂单: {:?}", &res);
        },

        Err(e) => {
            let err: AccountOffersSideKick = e;
            println!("err: {:?}", err);
        }   
    });    
}