extern crate jlib;
use jlib::api::payment::data::{TransactionTxResponse, PaymentSideKick};
use jlib::api::payment::api::request;

use jlib::message::common::amount::Amount;

use jlib::api::config::Config;

pub static TEST_SERVER: &'static str = "ws://123.57.219.73:5040"; //dev11 国密服务器
// pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040"; //dev12 国密服务器
// static TEST_SERVER: &'static str = "ws://59.175.148.101:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);

    let amount: Amount = Amount::new(Some("SWT".to_string()), "333".to_string(), Some("j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string()));
    // let amount: Amount = Amount::new(Some("USD".to_string()), "77777".to_string(), Some("j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string()));
    let from: String = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let secret:String= "shstwqJpVJbsqFA5uYJJw1YniXcDF".to_string();

    let to  : String = "jP7G6Ue5AcQ5GZ71LkMxXvf5Reg44EKrjy".to_string();
    let memo: Option<String> = None;//Some("来自国密版本的支付测试。".to_string());

    request(config, from, secret, to, amount, memo,
         |x| match x {
            Ok(response) => {
                let res: TransactionTxResponse = response;
                println!("payment info: {:?}", &res);
            },
            Err(e) => {
                let err: PaymentSideKick = e;
                println!("err: {:?}", err);
            }
    });
}