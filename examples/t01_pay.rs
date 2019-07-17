extern crate jlib;

use jlib::misc::config::*;
use jlib::message::transaction::transaction::TransactionTxResponse;
use jlib::api::transaction::pay::*;
use jlib::message::common::amount::Amount;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);//Config::default_with_box();
    let amount: Amount = Amount::new("SWT".to_string(), "0.5".to_string(), "".to_string());
    let from: String = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    let secret:String= "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();
    // let memo: String = "".to_string();
    // let memo: String = "给jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c支付0.5swt.".to_string();
    let sequence: u32= 11u32;
    Payment::new().payment( config.clone(), 
                            from,
                            to,
                            amount,
                            None,
                            Some(sequence),
                            Some(secret),

                                |x| match x {
        Ok(response) => {
            let res: TransactionTxResponse = response;
            println!("支付信息: {:?}", &res);
        },

        Err(_) => {
            panic!("Error Message.");
        }       
    });
}