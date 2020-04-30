extern crate jlib;
use jlib::message::transaction::transaction::{TransactionTxResponse, PaymentSideKick};
use jlib::api::transaction::pay::*;
use jlib::message::common::amount::Amount;

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let amount: Amount = Amount::new(Some("SWT".to_string()), "0.5".to_string(), None);
    let from: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    let memo: Option<String> = Some("TTTTTTTTTTTTTTTTTTTTTis memo".to_string());

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
