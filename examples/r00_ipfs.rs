extern crate jlib;

use jlib::api::ipfs::hash::only_hash;
 
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut f = File::open("/mnt/c/Users/zTgx/Desktop/github.com.c/ipfs/1.jpg").unwrap();
    let mut buf = Vec::new();
    f.read(&mut buf).unwrap();
    
    // target: QmdSNzh5fsfUTz66adXkDFVFFmDGpPSvziEp7u8bZcxYAr
    if let Some(hash) = only_hash( &buf ) {
        println!("hash : {}", hash);
    } else {
        println!("ipfs hash none");
    }
} 
