extern crate jlib;

use jlib::api::nth_tx::api::request;
use jlib::api::nth_tx::data::{RequestTxResponse, SpecTxSideKick};

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let tx_hash = "4552D9C58078855888A966F4FEE4FA46C413211A96C3174A7980651106C4E2DA".to_string();
    request(config, tx_hash, |x| match x {
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

