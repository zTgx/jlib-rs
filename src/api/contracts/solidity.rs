///////////////////////////////////////////////////////////////////////////////////////
//
// Solidity contract APIs: deploy && invoke
//
///////////////////////////////////////////////////////////////////////////////////////

pub use crate::api::contracts::message::{
    SolidityInitMessage,
    SolidityInitResponse,

    SolidityInvokeMessage,
    SolidityInvokeResponse,

    Arg,
};
use std::rc::Rc;
use std::cell::Cell;
use ws::{connect, CloseCode};
use serde_json::Value;

use crate::base::misc::util::{downcast_to_string};
use crate::api::config::Config;

pub struct SolidityDeploy <'a> {
    pub config  : Config,
    pub account : &'a String,
    pub secret  : &'a String,
}
impl <'a> SolidityDeploy <'a> {
    pub fn with_params(config: Config, account: &'a String, secret: &'a String) -> Self {
        SolidityDeploy {
            config: config,
            account: account,
            secret: secret,
        }
    }

    pub fn deploy<F>(&self, payload: &'a String, op: F)
    where F: Fn(Result<SolidityInitResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account = Rc::new(Cell::new( String::from( self.account.as_str() )) );
        let secret = Rc::new(Cell::new( String::from( self.secret.as_str() )) );
        let payload = Rc::new(Cell::new( String::from( payload.as_str() )) );

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account.clone();
            let secret = secret.clone();
            let payload = payload.clone();

            let message = SolidityInitMessage::with_params(account.take(), secret.take(), payload.take());
            if let Ok(command) = message.to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<SolidityInitResponse, serde_json::error::Error> {
                    op(Ok(v))
                } else {
                    op(Err("deploy Error!"))
                }
        } else {
            op(Err("deploy Error!"))
        }
    }
}

pub struct SolidityCall <'a> {
    pub config  : Config,
    pub account : &'a String,
    pub secret  : &'a String,
    pub address : &'a String,
}
impl <'a> SolidityCall <'a> {
    pub fn with_params(config: Config, account: &'a String, secret: &'a String, address: &'a String) -> Self {
        SolidityCall {
            config: config,
            account: account,
            secret: secret,
            address: address,
        }
    }

    pub fn call<F>(&self, method_name: &'a String, args: Vec<Arg>, op: F)
    where F: Fn(Result<SolidityInvokeResponse, &'static str>){
        let info = Rc::new(Cell::new("".to_string()));

        let account = Rc::new(Cell::new( String::from( self.account.as_str())) );
        let secret = Rc::new(Cell::new( String::from(self.secret.as_str())) );
        let address = Rc::new(Cell::new( String::from( self.address.as_str() )));
        let method_name = Rc::new(Cell::new( String::from( method_name.as_str() )) );
        let args = Rc::new(Cell::new(args));

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account.clone();
            let secret = secret.clone();
            let address = address.clone();
            let method_name = method_name.clone();
            let args = args.clone();

            let message = SolidityInvokeMessage::with_params(account.take(), secret.take(), address.take(), method_name.take(), args.take());
            if let Ok(command) = message.to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<SolidityInvokeResponse, serde_json::error::Error> {
                    op(Ok(v))
                } else {
                    op(Err("Err..."))
                }
        } else {
            op(Err("Err..."))
        }
    }
}