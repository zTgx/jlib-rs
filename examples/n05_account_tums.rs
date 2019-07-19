extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_tums::*;
use jlib::message::query::account_tums::{RequestAccountTumsResponse, AccounTumSideKick};

fn main() {
    let config = Config::new(TEST1, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    AccountTums::new().request_account_tums(config.clone(), account, |x| match x {
        Ok(response) => {
            let res: RequestAccountTumsResponse = response;
            println!("可接收和发送的货币: \n{:?}", &res);
        },

        Err(e) => {
            let err: AccounTumSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
