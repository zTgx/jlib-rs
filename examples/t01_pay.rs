extern crate jlib;
use jlib::message::transaction::transaction::{TransactionTxResponse, PaymentSideKick};
use jlib::api::transaction::pay::*;
use jlib::message::common::amount::Amount;

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://123.57.219.73:5040";

fn main() {
    let config = Config::new(TEST_SERVER, false);
    let amount: Amount = Amount::new(Some("SWT".to_string()), "0.5".to_string(), None);
    let from: String = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let secret:String= "shstwqJpVJbsqFA5uYJJw1YniXcDF".to_string();

    let to  : String = "jP7G6Ue5AcQ5GZ71LkMxXvf5Reg44EKrjy".to_string();
    let memo: Option<String> = None;//Some("来自国密版本的测试。".to_string());

    println!("准备发送 payment。");
    Payment::with_params(config, from, secret).payment(  to, amount, memo,
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
