
use std::rc::Rc;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate mylib;
// use mylib::common::*;
use mylib::remote::*;
use mylib::config::*;

fn main() {
    // Setup logging
    env_logger::init();
    info!("starting up");

    // let def: Box<dyn CommandConversion> = SubscribeCommand::with_params(0, "subscribe".to_string(), vec!["ledger".to_string(),"server".to_string(),"transactions".to_string()]);
    // let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //     Some(b) => b,
    //     None => panic!("&a isn't a B!"),
    // };
    // b.to_string();

    // let config: Box<Rc<Config>> = Config::new("ws://ts5.jingtum.com:5020", false);
    // println!("config: {:?}", config);

    let config: Box<Rc<Config>> = Config::default_with_box();
    println!("config : {:?}", config);

    // let ret = Remote::with_config(config.clone(), |x| { match x {
    //     Ok(response) => { 
    //         // if let Ok(response) = x {                        
    //         //     // use serde::{Serialize, Deserialize};
    //         //     // use serde_json::{Result, Value};
    //         //     // if let Ok(v) = serde_json::from_str(&x) as Result<Value> {
    //         //     //     println!("v : {}", v["ledger_hash"]);
    //         //     // }

    //         //     
    //         // }

    //         println!("Response fee_base : {}", response.fee_base);

    //     }
    //     Err(err) => { println!("error: {}", err); }
    // }});

    // Remote::request_server_info(config.clone(), |x| match x {
    //     Ok(response) => {
    //         println!("build_version : {}", response.build_version);
    //     }

    //     Err(_) => {
    //     }
    // });

    // Remote::request_ledger_closed(config.clone(), |x| match x {
    //     Ok(response) => {
    //         println!("ledger hash  : {}", response.ledger_hash);
    //         println!("ledger index : {}", response.ledger_index);
    //     }

    //     Err(_) => {
    //     }
    // });

    // Remote::request_ledger(config.clone(), Some(88670), None, true, |x| match x {
    //     Ok(response) => {
    //         println!("ledger : {:?}", response);
    //         println!("account_hash: {}", response.account_hash);
    //     },

    //     Err(_) => {

    //     }
    // });
    
    // Remote::request_account_info(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("Balance: {}", response.Balance);
    //     },

    //     Err(_) => {

    //     }
    // });

    // Remote::request_account_tums(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("Balance: {}", response.ledger_hash);
    //     },

    //     Err(_) => {

    //     }
    // });

    // Remote::request_account_relations(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), Some("trust".to_string()), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("ledger_hash: {}", response.ledger_hash);
    //     },

    //     Err(_) => {

    //     }
    // });   

    // Remote::request_account_offer(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("ledger_hash: {}", response.ledger_hash);
    //     },

    //     Err(_) => {

    //     }   
    // });    
 
    // Remote::request_account_tx(config.clone(), "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string(), Some(1), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("ledger_hash: {}", response.ledger_index_max);
    //     },

    //     Err(_) => {

    //     }   
    // });

    // let gets = OrderBookItem::with_params("SWT".to_string(), "".to_string());
    // let pays = OrderBookItem::with_params("CNY".to_string(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string());
    // Remote::request_order_book(config.clone(), gets, pays, |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("ledger_hash: {}", response.ledger_current_index);
    //     },

    //     Err(_) => {

    //     }   
    // });

    // Remote::request_brokerage(config.clone(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string(),
    //                                            1,
    //                                            "TES".to_string(), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("ledger_hash: {}", response.ledger_hash);
    //     },

    //     Err(_) => {

    //     }   
    // });

    // Remote::request_tx(config.clone(), "084C7823C318B8921A362E39C67A6FB15ADA5BCCD0C7E9A3B13485B1EF2A4313".to_string(), |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("ledger_hash: {}", response.app_type);
    //     },

    //     Err(_) => {

    //     }   
    // });


    // let amount: Amount = Amount::new("SWT".to_string(), 0.5, "".to_string());
    // let from: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    // let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    // let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    // // let memo: String = "".to_string();
    // let memo: String = "给jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c支付0.5swt.".to_string();
    // Remote::build_payment_tx(config.clone(), 
    //                                     from,
    //                                     to,
    //                                     amount,
    //                                     Some(memo),
    //                                     Some(secret),

    //                                      |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("tx_blob: {}", response.tx_blob);
    //     },

    //     Err(_) => {

    //     }   
    // });

    // let amount: Amount = Amount::new("SWT".to_string(), 0.5, "".to_string());
    // let from: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    // let to  : String = "jDUjqoDZLhzx4DCf6pvSivjkjgtRESY62c".to_string();
    // let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    // let relation_type = 1;
    // Remote::build_relation_tx(config.clone(), 
    //                                     from,
    //                                     to,
    //                                     relation_type,
    //                                     amount,
    //                                     Some(secret),

    //                                      |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("tx_blob: {}", response.tx_blob);
    //     },

    //     Err(_) => {

    //     }   
    // });

    
    // let taker_gets: AmountTest = AmountTest::new("CNY".to_string(), "0.01".to_string(), "jBciDE8Q3uJjf111VeiUNM775AMKHEbBLS".to_string());
    // let taker_pays: AmountTest = AmountTest::new("SWT".to_string(), "1".to_string(), "".to_string());
    // // let typ0 = "Sell".to_string();
    // let account: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    // // let app  : Option<u64> = None;
    // let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    // Remote::build_offer_create_tx(config.clone(), 
    //                                     account,
    //                                     taker_gets,
    //                                     taker_pays,
    //                                     Some(secret),

    //                                      |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("tx_blob: {}", response.tx_blob);
    //     },

    //     Err(_) => {

    //     }   
    // });

    
    // let offer_sequence: u64 = 688_u64;
    // let account: String = "jB7rxgh43ncbTX4WeMoeadiGMfmfqY2xLZ".to_string();
    // let secret:String= "sn37nYrQ6KPJvTFmaBYokS3FjXUWd".to_string();
    // Remote::build_offer_cancel_tx(config.clone(), 
    //                                     account,
    //                                     offer_sequence,
    //                                     Some(secret),

    //                                      |x| match x {
    //     Ok(response) => {
    //         //println!("ledger : {:?}", response);
    //         println!("tx_blob: {}", response.tx_blob);
    //     },

    //     Err(_) => {

    //     }   
    // });
}
