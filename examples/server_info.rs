extern crate mylib;

use mylib::misc::config::*;
use mylib::api::server_info::*;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST2, true);
    println!("config : {:?}", config);

    let _c = ServerInfo::new().request_server_info(config.clone(), |x| match x {
        Ok(response) => {
            println!("build_version : {:?}", response.build_version);
        }

        Err(_) => {
        }
    });
}