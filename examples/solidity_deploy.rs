
extern crate jlib;

use jlib::misc::config::*;
use jlib::{SolidityDeploy, SolidityInitResponse};

fn main() {
    let config = Config::new(TEST1, true);
    let account = "jHb9CJAWyB4jr91VRWn96DkukG4bwdtyTh".to_string();
    let secret  = "snoPBjXtMeMyMHUVTgbuqAfg1SUTb".to_string();
    let payload = "608060405234801561001057600080fd5b5060a48061001f6000396000f3fe6080604052348015600f57600080fd5b50600436106044577c0100000000000000000000000000000000000000000000000000000000600035046384e9ec3f81146049575b600080fd5b606360048036036020811015605d57600080fd5b50356075565b60408051918252519081900360200190f35b9056fea165627a7a7230582085890b2dceadbce6c6e9939a89026c2cc9b81b899445d5109cba8087166134a20029".to_string();

    // let message = SolidityInitMessage::with_params(account, secret, payload);

    // let mut solidity = Solidity::with_config(config.clone());
    // solidity.set_init_message(message);
    // solidity.deploy(|x| match x {
    //     Ok(response) => {
    //         let res: SolidityInitResponse = response;
    //         println!("deploy contract : {:?}", &res);
    //     },
    //
    //     Err(err) => {
    //         println!("err: {:?}", err);
    //     }
    // });

    SolidityDeploy::with_params(config, &account, &secret).deploy(&payload, |x| match x {
        Ok(response) => {
            let res: SolidityInitResponse = response;
            println!("deploy contract : {:?}", &res);
        },

        Err(err) => {
            println!("err: {:?}", err);
        }
    });
}
