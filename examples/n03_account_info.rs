extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_info::*;
use jlib::message::account_info::{RequestAccountInfoResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    println!("config : {:?}", config);

    AccountInfo::new().request_account_info(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), |x| match x {
        Ok(response) => {
            let res: RequestAccountInfoResponse = response;
            println!("账号信息: \n{:?}", &res);
        },

        Err(_) => {

        }
    });
}

