extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::spec_ledger::*;
use jlib::message::query::spec_ledger::{RequestLedgerResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    println!("config : {:?}", config);

    SpecLedger::new().request_ledger(config.clone(), Some(88670), None, true, |x| match x {
        Ok(response) => {
            let res: RequestLedgerResponse = response;
            println!("账本具体信息: \n{:?}", &res);
        },

        Err(_) => {
            panic!("Error Message.");
        }
    });
}