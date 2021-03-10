extern crate jlib;

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

//Ok && Err
use jlib::api::account_txs::api::request;
use jlib::api::account_txs::data::{RequestAccountTxResponse, AccounTxSideKick};

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();

    request(config, account, Some(1), |x| match x {
        Ok(response) => {
            let res: RequestAccountTxResponse = response;
            println!("account tx: \n{:?}", &res);
        },
        Err(e) => {
            let err: AccounTxSideKick = e;
            println!("Error : \n{:?}", err);
        }   
    });
}
