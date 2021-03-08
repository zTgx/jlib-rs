extern crate jlib;
use jlib::api::query::account_tx::*;

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

//Ok && Err
use jlib::message::query::account_tx::{RequestAccountTxResponse, AccounTxSideKick};

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    AccountTx::new().request_account_tx(config, account, Some(1), |x| match x {
        Ok(response) => {
            let res: RequestAccountTxResponse = response;
            println!("account tx: \n{:?}", &res);
        },
        Err(e) => {
            let err: AccounTxSideKick = e;
            println!("Error : \n{:?}", err);
        }   
    });
}
