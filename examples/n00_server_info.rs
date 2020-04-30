extern crate jlib;
use jlib::api::query::server_info::*;

use jlib::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

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
