pub mod pay;
pub mod relate;
pub mod create_offer;
pub mod cancel_offer;
pub mod set_brokerage;

use std::rc::Rc;
use std::cell::Cell;
use crate::misc::config::*;

use crate::api::query::account_info::*;
use crate::base::misc::util::{
    downcast_to_usize,
};

pub fn get_account_seq(account: &String) -> u32 {
    let seq_rc = Rc::new(Cell::new(0u64));

    let config = Config::new(TEST1, true);
    AccountInfo::new().request_account_info(config.clone(), account.to_string(), |x| match x {
        Ok(response) => {
            let seq = seq_rc.clone();
            seq.set(response.sequence);
        },
        Err(_) => { }
    });

    downcast_to_usize(seq_rc)
}
