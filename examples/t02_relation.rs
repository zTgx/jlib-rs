extern crate jlib;

use jlib::misc::config::*;
use jlib::message::transaction::relation::{RelationTxResponse, RelationSideKick};
use jlib::api::transaction::relate::*;
use jlib::message::common::amount::Amount;

fn main() {
    let config = Config::new(TEST1, true);
    
    let amount: Amount = Amount::new("SWT".to_string(), "0.5".to_string(), "".to_string());
    let from: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    let relation_type = 1;
    Relate::new().set_relation( config.clone(), 
                                from,
                                to,
                                relation_type,
                                amount,
                                Some(secret),

                                    |x| match x {
        Ok(response) => {
            let res: RelationTxResponse = response;
            println!("关系设置: {:?}", &res);
        },

        Err(e) => {
            let err: RelationSideKick = e;
            println!("err: {:?}", err);
        }   
    });
}