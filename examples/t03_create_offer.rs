extern crate jlib;
use jlib::message::common::amount::Amount;
use jlib::api::transaction::create_offer::*;
use jlib::message::transaction::offer_create::{OfferType, OfferCreateTxResponse, OfferCreateSideKick};

use jlib::misc::config::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, false);

    //Sell
    let offer_type = OfferType::Sell;
    let taker_gets: Amount = Amount::new(Some( "CNY".to_string() ), "0.01".to_string(), Some( "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string() ));
    let taker_pays: Amount = Amount::new(Some( "SWT".to_string() ), "1".to_string(),    None);

    //Buy
    //let taker_gets: Amount = Amount::new(Some( "SWT".to_string() ), "1".to_string(), None);
    //let taker_pays: Amount = Amount::new(Some( "CNY".to_string() ), "0.01".to_string(), Some("jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string()) );

    let account: String = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let secret : String= "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();

    CreateOffer::with_params(config.clone(), account, secret).create_offer( offer_type, taker_gets, taker_pays,
                                                |x| match x {
        Ok(response) => {
            let res: OfferCreateTxResponse = response;
            let fee = res.tx_json.fee.parse::<f32>().unwrap() / 1000000f32;
            println!("transaction fee: {}", fee);
        },

        Err(e) => {
            let err: OfferCreateSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
