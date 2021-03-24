extern crate jlib;

use jlib::api::ipfs::hash::only_hash;
 
use std::fs::File;
use std::io::prelude::*;

fn big_file_hash() {
    // let mut f = File::open("/mnt/c/Users/zTgx/Desktop/github.com.c/ipfs/1.png").unwrap();
    let mut f = File::open("/mnt/c/Users/zTgx/Desktop/github.com.c/ipfs/1.jpg").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    println!("buf size: {:?}", buf.len());

    let target = "QmdSNzh5fsfUTz66adXkDFVFFmDGpPSvziEp7u8bZcxYAr".to_owned();
    if let Some(hash) = only_hash( &buf ) {
        println!("big file hash: {}", hash);
        assert_eq!(hash, target);
    } else {
        println!("ipfs hash none");
    }
}

fn little_file_hash() {
    let target = "Qmf412jQZiuVUtdgnB36FXFX7xg5V6KEbSJ4dpQuhkLyfD".to_owned();
    let digest = "hello world".as_bytes().to_vec();
    let hash = only_hash( &digest ).unwrap();
    assert_eq!(hash, target);
}

enum Case {
    NoneFile,
    BigFile,
    LittleFile,
}

// ---------------------------------------------------------------------------------------------------------
//
// ipfs 主方法
//
// ---------------------------------------------------------------------------------------------------------
fn main() {
    let case = Case::BigFile;
    let case = Case::LittleFile;

    match case {
        Case::NoneFile => {

        },

        Case::BigFile => {
            big_file_hash();
        },

        Case::LittleFile => {
            little_file_hash()
        }
    }
} 