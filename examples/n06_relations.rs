extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::relations::*;
use jlib::message::query::relations::{RequestAccountRelationsResponse, RelationsSideKick};

fn main() {
    let config = Config::new(TEST3, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let rtype = Some("trust".to_string());
    Relations::new().request_account_relations(config.clone(), account, rtype, |x| match x {
        Ok(response) => {
            let res: RequestAccountRelationsResponse = response;
            println!("账号关系: {:?}", &res);
        },

        Err(e) => {
            let err: RelationsSideKick= e;
            println!("err: {:?}", err);
        }
    });   
}
