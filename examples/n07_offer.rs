extern crate jlib;
use jlib::api::query::account_offer::*;
use jlib::message::query::offer::{RequestAccountOfferResponse, AccountOffersSideKick};

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    AccountOffer::new().request_account_offer(config, account, |x| match x {
        Ok(response) => {
            let res: RequestAccountOfferResponse = response;
            println!("offer response: {:?}", &res);
        },
        Err(e) => {
            let err: AccountOffersSideKick = e;
            println!("err: {:?}", err);
        }   
    });    
}
