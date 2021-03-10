//Subscribe

use ws::{connect, Handler, Sender, Handshake, Message, CloseCode};

use serde_json::{Value};

use std::sync::Arc;
use crate::api::config::Config;

use crate::api::subscription::data::{
    SubscribeResponse, 
    SubscribeCommand
};

pub struct Client {
    out: Sender,
    op: Arc<dyn Fn(Result<SubscribeResponse, serde_json::error::Error>)>,
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


pub fn on<F>(config: Config, op: F)
    where F: 'static + Fn(Result<SubscribeResponse, serde_json::error::Error>) + std::marker::Send {

        let op_rc = Arc::new(op);

        connect(config.addr, |out| {

            let op = op_rc.clone();

            Client {
                out: out,
                op: op,
            }

        }).unwrap();
}    
