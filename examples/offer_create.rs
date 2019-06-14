extern crate mylib;
use mylib::remote::*;
use mylib::config::*;
use mylib::common::*;

use std::rc::Rc;

fn main() {

    let config: Box<Rc<Config>> = Config::default_with_box();
    println!("config : {:?}", config);

    let taker_gets: AmountTest = AmountTest::new("CNY".to_string(), "0.01".to_string(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string());
    let taker_pays: AmountTest = AmountTest::new("SWT".to_string(), "1".to_string(), "".to_string());
    // let typ0 = "Sell".to_string();
    let account: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    // let app  : Option<u64> = None;
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    Remote::build_offer_create_tx(config.clone(), 
                                        account,
                                        taker_gets,
                                        taker_pays,
                                        Some(secret),

                                         |x| match x {
        Ok(response) => {
            //println!("ledger : {:?}", response);
            println!("tx_blob: {}", response.tx_blob);
        },

        Err(_) => {

        }   
    });
}