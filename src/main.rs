
mod remote;
mod config;
mod server_info;
use remote::Remote;
use config::Config;

extern crate ws;
use ws::{connect, CloseCode, Message};

static URL: &'static str = "ws://ts5.jingtum.com:5020";
static URL2: &'static str = "ws://127.0.0.1:5060";
fn main() {
    println!("Hello, world!");

    let config: Box<Config> = Config::new(URL, false);
    println!("config: {:?}", config);

    let ret = Remote::connect(|x| { match x {
        Ok(x) => { 
            println!("ret : {}", x.is_empty()); 
            println!("len : {}", x.len());
            if let Ok(x) = x.into_text() {
                println!("text: {}", x);
                        
                use serde::{Serialize, Deserialize};
                use serde_json::{Result, Value};
                if let Ok(v) = serde_json::from_str(&x) as Result<Value> {
                    println!("v : {}", v["ledger_hash"]);
                }

                //request server info
                // ret.request_server_info();


            }
        }
        Err(err) => { println!("error: {}", err); }
    } 
    });

    Remote::request_server_info(|x| match x {
        Ok(x) => {
            println!("request info : {}", x);
        }

        Err(_) => {

        }
    });

    fn call_with_one<F>(func: F) -> usize
    where F: Fn(usize) -> usize {
    func(1)
    }

    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);

}
