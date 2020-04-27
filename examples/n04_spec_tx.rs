extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::spec_tx::*;
use jlib::message::query::spec_tx::{RequestTxResponse, SpecTxSideKick};

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let tx_hash = "4552D9C58078855888A966F4FEE4FA46C413211A96C3174A7980651106C4E2DA".to_string();
    SpecTx::new().request_tx(config.clone(), tx_hash, |x| match x {
        Ok(response) => {
            let res: RequestTxResponse = response;
            println!("transaction info: \n{:?}", &res);
        },

        Err(e) => {
            let err: SpecTxSideKick = e;
            println!("err: {:?}", err);
        }
    });
}

