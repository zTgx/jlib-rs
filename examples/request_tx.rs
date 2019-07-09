extern crate mylib;
use mylib::remote::*;
use mylib::config::*;
use mylib::common::*;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::default_with_box();

    Remote::request_tx(config.clone(), "4552D9C58078855888A966F4FEE4FA46C413211A96C3174A7980651106C4E2DA".to_string(), |x| match x {
        Ok(response) => {
            println!("transaction fee : {}", response.fee.parse::<u32>().unwrap() / 1000000);
        },

        Err(_) => {

        }   
    });
}