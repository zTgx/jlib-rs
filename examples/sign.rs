
extern crate hex;
extern crate mylib;

use mylib::base::sign::*;

fn main () {
    let message = "11D06DFD3CDC4D8FE00214879A97B3E4B40B75F8DF82D89ECE72A96066A52583";
    let key = [ 154, 130, 174, 40, 14, 141, 200, 71, 58, 158, 235, 154, 10, 79, 66, 243, 
                85, 228, 136, 132, 102, 248, 112, 40, 17,  16, 1, 139, 57, 145, 205, 182 ];

    if let Ok(msg) = hex::decode(message) {
        let signed_hex_string = SignatureX::sign(&msg, &key);
        println!("signed_hex_string: {}", signed_hex_string);

        //verify
        if let Ok(sig) = hex::decode(signed_hex_string) {
            let ret = SignatureX::verify(&msg, &sig, &key);
            println!("verify result : {}", ret);
        }
    }
}