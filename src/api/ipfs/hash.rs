
use basex_rs::{BaseX, Decode, BITCOIN, Encode};
use ipfs_unixfs::file::adder::FileAdder;
use multihash::{Code};
use multihash::MultihashDigest;

pub fn only_hash(digest : &Vec<u8>) -> Option<String> { 
    let mut file_addr = FileAdder::builder().build();

    let hash = Code::Sha2_256.digest( &digest );
    file_addr.push( &hash.to_bytes() );

    let mut finish = file_addr.finish();

    if let Some( (cid, _block) ) = finish.next() {
        let data = cid.to_bytes();
        let hash = cid.hash();
        println!("data: {:#?}", cid);

        return Some( BaseX::new(BITCOIN).encode(&data) );
    }

    None
}