extern crate jlib;
use jlib::api::transaction::cancel_offer::*;
use jlib::message::transaction::offer_cancel::{OfferCancelTxResponse, OfferCancelSideKick};

use jlib::misc::config::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);

    let account: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let secret : String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    let offer_sequence: u64 = 688_u64;

    CancelOffer::with_params(config.clone(), account, secret).cancel_offer( offer_sequence,
                                        |x| match x {
        Ok(response) => {
            let res: OfferCancelTxResponse = response;
            println!("cancel offer: {:?}", &res);
        },
        Err(e) => {
            let err:  OfferCancelSideKick = e;
            println!("err: {:?}", err);
        }   
    });
}
