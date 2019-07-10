extern crate mylib;
use mylib::remote::*;
use mylib::config::*;
// use mylib::common::*;
use mylib::message::Amount;

use std::rc::Rc;

fn main() {

    let config: Box<Rc<Config>> = Config::default_with_box();
    println!("config : {:?}", config);

    //Sell
    let taker_gets: Amount = Amount::new("CNY".to_string(), "0.01".to_string(), "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string());
    let taker_pays: Amount = Amount::new("SWT".to_string(), "1".to_string(), "".to_string());

    //Buy
    // let taker_gets: Amount = Amount::new("SWT".to_string(), "1".to_string(), "".to_string());
    // let taker_pays: Amount = Amount::new("CNY".to_string(), "0.01".to_string(), "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string());

    let account: String = "jn9XgdNptm9DhjZN2qLtxjTfVuDt6rQwLh".to_string();
    let secret:String= "sh6Wgh5XchPNVCcRkLyE291ecT5Gi".to_string();
    Remote::build_offer_create_tx(config.clone(), 
                                        account,
                                        taker_gets,
                                        taker_pays,
                                        Some(secret),

                                         |x| match x {
        Ok(response) => {
            let fee = response.tx_json.fee.parse::<f32>().unwrap() / 1000000f32;
            println!("[交易费: {}]", fee);
        },

        Err(_) => {

        }   
    });
}