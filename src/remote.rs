
extern crate futures;
extern crate tokio;
extern crate websocket;

use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use std::io::stdin;
use std::thread;
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

    pub fn connect(&self) -> bool {
        let mut runtime = tokio::runtime::current_thread::Builder::new().build().unwrap();

        let (usr_msg, stdin_ch) = mpsc::channel(0);
	    thread::spawn(|| {
            let mut input = String::new();
            let mut stdin_sink = usr_msg.wait();
            loop {
                input.clear();
                stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();

                let (close, msg) = match trimmed {
                    "/close" => (true, OwnedMessage::Close(None)),
                    "/ping" => (false, OwnedMessage::Ping(b"PING".to_vec())),
                    _ => (false, OwnedMessage::Text(trimmed.to_string())),
                };

                stdin_sink
                    .send(msg)
                    .expect("Sending message across stdin channel.");

                if close {
                    break;
                }
            }
        });

	let runner = ClientBuilder::new(self.addr)
		.unwrap()
		.add_protocol("rust-websocket")
		.async_connect_insecure()
		.and_then(|(duplex, _)| {
			let (sink, stream) = duplex.split();
			stream
				.filter_map(|message| {
					//println!("Received Message: {:?}", message);

                    //zhtian parse json
                    {
                        use serde::{Serialize, Deserialize};
                        use serde_json::{Result, Value};
                        
                        if let OwnedMessage::Text(ref txt) = message {
                            let x = txt.as_str();
                            println!("txt : {}", x);

                            #[derive(Serialize, Deserialize, Debug)]
                            struct Resp {
                                fee_base: i32,
                                fee_ref: i32, 
                                hostid: String, 
                                ledger_hash: String,
                                ledger_index: i64,
                                ledger_time: i64, 
                                load_base: i32, 
                                load_factor: i32,
                                pubkey_node: String, 
                                random: String,
                                reserve_base: i32,
                                reserve_inc: i32,
                                server_status: String,
                                validated_ledgers: String, 
                            }


                            #[derive(Serialize, Deserialize, Debug)]
                            struct Resption {
                                id: i32,
                                result: Resp,
                                status: String,
                                typeid: String,
                            }

                            if let Ok(v) = serde_json::from_str(x) as Result<Value> {
                                println!("id : {:?}", v["id"]);
                                println!("result : {:?}", v["result"]);
                                println!("status : {:?}", v["status"]);
                                println!("type : {:?}", v["type"]);

                                println!("hash : {}", v["result"]["ledger_hash"]);
                            }
                            
                        }
                        
                    }

					match message {
						OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
						OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
						_ => None,
					}
				})
				.select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
				.forward(sink)
		});
	
        runtime.block_on(runner).unwrap();

        true
    } 

    pub fn disconnect(&self) -> bool {
        true
    }

    pub fn is_connected(&self) -> bool {
        true
    }

    //get 
    pub fn requestServerInfo(&self) -> Box<ServerInfo> {
        Box::new( ServerInfo {
            ledger: String::from(""),
            public_key: String::from(""),
            state: String::from(""),
            peers: vec![0;0],
            version: String::from("Skywell.10.0.1"),
        })
    }
}