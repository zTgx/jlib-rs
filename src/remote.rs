
extern crate futures;
extern crate websocket;

use std::io::stdin;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};

use crate::server_info::ServerInfo;

pub struct Remote {
    addr: &'static str,
    local_sign: bool,
}

impl Remote {
    pub fn new(addr: &'static str, local_sign: bool) -> Self {
        Remote {
            addr: addr,
            local_sign: local_sign,
        }
    }

    pub fn connect(&self) {
        extern crate ws;

        use ws::{connect, CloseCode};

        connect(self.addr, |out| {

            use serde_json::json;
            let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
            let compact = format!("{}", json);
            println!("input : {}", compact);
            out.send(compact).unwrap();
            move |msg| {

                println!("get : {}", msg);
                let json = json!({ "id": "1", "command": "server_info" });
                let compact = format!("{}", json);
                println!("compact : {}", compact);
                out.send(compact).unwrap();

                // move |x| {
                //     println!("x : {}", x);
                    
                //     out.close(CloseCode::Normal)
                // };

                out.close(CloseCode::Normal)
                
            }
        }).unwrap()
    } 

    pub fn disconnect(&self) -> bool {
        true
    }

    pub fn is_connected(&self) -> bool {
        true
    }

    //get 
    pub fn request_server_info(&self) -> Box<ServerInfo> {

        Box::new( ServerInfo {
                                    ledger: String::from(""),
                                    public_key: String::from(""),
                                    state: String::from("dddddd"),
                                    peers: vec![0;0],
                                    version: String::from("Skywell.10.0.1"),
                                })
        
    }
}