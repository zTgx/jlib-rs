use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::api::config::Config;
use crate::api::set_relation::data::*;

use crate::message::common::amount::Amount;
use crate::api::local_sign_tx::{LocalSignTx};
use crate::base::local_sign::sign_tx::{SignTx};
use crate::base::misc::util::{downcast_to_string};
use crate::api::util::get_account_sequence;

pub struct Relation {
    pub config : Config,
    pub account: String,
    pub secret : String,
}
impl Relation {
    pub fn with_params(config: Config, account: String, secret: String) -> Self {
        Relation {
            config : config,
            account: account,
            secret : secret,
        }
    }

    pub fn set_relation<F>(&self, rtype: RelationType, target: String, amount: Amount, op: F)
    where F: Fn(Result<RelationTxResponse, RelationSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(String::from(self.account.as_str())));
        let secret_rc  = Rc::new(Cell::new(String::from(self.secret.as_str())));

        let rtype_rc  = Rc::new(Cell::new(rtype.get()));
        let target_rc = Rc::new(Cell::new(target));

        let amount_rc = Rc::new(Cell::new(amount));

        // Get Account Seq
        let account_seq = get_account_sequence(&self.config, self.account.clone());
        
        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account = account_rc.clone();
            let secret = secret_rc.clone();

            let rtype  = rtype_rc.clone();
            let target = target_rc.clone();
            let amount = amount_rc.clone();
            
            let tx_json = RelationTxJson::new(account.take(), target.take(), rtype.take(), amount.take());
            if self.config.local_sign {
                let blob = SignTx::with_params(account_seq, &secret.take()).relate(&tx_json);
                if let Ok(command) = LocalSignTx::new(blob).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = RelationTx::new(secret.take(), tx_json).to_string() {
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
                if let Ok(v) = serde_json::from_str(&x) as Result<RelationTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<RelationSideKick, serde_json::error::Error> {
                    op(Err(v))
                }
            }
        }
    }
}
