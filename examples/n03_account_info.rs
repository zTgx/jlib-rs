extern crate jlib;
use jlib::api::query::account_info::*;
use jlib::message::query::account_info::{RequestAccountInfoResponse, AccounInfoSideKick};

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();

    AccountInfo::new().request_account_info(&config, account, |x| match x {
        Ok(response) => {
            let res: RequestAccountInfoResponse = response;
            println!("account info: \n{:?}", &res);
        },
        Err(e) => {
            let err: AccounInfoSideKick = e;
            println!("{:?}", err);
        }
    });
}
