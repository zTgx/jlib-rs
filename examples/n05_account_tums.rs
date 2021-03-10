extern crate jlib;

use jlib::api::account_tums::api::request;
use jlib::api::account_tums::data::{RequestAccountTumsResponse, AccounTumSideKick};

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();

    request(config, account, |x| match x {
        Ok(response) => {
            let res: RequestAccountTumsResponse = response;
            println!("account tums: \n{:?}", &res);
        },
        Err(e) => {
            let err: AccounTumSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
