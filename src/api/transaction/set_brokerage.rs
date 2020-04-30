extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::Config;
use crate::message::transaction::set_brokerage::*;
use crate::message::common::command_trait::CommandConversion;
use crate::message::common::amount::Amount;
use crate::message::transaction::local_sign_tx::{LocalSignTx};
use crate::base::local_sign::sign_tx::{SignTx};
use crate::base::misc::util::{
    downcast_to_string,
    check_address, check_secret, check_amount,
};
use crate::api::query::get_account_sequence;

pub trait BrokerageManageI {
    fn set_rate<F>(&self, den: u64, num: u64, amount: Amount, op: F)
    where F: Fn(Result<SetBrokerageTxResponse, SetBrokerageSideKick>);
}

pub struct BrokerageManage {
    pub config  : Config,
    pub account : String,
    pub secret  : String,
    pub fee_account: String,
}
impl BrokerageManage {
    pub fn with_params(config: Config, account: String, secret: String, fee_account: String) -> Self {
        if check_address(&account).is_none() {
            panic!("invalid account.");
        }

        if check_secret(&secret).is_none() {
            panic!("invalid secret");
        }

        if check_secret(&fee_account).is_none() {
            panic!("invalid fee_account");
        }

        BrokerageManage {
            config: config,
            account: account,
            secret: secret,
            fee_account: fee_account,
        }
    }
}

impl BrokerageManageI for BrokerageManage {
    fn set_rate<F>(&self, den: u64, num: u64, amount: Amount, op: F)
    where F: Fn(Result<SetBrokerageTxResponse, SetBrokerageSideKick>) {
        if num <= 0 {
            panic!("invalid num.");
        }

        if check_amount(&amount) == false {
            panic!("invalid Amount.");
        }

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc       = Rc::new(Cell::new(String::from(self.account.as_str())));
        let secret_rc        = Rc::new(Cell::new(String::from(self.secret.as_str())));
        let fee_account_rc   = Rc::new(Cell::new(String::from(self.fee_account.as_str())));

        let den_rc   = Rc::new(Cell::new( den ));
        let num_rc = Rc::new(Cell::new( num ));
        let amount_rc = Rc::new(Cell::new(amount));

        // Get Account Seq
        let account_seq = get_account_sequence(&self.config, self.account.clone());

        connect(self.config.addr, |out| {
            let copy = info.clone();

            let account     = account_rc.clone();
            let secret      = secret_rc.clone();
            let fee_account = fee_account_rc.clone();

            let den     = den_rc.clone();
            let num     = num_rc.clone(); 
            let amount  = amount_rc.clone();

            let account = account.take();

            let tx_json = SetBrokerageTxJson::new(account, fee_account.take(), account_seq, den.take(), num.take(), amount.take());
            if self.config.local_sign {
                let blob = SignTx::with_params(account_seq, &secret.take()).set_rate(&tx_json);
                if let Ok(command) = LocalSignTx::new(blob).to_string() {
                    out.send(command).unwrap()
                }
            } else {
                if let Ok(command) = SetBrokerageTx::new(secret.take(), tx_json).to_string() {
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
                if let Ok(v) = serde_json::from_str(&x) as Result<SetBrokerageTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            } else {
                if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<SetBrokerageSideKick, serde_json::error::Error> {
                    op(Err(v))
                }
            }
        }
    }
}
