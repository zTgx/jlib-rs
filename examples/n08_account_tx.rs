extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_tx::*;

//Ok && Err
use jlib::message::query::account_tx::{RequestAccountTxResponse, AccounTxSideKick};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    let ok_account = "jB8rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let _err_account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    AccountTx::new().request_account_tx(config.clone(), ok_account, Some(1), |x| match x {
        Ok(response) => {
            let res: RequestAccountTxResponse = response;
            println!("账号交易列表: \n{:?}", &res);
        },

        Err(e) => {
            let err: AccounTxSideKick = e;
            println!("交易 Error : \n{:?}", err);
        }   
    });
}
