
mod remote;
mod config;
mod server_info;
use remote::Remote;
use config::Config;
use std::rc::Rc;

extern crate ws;
use ws::{connect, CloseCode, Message};

fn main() {

    let config: Box<Rc<Config>> = Config::new("ws://ts5.jingtum.com:5020", false);
    println!("config: {:?}", config);

    let ret = Remote::with_config(config.clone(), |x| { match x {
        Ok(x) => { 
            // if let Ok(x) = x.into_text() {                        
            //     use serde::{Serialize, Deserialize};
            //     use serde_json::{Result, Value};
            //     if let Ok(v) = serde_json::from_str(&x) as Result<Value> {
            //         println!("v : {}", v["ledger_hash"]);
            //     }
            // }
        }
        Err(err) => { println!("error: {}", err); }
    }});

    Remote::request_server_info(config.clone(), |x| match x {
        Ok(x) => {
            println!("request info : {}", x);
        }

        Err(_) => {
        }
    });
}
