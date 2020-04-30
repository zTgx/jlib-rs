extern crate jlib;
use jlib::api::query::spec_ledger::*;
use jlib::message::query::spec_ledger::{RequestLedgerResponse, SpecLedgerSideKick};

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let ledger_index = 15812149;
    let ledger_hash = None;
    let return_prev_tx_list = false;
    SpecLedger::new().request_ledger(config, Some(ledger_index), ledger_hash, return_prev_tx_list, |x| match x {
        Ok(response) => {
            let res: RequestLedgerResponse = response;
            println!("ledger info: \n{:?}", &res);
        },

        Err(e) => {
            let err: SpecLedgerSideKick = e;
            println!("err: {:?}", err);
        }
    });
}
