extern crate jlib;
use jlib::message::transaction::set_brokerage::{SetBrokerageTxResponse, SetBrokerageSideKick};
use jlib::api::transaction::set_brokerage::*;
use jlib::message::common::amount::Amount;

use jlib::misc::config::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);

    let account: String     = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let secret:String       = "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();
    let fee_account: String = "jzTx4CRUZJT1ZsBhGHi7Wqikada63xRVv".to_string();

    let den = 1u64;
    let num = 1000u64;
    let amount: Amount = Amount::new(Some("TES".to_string()), "3".to_string(), Some("jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string()));

    BrokerageManage::with_params(config.clone(), account, secret, fee_account).set_rate(  den, num, amount,
         |x| match x {
            Ok(response) => {
                let res: SetBrokerageTxResponse = response;
                println!("set brokerage: {:?}", &res);
            },
            Err(e) => {
                let err: SetBrokerageSideKick = e;
                println!("err: {:?}", err);
            }
    });
}
