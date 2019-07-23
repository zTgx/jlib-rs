//
// 取消挂单
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::transaction::offer_cancel::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

use crate::base::sign_cancel_offer::*;

pub trait CancelOfferI {
    fn cancel_offer<F>(&self, offer_sequence: u64, op: F) 
    where F: Fn(Result<OfferCancelTxResponse, OfferCancelSideKick>);
}

pub struct CancelOffer {
    pub config: Box<Rc<Config>>,
    pub account: String,
    pub secret: String,
}
impl CancelOffer {
        pub fn with_params(config: Box<Rc<Config>>, account: String, secret: String) -> Self {
        CancelOffer {
            config  : config,
            account : account,
            secret  : secret,
        }
    }
}

impl CancelOfferI for CancelOffer { 
    fn cancel_offer<F>(&self, offer_sequence: u64, op: F) 
    where F: Fn(Result<OfferCancelTxResponse, OfferCancelSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(String::from(self.account.as_str())));
        let secret_rc  = Rc::new(Cell::new(String::from(self.secret.as_str())));

        let offer_sequence_rc = Rc::new(Cell::new(offer_sequence));
        
        connect(self.config.addr, |out| { 
            let copy = info.clone();

            let account        = account_rc.clone();
            let secret         = secret_rc.clone();
            let offer_sequence = offer_sequence_rc.clone();
            
            let tx_json = OfferCancelTxJson::new(account.take(),  offer_sequence.take());
            //local sign
            if self.config.local_sign {
                //Keypair对象的生成不合理，待重构!!!
                use crate::base::keypair::*;
                use crate::base::seed::*;
                let seed_property = SeedProperty::new(&secret.take(), 16);
                let seed = SeedBuilder::new(seed_property).build();

                //keypair
                let keypair = KeypairBuilder::new(&seed).build();
                let blob = SignTxCancelOffer::with_params(&keypair, &tx_json).build();
                println!("cancel offer: {}", blob);
            } else {
                if let Ok(command) = OfferCancelTx::new(secret.take(), tx_json).to_string() {
                    out.send(command).unwrap()
                }
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let status = x["status"].to_string();
            if status == "\"success\"" {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<OfferCancelTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<OfferCancelSideKick, serde_json::error::Error> {
                    op(Err(v))
                } 
            }
        }         

    }
}