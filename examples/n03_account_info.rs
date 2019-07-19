extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_info::*;
use jlib::message::query::account_info::{RequestAccountInfoResponse, AccounInfoSideKick};

fn main() {
    let config = Config::new(TEST1, true);
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyth".to_string();
    AccountInfo::new().request_account_info(config.clone(), account, |x| match x {
        Ok(response) => {
            let res: RequestAccountInfoResponse = response;
            println!("账号信息: \n{:?}", &res);
        },

        Err(e) => {
            let err: AccounInfoSideKick = e;
            println!("{:?}", err);
        }
    });
}

