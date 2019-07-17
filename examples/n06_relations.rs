extern crate jlib;

use jlib::misc::config::*;
use jlib::api::relations::*;
use jlib::commands::relations::{RequestAccountRelationsResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    let account = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    Relations::new().request_account_relations(config.clone(), account, Some("trust".to_string()), |x| match x {
        Ok(response) => {
            let res: RequestAccountRelationsResponse = response;
            println!("账号关系: {:?}", &res);
        },

        Err(_) => {
            panic!("Error Message.");
        }
    });   
}