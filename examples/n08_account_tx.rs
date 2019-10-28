extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::account_tx::*;

//Ok && Err
use jlib::message::query::account_tx::{RequestAccountTxResponse, AccounTxSideKick};

fn main() {
    let config = Config::new(TEST3, true);
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    AccountTx::new().request_account_tx(config.clone(), account, Some(1), |x| match x {
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
