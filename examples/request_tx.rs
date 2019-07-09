extern crate mylib;
use mylib::remote::*;
use mylib::config::*;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::default_with_box();

    Remote::request_tx(config.clone(), "4552D9C58078855888A966F4FEE4FA46C413211A96C3174A7980651106C4E2DA".to_string(), |x| match x {
        Ok(response) => {
            let fee = response.fee.parse::<f32>().unwrap() / 1000000f32;
            println!("[交易费: {}]", fee);
        },

        Err(_) => {

        }   
    });
}