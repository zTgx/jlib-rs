extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::server_info::*;

fn main() {
    let config = Config::new(TEST3, true);
    ServerInfo::new().request_server_info(config.clone(), |x| match x {
        Ok(response) => {
            println!("build_version : {:?}", response.build_version);
        }
        Err(_) => {
	    println! ("error occured.");
        }
    });
}
