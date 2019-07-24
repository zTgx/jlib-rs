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
use crate::base::sign_pay::*;
use crate::message::common::amount::Amount;
use crate::base::util::{downcast_to_usize, downcast_to_string};
use crate::api::query::account_info::*;

use cast_rs::hex_t;

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
        //params check
        // var tx = new Transaction(this);
        // if (options === null || typeof options !== 'object') {
        //     tx.tx_json.obj = new Error('invalid options type');
        //     return tx;
        // }
        // var src = options.source || options.from || options.account;
        // var dst = options.destination || options.to;
        // var amount = options.amount;
        // if (!utils.isValidAddress(src)) {
        //     tx.tx_json.src = new Error('invalid source address');
        //     return tx;
        // }
        // if (!utils.isValidAddress(dst)) {
        //     tx.tx_json.dst = new Error('invalid destination address');
        //     return tx;
        // }
        // if (!utils.isValidAmount(amount)) {
        //     tx.tx_json.amount = new Error('invalid amount');
        //     return tx;
        // }

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

        //Get Account Seq
        let seq = self.get_account_seq();
        let sequence_rc = Rc::new(Cell::new(seq));
        
        connect(self.config.addr, |out| {
            let copy = info.clone();

            let from   = from_rc.clone();
            let secret = secret_rc.clone();

            let to     = to_rc.clone();
            let amount = amount_rc.clone();
            let memo   = memo_rc.clone();

            let sequence = sequence_rc.clone();

            //txjson
            use crate::base::*;
            let x = secret.take();

            let signing_pub_key = Some(util::get_public_key_from_secret(&x).property.public_key);
            let d_secret = String::from(x.as_str());

            let tx_json = TxJson::new(from.take(), to.take(), amount.take(),sequence.take(),  memo.take(), signing_pub_key);

            if self.config.local_sign {
                use crate::message::transaction::local_sign_tx::*;
                let mut local_sign = SignTx::default();

                let blob = local_sign.prepare(tx_json, d_secret);
                if let Ok(command) = LocalSignTx::new(blob.unwrap()).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = TransactionTx::new(d_secret, tx_json).to_string() {
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
        println!("resp: {}", &resp);

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
