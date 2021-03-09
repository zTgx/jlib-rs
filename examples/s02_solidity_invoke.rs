
extern crate jlib;
use jlib::api::contracts::solidity::{SolidityCall, SolidityInvokeResponse};
use jlib::api::contracts::message::Arg;

use jlib::api::config::Config;
pub static TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

fn main() {
    let config = Config::new(TEST_SERVER, true);

    //invoke
    let secret  = "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let address = "jJpU95p4ekWaJJZ7biS7oSM1QGsetxb269".to_string();

    let method_name = "0x84e9ec3f".to_string();

    //concat args:
    //THe First MUST BE: function name which is to be called.
    let mut v: Vec<Arg> = vec![];

    let p = Arg::new("3739".to_string(), 0);
    v.push(p);

    // no params
    // let message = SolidityInvokeMessage::with_params(account, secret, address, "6236653435366262".to_string(), v);

    // with params
    // let p = Arg::new("79".to_string(), 0);
    // v.push(p);
    // let message = SolidityInvokeMessage::with_params(account, secret, address, "0x84e9ec3f".to_string(), v);
    //
    // let mut solidity = Solidity::with_config(config);
    // solidity.set_invoke_message(message);
    // solidity.invoke(|x| match x {
    //     Ok(response) => {
    //         let res: SolidityInvokeResponse = response;
    //         println!("contract info : {:?}", &res);
    //     },
    //
    //     Err(err) => {
    //         println!("err: {:?}", err);
    //     }
    // });

    SolidityCall::with_params(config, &account, &secret, &address).call(&method_name, v, |x| match x {
        Ok(response) => {
            let res: SolidityInvokeResponse = response;
            println!("call contract : {:?}", &res);
        },

        Err(err) => {
            println!("err: {:?}", err);
        }
    });
}
