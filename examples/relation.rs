extern crate mylib;
use mylib::remote::*;
use mylib::config::*;
use mylib::common::*;

use std::rc::Rc;

fn main() {

    let config: Box<Rc<Config>> = Config::default_with_box();
    println!("config : {:?}", config);
        let amount: Amount = Amount::new("SWT".to_string(), 0.5, "".to_string());
    let from: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    let relation_type = 1;
    Remote::build_relation_tx(config.clone(), 
                                        from,
                                        to,
                                        relation_type,
                                        amount,
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