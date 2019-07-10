
// use std::rc::Rc;

// #[macro_use]
// extern crate log;
// extern crate env_logger;

// extern crate mylib;
// use mylib::config::*;

fn main() {
    println!("Hello world. ruster.");
    // Setup logging
    // env_logger::init();
    // info!("starting up");

    // let def: Box<dyn CommandConversion> = SubscribeCommand::with_params(0, "subscribe".to_string(), vec!["ledger".to_string(),"server".to_string(),"transactions".to_string()]);
    // let b: &SubscribeCommand = match def.box_to_raw().downcast_ref::<SubscribeCommand>() {
    //     Some(b) => b,
    //     None => panic!("&a isn't a B!"),
    // };
    // b.to_string();

    // let config: Box<Rc<Config>> = Config::new("ws://ts5.jingtum.com:5020", false);
    // println!("config: {:?}", config);

    // let config: Box<Rc<Config>> = Config::default_with_box();
    // println!("config : {:?}", config);

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



    

}
