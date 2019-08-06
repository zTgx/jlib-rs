extern crate jlib;

use jlib::misc::config::*;
use jlib::message::common::amount::Amount;
use jlib::api::transaction::create_offer::*;
use jlib::message::transaction::offer_create::{OfferCreateTxResponse, OfferCreateSideKick};
use jlib::OfferType;

fn main() {
    let config = Config::new(TEST2, false);

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
            println!("res: {:?}", &res);
            let fee = res.tx_json.fee.parse::<f32>().unwrap() / 1000000f32;
            println!("[交易费: {}]", fee);
        },

        Err(e) => {
            let err: OfferCreateSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
