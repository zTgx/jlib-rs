extern crate jlib;
use jlib::api::query::ledger_closed::*;
use jlib::message::query::ledger_closed::{LedgerClosedResponse, LedgerClosedSideKick};

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let _c = LedgerClosed::new().request_ledger_closed(config, |x| match x {
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
