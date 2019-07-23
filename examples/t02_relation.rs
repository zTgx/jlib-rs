extern crate jlib;

use jlib::misc::config::*;
use jlib::message::transaction::relation::{RelationTxResponse, RelationSideKick};
use jlib::api::transaction::relate::*;
use jlib::message::common::amount::Amount;
use jlib::RelationType::AUTHORIZE;

fn main() {
    let config = Config::new(TEST1, true);
    
    let from: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();

    let relation_type = AUTHORIZE;
    let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    let amount: Amount = Amount::new("CCA".to_string(), "0.01".to_string(), "js7M6x28mYDiZVJJtfJ84ydrv2PthY9W9u".to_string());

    Relate::with_params(config.clone(), from, secret)
            .set_relation(relation_type, to, amount,
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
