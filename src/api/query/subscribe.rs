//
// 订阅信息
//

mod Sub {

use ws::{connect, Handler, Sender, Handshake, Message, CloseCode};

use std::rc::Rc;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::subscribe::*;
use crate::message::common::command_trait::CommandConversion;

pub struct Client {
    out: Sender,
    op: Rc<dyn Fn(Result<SubscribeResponse, serde_json::error::Error>)>,
}
impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        if let Ok(command) = SubscribeCommand::default().to_string() {
            self.out.send(command).unwrap();
        }

        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        // Close the connection when we get a response from the server
        let resp = msg.into_text().unwrap();
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let result: String = x["result"].to_string();
            if let Ok(x) = serde_json::from_str(&result) as Result<SubscribeResponse, serde_json::error::Error> {
                //to call the function stored in `op`, surround the field access with parentheses
                (self.op)(Ok(x))
            } else if let Ok(x) = serde_json::from_str(&resp) as Result<SubscribeResponse, serde_json::error::Error> {
                (self.op)(Ok(x))
            }
        }

        // self.out.close(CloseCode::Normal)
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

    pub trait SubscribeI {
        fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
        where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>);
    }

    pub struct Subscribe {}
    impl Subscribe {
        pub fn new() -> Self {
            Subscribe {
            }
        }
    }

    impl SubscribeI for Subscribe {
        fn with_config<F>(&self, config: Box<Rc<Config>>, op: F)
        where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>) {

            let op_rc = Rc::new(op);

            connect(config.addr, |out| {

                let op = op_rc.clone();

                Client {
                    out: out,
                    op: op,
                }

            }).unwrap();
        }
    }
}

lazy_static! {
    pub static ref SUBSCRIBE: Sub::Subscribe = {
        Sub::Subscribe::new()
    };
}
