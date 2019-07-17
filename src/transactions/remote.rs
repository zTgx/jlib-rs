
extern crate futures;

use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

extern crate ws;
use ws::{connect, CloseCode};
use serde_json::{Value};

use crate::message::common::command_trait::*;
use crate::message::query::subscribe::*;

use crate::message::common::amount::Amount;
use crate::message::transaction::relation::*;
use crate::message::transaction::offer_create::*;
use crate::message::transaction::offer_cancel::*;

use crate::misc::config::Config;

// pub struct Conn {
//     conn: Option<Rc<ws::Sender>>,
// }

// impl Conn {
//     pub fn new(out: Option<Rc<ws::Sender>>) -> Self {
//         Conn {
//             conn: out,
//         }
//     }
//     pub fn request_server_info(&self) {
//         use serde_json::json;
//         let json = json!({ "id": "1", "command": "server_info" });
//         let compact = format!("{}", json);
//         println!("compact : {}", compact);
   
//     }
// }

pub struct Remote {
    // addr: &'static str,
    // local_sign: bool,
    // conn: Option<Conn>,
}

//TODO::Remote 中的请求接口，要单独处理封装～～～（待实现）
impl Remote  {
    // pub fn new(addr: &'static str, local_sign: bool) -> Self {
    //     Remote {
    //         // addr: addr,
    //         // local_sign: local_sign,
    //         // conn: None,
    //     }
    // }

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

    /*
    4.15支付
    */


    /*
    4.16设置关系
    */
    pub fn build_relation_tx<F>(config: Box<Rc<Config>>, account: String, target: String, relation_type: u64, amount: Amount, 
                                                     secret: Option<String>, 
                                                     op: F) 
        where F: Fn(Result<RelationTxResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let target_rc = Rc::new(Cell::new(target));
        let relation_type_rc = Rc::new(Cell::new(relation_type));
        let amount_rc = Rc::new(Cell::new(amount));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let target   = target_rc.clone();
            let relation_type = relation_type_rc.clone();
            let amount = amount_rc.clone();
            let secret = secret_rc.clone();

            if let Ok(command) = RelationTx::new(secret.take(), RelationTxJson::new(account.take(), target.take(), relation_type.take(), amount.take())).to_string() {
                out.send(command).unwrap()
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
            //println!("x : {}", x);
            if let Ok(v) = serde_json::from_str(&x) as Result<RelationTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }

    /*
    4.18挂单
    */
    pub fn build_offer_create_tx<F>(config: Box<Rc<Config>>, account: String, taker_gets: Amount, taker_pays: Amount, 
                                                     secret: Option<String>, 
                                                     op: F) 
        where F: Fn(Result<OfferCreateTxResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let taker_gets_rc = Rc::new(Cell::new(taker_gets));
        let taker_pays_rc = Rc::new(Cell::new(taker_pays));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let taker_gets = taker_gets_rc.clone();
            let taker_pays = taker_pays_rc.clone();
            let secret = secret_rc.clone();


            // let xy = OfferCreateTx::new(secret.take(), OfferCreateTxJson::new(account.take(), 
            //                                                                 taker_gets.take(), taker_pays.take()));
            // println!("js : {:?}", serde_json::to_string(&xy));
            if let Ok(command) = OfferCreateTx::new(secret.take(), OfferCreateTxJson::new(account.take(), 
                                                                            taker_gets.take(), taker_pays.take())).to_string() {
                out.send(command).unwrap()
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
            //println!("x : {}", x);
            if let Ok(v) = serde_json::from_str(&x) as Result<OfferCreateTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }

    /*
    4.19取消挂单
    */
    pub fn build_offer_cancel_tx<F>(config: Box<Rc<Config>>, account: String, offer_sequence: u64, 
                                                     secret: Option<String>, 
                                                     op: F) 
        where F: Fn(Result<OfferCancelTxResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let offer_sequence_rc = Rc::new(Cell::new(offer_sequence));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let offer_sequence = offer_sequence_rc.clone();
            let secret = secret_rc.clone();

            if let Ok(command) = OfferCancelTx::new(secret.take(), OfferCancelTxJson::new(account.take(),  offer_sequence.take())).to_string() {
                out.send(command).unwrap()
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
            //println!("x : {}", x);
            if let Ok(v) = serde_json::from_str(&x) as Result<OfferCancelTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }
}