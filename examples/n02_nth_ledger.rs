extern crate jlib;
use jlib::api::nth_ledger::api::request;
use jlib::api::nth_ledger::data::{RequestLedgerResponse, SpecLedgerSideKick};

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let ledger_index = 1281;
    let ledger_hash = None;
    let return_prev_tx_list = false;

    request(config, Some(ledger_index), ledger_hash, return_prev_tx_list, |x| match x {
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
