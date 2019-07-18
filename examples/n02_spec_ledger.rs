extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::spec_ledger::*;
use jlib::message::query::spec_ledger::{RequestLedgerResponse, SpecLedgerSideKick};

fn main() {
    let config = Config::new(TEST1, true);
    SpecLedger::new().request_ledger(config.clone(), Some(1), None, true, |x| match x {
        Ok(response) => {
            let res: RequestLedgerResponse = response;
            println!("账本具体信息: \n{:?}", &res);
        },

        Err(e) => {
            let err: SpecLedgerSideKick = e;
            println!("err: {:?}", err);
        }
    });
}