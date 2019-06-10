
extern crate futures;

use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

extern crate ws;
use ws::{connect, CloseCode};
use serde_json::json;
use serde_json::{Value};

use crate::Config;

use crate::commands::command_trait::*;
use crate::commands::command_subscribe::*;
use crate::commands::command_serverinfo::*;
use crate::commands::command_ledger_closed::*;
use crate::commands::command_request_ledger::*;
use crate::commands::command_request_accountinfo::*;
use crate::commands::command_request_accounttums::*;
use crate::commands::command_request_account_relations::*;

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

//TODO::Remote 中的请求接口，要单独处理封装～～～（待实现）
impl Remote  {
    pub fn new(addr: &'static str, local_sign: bool) -> Self {
        Remote {
            addr: addr,
            local_sign: local_sign,
            conn: None,
        }
    }

    pub fn with_config<F>(config: Box<Rc<Config>>, op: F) 
        where F: Fn(Result<SubscribeResponse, &'static str>) {

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

        //parse 
        let resp = Remote::print_if(ws_message);
        {
            //TOD::解析的类可以抽象出来～～～
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                //println!("x : {:?}", x["result"]);
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<SubscribeResponse, serde_json::error::Error> {
                    println!("subscribe response : {:?}", x);

                    op(Ok(x))
                }
            }
        }

        //op(Ok(ws::Message::text(resp)))
    } 
    
    pub fn print_if(value: Rc<dyn Any>) -> String {
        match value.downcast::<Cell<String>>() {
            Ok(string) => {
                return string.take();
            },
            Err(_) => { "None".to_string() }
        }
    }
    
    pub fn request_server_info<F> (config: Box<Rc<Config>>, op: F)
        where F: Fn(Result<ServerInfoResponse, &'static str>) {
        
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
        
        let resp = Remote::print_if(info);
        println!("resp : {}", &resp);
        {
            //TOD::解析的类可以抽象出来～～～
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                    let x: String = x["info"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<ServerInfoResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }
        }


    }

    pub fn request_ledger_closed<F>(config: Box<Rc<Config>>, op: F)
        where F: Fn(Result<LedgerClosedResponse, &'static str>) {
            let info = Rc::new(Cell::new("".to_string()));

            connect(config.addr, |out| { 
                let copy = info.clone();

                if let Ok(command) = LedgerClosedCommand::default().to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<LedgerClosedResponse, serde_json::error::Error> {
                    op(Ok(x));
                }
            }

    }

    pub fn request_ledger<F>(config: Box<Rc<Config>>, ledger_index: Option<u64>, ledger_hash: Option<String>, transactions: bool, op: F) 
        where F: Fn(Result<RequestLedgerResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let ledger_index_rc = Rc::new(Cell::new(None));
            if ledger_index.is_some() {
                ledger_index_rc.set(ledger_index);
            }
            let ledger_hash_rc = Rc::new(Cell::new(None));
            if ledger_hash.is_some() {
                ledger_hash_rc.set(ledger_hash);
            }
            let transactions_rc = Rc::new(Cell::new(transactions));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let index = ledger_index_rc.clone();
                let hash = ledger_hash_rc.clone();
                let trans = transactions_rc.clone();
                if let Ok(command) = RequestLedgerCommand::with_params(index.take(), hash.take(), trans.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                    let x: String = x["ledger"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestLedgerResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }         
    }

    //此方法调试connect
    pub fn request_account_info<F>(config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountInfoResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                if let Ok(command) = RequestAccountInfoCommand::with_params(account.take()).to_string() {
                    out.send(command).unwrap();
                }

                println!("zhtian@remote.connect.");
                move |msg: ws::Message| {
                    println!("zhtian@msg");

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            //println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                    let x: String = x["account_data"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountInfoResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }         
    }

    pub fn request_account_tums<F>(config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountTumsResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                if let Ok(command) = RequestAccountTumsCommand::with_params(account.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountTumsResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    //[[[接口@4.8~4.11 参数一直，考虑后期合并。]]]!!!
    pub fn request_account_relations<F>(config: Box<Rc<Config>>, account: String, relation_type: Option<String>, op: F) 
        where F: Fn(Result<RequestAccountRelationsResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));
            let relation_type_rc = Rc::new(Cell::new(relation_type));
            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                let relation_type = relation_type_rc.clone();
                if let Ok(command) = RequestAccountRelationsCommand::with_params(account.take(), relation_type.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountRelationsResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }
}