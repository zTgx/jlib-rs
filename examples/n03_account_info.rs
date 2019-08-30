extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_info::*;
use jlib::message::query::account_info::{RequestAccountInfoResponse, AccounInfoSideKick};

use std::rc::Rc;
use std::cell::Cell;
use jlib::base::misc::util::{downcast_to_usize};


fn main() {
    let config = Config::new(TEST3, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();

    let seq_rc = Rc::new(Cell::new(0u64));
    AccountInfo::new().request_account_info(config.clone(), account, |x| match x {
        Ok(response) => {
            let res: RequestAccountInfoResponse = response;
            println!("账号信息: \n{:?}", &res);

            let seq = seq_rc.clone();
            seq.set(res.sequence);
        },

        Err(e) => {
            let err: AccounInfoSideKick = e;
            println!("{:?}", err);
        }
    });

    let x = downcast_to_usize(seq_rc);
    println!("x : {}", x);
}
