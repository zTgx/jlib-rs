
extern crate futures;
extern crate websocket;

use std::io::stdin;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};

use crate::server_info::ServerInfo;
use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

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

pub struct Config {
    addr: str,
}
impl Config {
    pub fn get_url() -> &'static str {
        "ws://ts5.jingtum.com:5020"
    }
}


#[derive(Clone)]
pub struct Dum {
    pub text: String,
}
impl Dum {
    pub fn new(s: String) -> Self {
        Dum { text: s, }
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

    pub fn connect<F>(op: F) 
        where F: Fn(Result<ws::Message, &'static str>) {
        extern crate ws;
        use ws::{connect, CloseCode};

        connect(Config::get_url(), |out| {
            
            let dumx = Rc::new(out);
            let out = dumx.clone();
            //self.f = None;


            use serde_json::json;
            let json = json!({ "id": "0", "command": "subscribe" , "streams" : ["ledger","server","transactions"]});
            let compact = format!("{}", json);
            println!("input : {}", compact);
            out.send(compact).unwrap();

            move |msg| {
                //println!("get : {}", msg);
                out.close(CloseCode::Normal)
                //Ok(())
            }
        }).unwrap()
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
            Err(_) => { "".to_string() }
        }
    }

    pub fn request_server_info<F> (op: F) -> Box<ServerInfo> 
        where F: Fn(Result<String, &'static str>) {

        extern crate ws;
        use ws::{connect, CloseCode};
        
        let info = Rc::new(Cell::new("".to_string()));


        connect(Config::get_url(), |out| { 
        let copy = info.clone();

        use serde_json::json;
        let json = json!({ "id": "1", "command": "server_info" });
        let compact = format!("{}", json);
        println!("compact : {}", compact);

        out.send(compact).unwrap();

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