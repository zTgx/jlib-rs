pub mod server_info;
pub mod ledger_closed;
pub mod spec_ledger;
pub mod account_info;
pub mod spec_tx;
pub mod account_tums;
pub mod relations;
pub mod account_offer;
pub mod account_tx;
pub mod order_book;
pub mod brokerage;

use crate::base::misc::util::{downcast_to_usize};
use api::query::account_info::{AccountInfo, AccountInfoI};
use std::cell::Cell;
use std::rc::Rc;
use crate::api::config::Config;

pub fn get_account_sequence(config: &Config, account: String) -> u32 {
    let seq_rc = Rc::new(Cell::new(0u64));

    AccountInfo::new().request_account_info(config, account, |x| match x {
        Ok(response) => {
            let seq = seq_rc.clone();
            seq.set(response.sequence);
        },
        Err(_) => { }
    });

   downcast_to_usize(seq_rc)
} 