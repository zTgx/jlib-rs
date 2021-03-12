use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::api::config::Config;
use crate::api::cancel_offer::data::*;

use crate::api::message::local_sign_tx::{LocalSignTx};
use crate::base::misc::util::{downcast_to_string};
use crate::base::local_sign::sign_tx::{SignTx};
use crate::api::utils::cast::get_account_sequence;

pub struct CancelOffer {
    pub config : Config,
    pub account: String,
    pub secret : String,
}
impl CancelOffer {
        pub fn with_params(config: Config, account: String, secret: String) -> Self {
        CancelOffer {
            config  : config,
            account : account,
            secret  : secret,
        }
    }
    
    pub fn cancel_offer<F>(&self, offer_sequence: u64, op: F)
    where F: Fn(Result<OfferCancelTxResponse, OfferCancelSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(String::from(self.account.as_str())));
        let secret_rc  = Rc::new(Cell::new(String::from(self.secret.as_str())));

        let offer_sequence_rc = Rc::new(Cell::new(offer_sequence));

        // Get Account Seq
        let account_seq = get_account_sequence(&self.config, self.account.clone());

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account        = account_rc.clone();
            let secret         = secret_rc.clone();
            let offer_sequence = offer_sequence_rc.clone();

            let tx_json = OfferCancelTxJson::new(account.take(),  offer_sequence.take());
            if self.config.local_sign {
                let blob = SignTx::with_params(account_seq, &secret.take()).cancel_offer(&tx_json);
                if let Ok(command) = LocalSignTx::new(blob).to_string() {
                    out.send(command).unwrap()
                }
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
            if let Some(status) = x["status"].as_str() {
                if status == "success" {
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
}
