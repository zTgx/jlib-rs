extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::relations::*;
use jlib::message::query::relations::{RequestAccountRelationsResponse, RelationsSideKick};

fn main() {
    let config = Config::new(TEST_SERVER, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let rtype = Some("trust".to_string());
    Relations::new().request_account_relations(config.clone(), account, rtype, |x| match x {
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
