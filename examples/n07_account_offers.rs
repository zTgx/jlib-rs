extern crate jlib;
use jlib::api::account_offers::api::request;
use jlib::api::account_offers::data::{RequestAccountOfferResponse, AccountOffersSideKick};

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();

    request(config, account, |x| match x {
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
