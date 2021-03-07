// Copyright 2019-2020 zTgx <beautifularea@163.com>.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! How to use jlib-rs
//!
//!
//! # Quick Start
//!
//! To get you started quickly, the easiest and highest-level way to get
//! current server info is to use [`request_server_info`]; 
//!
//! ```
//! use jlib::api::query::server_info::*;
//! use jlib::Config;
//! pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";
//! 
//! fn main() {
//!     let config = Config::new(TEST_SERVER, true);
//!     ServerInfo::new().request_server_info(config, |x| match x {
//!         Ok(response) => {
//!             println!("build_version : {:?}", response.build_version);
//!         }
//!         Err(_) => {
//!             println! ("error occured.");
//!         }
//!     });
//! }
//! 
//! ```
//!
//! # Wiki
//!
//! For the user guide and further documentation, please read
//! [jlib-wiki](https://github.com/zTgx/jlib-rs/wiki).

#[macro_use]
extern crate lazy_static;
extern crate typename;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate num;
extern crate void;
extern crate ws;
extern crate basex_rs;
extern crate hex;
extern crate phf;
extern crate libsm;
extern crate crypto;
extern crate rfc1751;

// exports
pub mod api;
pub mod message;
pub mod base;

#[derive(Debug)]
pub struct Config {
    pub addr        : &'static str, // server address 
    pub local_sign  : bool,         // local sign flag
}

impl Config {
    pub fn new(addr: &'static str, local_sign: bool) -> Self {
        Config {
            addr: addr,
            local_sign: local_sign,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WalletType {
    SECP256K1,
    ED25519,
    SM2P256V1, // sm2p256v1
}

