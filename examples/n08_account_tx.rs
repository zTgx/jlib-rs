extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_tx::*;
use jlib::message::query::account_tx::{RequestAccountTxResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    AccountTx::new().request_account_tx(config.clone(), account, Some(1), |x| match x {
        Ok(response) => {
            let res: RequestAccountTxResponse = response;
            println!("账号交易列表: \n{:?}", &res);
        },

        Err(_) => {

        }   
    });
}