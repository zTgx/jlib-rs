extern crate jlib;

use jlib::misc::config::*;
use std::rc::Rc;
use jlib::api::transaction::cancel_offer::*;
use jlib::message::transaction::offer_cancel::{OfferCancelTxResponse};

fn main() {
    let config: Box<Rc<Config>> = Config::default_with_box();

    let offer_sequence: u64 = 688_u64;
    let account: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();

    CancelOffer::new().cancel_offer(    config.clone(), 
                                        account,
                                        offer_sequence,
                                        Some(secret),

                                        |x| match x {
        Ok(response) => {
            let res: OfferCancelTxResponse = response;
            println!("取消挂单: {:?}", &res);
        },

        Err(_) => {
            panic!("Error Message.");
        }   
    });
}