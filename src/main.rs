
mod remote;
mod config;
mod server_info;
use remote::Remote;
use config::Config;
use std::rc::Rc;

extern crate ws;
use ws::{connect, CloseCode, Message};

mod commands;
//mod jingtum_lib::commands;

fn main() {

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

    let ret = Remote::with_config(config.clone(), |x| { match x {
        Ok(response) => { 
            // if let Ok(response) = x {                        
            //     // use serde::{Serialize, Deserialize};
            //     // use serde_json::{Result, Value};
            //     // if let Ok(v) = serde_json::from_str(&x) as Result<Value> {
            //     //     println!("v : {}", v["ledger_hash"]);
            //     // }

            //     
            // }

            println!("Response fee_base : {}", response.fee_base);

        }
        Err(err) => { println!("error: {}", err); }
    }});

    // Remote::request_server_info(config.clone(), |x| match x {
    //     Ok(x) => {
    //         println!("request info : {}", x);
    //     }

    //     Err(_) => {
    //     }
    // });
}
