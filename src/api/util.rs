use crate::base::misc::util::{downcast_to_usize};
use crate::api::account_info;

use std::cell::Cell;
use std::rc::Rc;
use crate::api::config::Config;

pub fn get_account_sequence(config: &Config, account: String) -> u32 {
    let seq_rc = Rc::new(Cell::new(0u64));

    account_info::api::request(config, account, |x| match x {
        Ok(response) => {
            let seq = seq_rc.clone();
            seq.set(response.sequence);
        },
        Err(_) => { }
    });

   downcast_to_usize(seq_rc)
} 

