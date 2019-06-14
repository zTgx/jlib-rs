extern crate mylib;
use mylib::remote::*;
use mylib::config::*;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::default_with_box();
    println!("config : {:?}", config);

    let offer_sequence: u64 = 688_u64;
    let account: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    Remote::build_offer_cancel_tx(config.clone(), 
                                        account,
                                        offer_sequence,
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