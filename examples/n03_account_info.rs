extern crate jlib;
use jlib::api::query::account_info::*;
use jlib::message::query::account_info::{RequestAccountInfoResponse, AccounInfoSideKick};

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://123.57.219.73:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "jP7G6Ue5AcQ5GZ71LkMxXvf5Reg44EKrjy".to_string();

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
