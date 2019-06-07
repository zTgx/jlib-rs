
extern crate futures;

use crate::server_info::ServerInfo;
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

extern crate ws;
use ws::{connect, CloseCode};
use serde_json::json;

use crate::Config;
use crate::command::*;

pub struct Conn {
    conn: Option<Rc<ws::Sender>>,
}

impl Conn {
    pub fn new(out: Option<Rc<ws::Sender>>) -> Self {
        Conn {
            conn: out,
        }
    }
    pub fn request_server_info(&self) {
        use serde_json::json;
        let json = json!({ "id": "1", "command": "server_info" });
        let compact = format!("{}", json);
        println!("compact : {}", compact);
   
    }
}

pub struct Remote {
    addr: &'static str,
    local_sign: bool,
    conn: Option<Conn>,
}

impl Remote  {
    pub fn new(addr: &'static str, local_sign: bool) -> Self {
        Remote {
            addr: addr,
            local_sign: local_sign,
            conn: None,
        }
    }

    pub fn with_config<F>(config: Box<Rc<Config>>, op: F) 
        where F: Fn(Result<ws::Message, &'static str>) {

        let ws_message = Rc::new(Cell::new("".to_string()));

        connect(config.addr, |out| {
            
            let cloned_ws_message = ws_message.clone();

            if let Ok(command) = SubscribeCommand::default().to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let text = msg.as_text()?;
                cloned_ws_message.set(text.to_string());

                out.close(CloseCode::Normal)
            }
        }).unwrap();

        op(Ok(ws::Message::text(Remote::print_if(ws_message))))
    } 

    pub fn disconnect(&self) -> bool {
        true
    }

    pub fn is_connected(&self) -> bool {
        true
    }

    pub fn print_if(value: Rc<dyn Any>) -> String {
        match value.downcast::<Cell<String>>() {
            Ok(string) => {
                return string.take();
            },
            Err(_) => { "None".to_string() }
        }
    }
    
    pub fn request_server_info<F> (config: Box<Rc<Config>>, op: F) -> Box<ServerInfo> 
        where F: Fn(Result<String, &'static str>) {
        
        let info = Rc::new(Cell::new("".to_string()));


        connect(config.addr, |out| { 
        let copy = info.clone();

        if let Ok(command) = ServerInfoCommand::default().to_string() {
            out.send(command).unwrap();
        }

        move |msg: ws::Message| {
            let c = msg.as_text()?;
            copy.set(c.to_string());
            
            out.close(CloseCode::Normal) 
        }
        
        }).unwrap();
        
        let re = Remote::print_if(info);
        op(Ok(re));

        Box::new( ServerInfo {
                                    ledger: String::from(""),
                                    public_key: String::from(""),
                                    state: String::from("dddddd"),
                                    peers: vec![0;0],
                                    version: String::from("Skywell.10.0.1"),
                                })
        
    }
}