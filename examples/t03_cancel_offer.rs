extern crate jlib;

use jlib::api::cancel_offer::api::CancelOffer;
use jlib::api::cancel_offer::data::{OfferCancelTxResponse, OfferCancelSideKick};

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040"; //dev12 国密服务器

fn main() {
    let config = Config::new(TEST_SERVER, true);

    let account: String = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let secret : String= "shstwqJpVJbsqFA5uYJJw1YniXcDF".to_string();
    let offer_sequence: u64 = 1_u64;

    CancelOffer::with_params(config, account, secret).cancel_offer( offer_sequence,
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
