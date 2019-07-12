
extern crate mylib;

use mylib::misc::config::*;
use mylib::contracts::solidity::*;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST2, true);
    println!("config : {:?}", config.clone());

    let mut solidity = Solidity::with_config(config.clone());

    //invoke
    //address: 
    let secret  = "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let address = "jffPph792Rp3j1QgvYkj9ptdkHAoLsUAQH".to_string();

    //concat args:
    //THe First MUST BE: function name which is to be called.
    let mut v: Vec<Arg> = vec![];
    
    // let p = Arg::new("3739".to_string(), 0);
    // v.push(p);

    // 不带参数的调用
    // let message = SolidityInvokeMessage::with_params(account, secret, address, "6236653435366262".to_string(), v);
    
    // 带参数的调用
    let p = Arg::new("30303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303466".to_string(), 0);
    v.push(p);
    let message = SolidityInvokeMessage::with_params(account, secret, address, "3834653965633366".to_string(), v);

    let mut solidity = Solidity::with_config(config);
    solidity.set_invoke_message(message);
    solidity.invoke();
}