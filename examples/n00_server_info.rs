extern crate jlib;
use jlib::api::query::server_info::*;

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://47.104.108.255:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    ServerInfo::new().request_server_info(config, |x| match x {
        Ok(response) => {
            println!("build_version : {:?}", response.build_version);
        },
        Err(_) => {
            println! ("error occured.");
        }
    });
}
