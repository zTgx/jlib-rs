extern crate jlib;
use jlib::api::contracts::abi::function_hash;

fn main() {
    let hashed = function_hash("retrieve()");
    assert_eq!(hashed, "2e64cec1".to_owned());
}