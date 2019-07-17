//
// 挂单
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::transaction::offer_create::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;
use crate::message::common::amount::Amount;

pub trait CreateOfferI {
    fn build_offer_create_tx<F>(&self, config: Box<Rc<Config>>, account: String, taker_gets: Amount, taker_pays: Amount, 
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<OfferCreateTxResponse, serde_json::error::Error>);
}

pub struct CreateOffer {}
impl CreateOffer {
    pub fn new() -> Self {
        CreateOffer {
        }
    }
}

impl CreateOfferI for CreateOffer { 
    fn build_offer_create_tx<F>(&self, config: Box<Rc<Config>>, account: String, taker_gets: Amount, taker_pays: Amount, 
                                                    secret: Option<String>, 
                                                    op: F) 
    where F: Fn(Result<OfferCreateTxResponse, serde_json::error::Error>) {

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
        
        let resp = downcast_to_string(info);
        println!("resP: {:?}", resp);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            if let Ok(v) = serde_json::from_str(&x) as Result<OfferCreateTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }
}