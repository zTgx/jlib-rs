extern crate jlib;

use jlib::api::set_fee_rate::data::{FeeRateResponse, SetBrokerageSideKick};
use jlib::api::set_fee_rate::api::FeeRate;

use jlib::message::common::amount::Amount;

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040"; //dev12 国密服务器

fn main() {
    let config = Config::new(TEST_SERVER, true);

    let account: String     = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let secret:String       = "shstwqJpVJbsqFA5uYJJw1YniXcDF".to_string();
    let fee_account: String = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();

    let den = 1u64;
    let num = 1000u64;
    let amount: Amount = Amount::new(Some("TES".to_string()), "3".to_string(), Some("jP7G6Ue5AcQ5GZ71LkMxXvf5Reg44EKrjy".to_string()));

    FeeRate::with_params(config, account, secret, fee_account).set_rate(  den, num, amount,
         |x| match x {
            Ok(response) => {
                let res: FeeRateResponse = response;
                println!("set brokerage: {:?}", &res);
            },
            Err(e) => {
                let err: SetBrokerageSideKick = e;
                println!("err: {:?}", err);
            }
    });
}
