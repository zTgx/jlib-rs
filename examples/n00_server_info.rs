extern crate jlib;

use jlib::api::config::Config;
use jlib::api::server_info::api::request;
use jlib::api::server_info::data::ServerInfoResponse;

pub static TEST_SERVER: &'static str = "ws://101.200.176.249:5040";

fn main() {
    let config = Config::new(TEST_SERVER, true);
    // request(config, |x| match x {
    //     Ok(response) => {
    //         println!("build_version : {:?}", response.build_version);
    //     },
    //     Err(_) => {
    //         println! ("error occured.");
    //     }
    // });

    request(config, callback);

    fn callback(server_info: Result<ServerInfoResponse, serde_json::error::Error>) {
        match server_info {
            Ok(response) => {
                println!("build_version : {:?}", response.build_version);
            },
            Err(_) => {
                println! ("error occured.");
            }
        }
    }
}
