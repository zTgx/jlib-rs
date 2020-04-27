extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::ledger_closed::*;
use jlib::message::query::ledger_closed::{LedgerClosedResponse, LedgerClosedSideKick};

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let _c = LedgerClosed::new().request_ledger_closed(config.clone(), |x| match x {
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
