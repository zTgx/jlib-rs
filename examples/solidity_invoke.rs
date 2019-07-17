
extern crate jlib;

use jlib::misc::config::*;
use jlib::contracts::solidity::*;

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    println!("config : {:?}", config.clone());

    //invoke
    //address: 
    let secret  = "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let address = "j94WCA3ZSTWv2SkjxHh7friknF7PmyvDiU".to_string();

    //concat args:
    //THe First MUST BE: function name which is to be called.
    let mut v: Vec<Arg> = vec![];
    
    // let p = Arg::new("3739".to_string(), 0);
    // v.push(p);

    // 不带参数的调用
    // let message = SolidityInvokeMessage::with_params(account, secret, address, "6236653435366262".to_string(), v);
    
    // 带参数的调用
    let p = Arg::new("79".to_string(), 0);
    v.push(p);
    let message = SolidityInvokeMessage::with_params(account, secret, address, "0x84e9ec3f".to_string(), v);

    let mut solidity = Solidity::with_config(config);
    solidity.set_invoke_message(message);
    solidity.invoke();
}