extern crate jlib;

use jlib::api::set_relation::data::{RelationType, RelationTxResponse, RelationSideKick};
use jlib::api::set_relation::api::Relation;

use jlib::message::common::amount::Amount;

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040"; //dev12 国密服务器

fn main() {
    let config = Config::new(TEST_SERVER, true);
    
    let from: String = "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string();
    let secret:String= "shstwqJpVJbsqFA5uYJJw1YniXcDF".to_string();

    let relation_type = RelationType::AUTHORIZE;
    let to  : String = "jP7G6Ue5AcQ5GZ71LkMxXvf5Reg44EKrjy".to_string();
    let amount: Amount = Amount::new(Some( "CCA".to_string() ), "0.01".to_string(), Some( "j9syYwWgtmjchcbqhVB18pmFqXUYahZvvg".to_string()) );

    Relation::with_params(config, from, secret)
            .set_relation(relation_type, to, amount,
                                         |x| match x {
        Ok(response) => {
            let res: RelationTxResponse = response;
            println!("relations: {:?}", &res);
        },
        Err(e) => {
            let err: RelationSideKick = e;
            println!("err: {:?}", err);
        }   
    });
}
