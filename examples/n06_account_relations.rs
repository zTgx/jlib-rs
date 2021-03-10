extern crate jlib;

use jlib::api::account_relations::api::request;
use jlib::api::account_relations::data::{RequestAccountRelationsResponse, RelationsSideKick};

use jlib::api::config::Config;
static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let rtype = Some("trust".to_string());

    request(config, account, rtype, |x| match x {
        Ok(response) => {
            let res: RequestAccountRelationsResponse = response;
            println!("account relations: {:?}", &res);
        },
        Err(e) => {
            let err: RelationsSideKick= e;
            println!("err: {:?}", err);
        }
    });   
}
