//
// 支付
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::transaction::transaction::*;
use crate::message::common::command_trait::CommandConversion;
use crate::message::common::memo::*;
use crate::message::common::amount::Amount;
use crate::api::query::account_info::*;

use cast_rs::hex_t;

use crate::message::transaction::local_sign_tx::{LocalSignTx};
use crate::base::local_sign::sign_tx::{SignTx};
use crate::base::misc::util::{
    downcast_to_usize, downcast_to_string,
    check_address, check_secret, check_amount,
};

pub trait PaymentI {
    fn payment<F>(&self, to: String, amount: Amount, memo: Option<String>, op: F)
    where F: Fn(Result<TransactionTxResponse, PaymentSideKick>);
}

pub struct Payment {
    pub config: Box<Rc<Config>>,
    pub account: String,
    pub secret: String,
}
impl Payment {
    pub fn with_params(config: Box<Rc<Config>>, account: String, secret: String) -> Self {
        if check_address(&account).is_none() {
            panic!("invalid account.");
        }
        if check_secret(&secret).is_none() {
            panic!("invalid secret");
        }

        Payment {
            config: config,
            account: account,
            secret: secret,
        }
    }

    pub fn get_account_seq(&self) -> u32 {
        let seq_rc = Rc::new(Cell::new(0u64));

        let acc = String::from(self.account.as_str());
        AccountInfo::new().request_account_info(self.config.clone(), acc, |x| match x {
            Ok(response) => {
                let seq = seq_rc.clone();
                seq.set(response.sequence);
            },
            Err(_) => { }
        });

       downcast_to_usize(seq_rc)
    }
}

impl PaymentI for Payment {
    fn payment<F>(&self,  to: String, amount: Amount, memo: Option<String>, op: F)
    where F: Fn(Result<TransactionTxResponse, PaymentSideKick>) {
        if check_address(&to).is_none() {
            panic!("invalid destination.");
        }
        if check_amount(&amount) == false {
            panic!("invalid Amount.");
        }

        let info = Rc::new(Cell::new("".to_string()));

        let from_rc   = Rc::new(Cell::new(String::from(self.account.as_str())));
        let secret_rc = Rc::new(Cell::new(String::from(self.secret.as_str())));

        let to_rc     = Rc::new(Cell::new(to));
        let amount_rc = Rc::new(Cell::new(amount));
        let memo_rc   = Rc::new(Cell::new(None));
        if memo.is_some() {
            let upper_hex_memo = hex_t::encode(&memo.unwrap()).to_ascii_uppercase();
            let memos = MemosBuilder::new( upper_hex_memo ).build();
            memo_rc.set(Some(vec![memos]));
        }

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let from   = from_rc.clone();
            let secret = secret_rc.clone();

            let to     = to_rc.clone();
            let amount = amount_rc.clone();
            let memo   = memo_rc.clone();

            //Get Account Seq
            let sequence = self.get_account_seq();
            let tx_json = TxJson::new(from.take(), to.take(), amount.take(), sequence, memo.take());
            if self.config.local_sign {
                let blob = SignTx::with_params(sequence, &secret.take()).pay(&tx_json);
                if let Ok(command) = LocalSignTx::new(blob).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = TransactionTx::new(secret.take(), tx_json).to_string() {
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
                if let Ok(v) = serde_json::from_str(&x) as Result<TransactionTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<PaymentSideKick, serde_json::error::Error> {
                    op(Err(v))
                }
            }
        }
    }
}
