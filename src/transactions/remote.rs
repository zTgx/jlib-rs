
extern crate futures;

use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

extern crate ws;
use ws::{connect, CloseCode};
use serde_json::{Value};

use crate::message::query::subscribe::*;
use crate::message::common::command_trait::CommandConversion;

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


    /*
    4.18挂单
    */

    /*
    4.19取消挂单
    */

}