extern crate jlib;
use jlib::api::message::amount::Amount;

use jlib::api::create_offer::api::CreateOffer;
use jlib::api::create_offer::data::{OfferType, OfferCreateTxResponse, OfferCreateSideKick};

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040"; //dev12 国密服务器

fn main() {
    let config = Config::new(TEST_SERVER, false);

    //Sell
    let offer_type = OfferType::Sell;
    let taker_gets: Amount = Amount::new(Some( "CNY".to_string() ), "0.01".to_string(), Some( "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string() ));
    let taker_pays: Amount = Amount::new(Some( "SWT".to_string() ), "1".to_string(),    None);

    //Buy
    //let taker_gets: Amount = Amount::new(Some( "SWT".to_string() ), "1".to_string(), None);
    //let taker_pays: Amount = Amount::new(Some( "CNY".to_string() ), "0.01".to_string(), Some("jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string()) );

    let account: String = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let secret : String= "shstwqJpVJbsqFA5uYJJw1YniXcDF".to_string();

    CreateOffer::with_params(config, account, secret).create_offer( offer_type, taker_gets, taker_pays,
                                                |x| match x {
        Ok(response) => {
            let res: OfferCreateTxResponse = response;
            // let fee = res.tx_json.fee.parse::<f32>().unwrap() / 1000000f32;
            println!("挂单返回数据: {:#?}", res);
        },

        Err(e) => {
            let err: OfferCreateSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
