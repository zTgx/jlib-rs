use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::api::config::Config;

use crate::api::payment::data::*;

use crate::api::message::memo::*;
use crate::api::message::amount::Amount;

use hex;

use crate::api::message::local_sign_tx::{LocalSignTx};
use crate::base::local_sign::sign_tx::{SignTx};
use crate::base::misc::util::{
    downcast_to_string,
    check_address, check_secret, check_amount,
};
use crate::api::utils::cast::get_account_sequence;

pub fn request<F>(config: Config, account: String, secret: String, to: String, amount: Amount, memo: Option<String>, op: F)
    where F: Fn(Result<TransactionTxResponse, PaymentSideKick>) {
        if check_address(&account).is_none() {
            panic!("invalid account.");
        }
        if check_secret(&secret).is_none() {
            panic!("invalid secret");
        }

        if check_address(&to).is_none() {
            panic!("invalid destination.");
        }
        // if check_amount(&amount) == false {
        //     panic!("invalid Amount.");
        // }

        let info = Rc::new(Cell::new(String::new()));

        let from_rc   = Rc::new(Cell::new(String::from(account.as_str())));
        let secret_rc = Rc::new(Cell::new(String::from(secret.as_str())));

        let to_rc     = Rc::new(Cell::new(to));
        let amount_rc = Rc::new(Cell::new(amount));
        let memo_rc   = Rc::new(Cell::new(None));

        // Get Account Seq
        let seq_rc = get_account_sequence(&config, account.clone());
        if memo.is_some() {
            let upper_hex_memo = hex::encode(&memo.unwrap()).to_ascii_uppercase();
            let memos = MemosBuilder::new( upper_hex_memo ).build();
            memo_rc.set(Some(vec![memos]));
        }

        connect(config.addr, |out| {
            let copy = info.clone();

            let from   = from_rc.clone();
            let secret = secret_rc.clone();

            let to     = to_rc.clone();
            let amount = amount_rc.clone();
            let memo   = memo_rc.clone();

            let sequence = seq_rc;

            let tx_json = TxJson::new(from.take(), to.take(), amount.take(), sequence, memo.take());
            if config.local_sign {
                let blob = SignTx::with_params(sequence, &secret.take()).pay(&tx_json);
                if let Ok(command) = LocalSignTx::new(blob).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = TransactionTx::new(secret.take(), tx_json).to_string() {
                    println!("command: {:?}", command);
                    out.send(command).unwrap()
                }
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;

                println!("msg: {:?}", c);
                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);

        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            if let Some(status) = x["status"].as_str() {
                if status == "success" {
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
