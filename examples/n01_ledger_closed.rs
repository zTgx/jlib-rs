extern crate jlib;
use jlib::api::ledger_closed::api::request;
use jlib::api::ledger_closed::data::{LedgerClosedResponse, LedgerClosedSideKick};

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    request(config, |x| match x {
        Ok(response) => {
            let res: LedgerClosedResponse = response;
            println!("----------------------------------------------------------------------------------");
            println!("last closed ledger info : ");
            println!("-- ledger hash : {}", &res.ledger_hash);
            println!("-- ledger index: {}", &res.ledger_index);
            println!("----------------------------------------------------------------------------------");
        },
        Err(e) => {
            let err: LedgerClosedSideKick = e;
            println!("{:?}", err);
        }
    });
}
