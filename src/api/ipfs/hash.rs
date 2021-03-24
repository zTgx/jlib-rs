
use ipfs_unixfs::file::adder::FileAdder;
use basex_rs::{BaseX, BITCOIN, Encode}; 

use ipfs_unixfs::file::adder::Chunker;
use ipfs_unixfs::file::adder::BalancedCollector;

/*
    【输入】 digest 需要计算hash的文件字节流
    【输出】 Option类型、计算后的hash
*/
pub fn only_hash(digest : &Vec<u8>) -> Option<String> { 
    // let mut file_addr = FileAdder::builder().build();
    // file_addr.push( &digest );

    // println!("finish: {:#?}", file_addr);

    // let mut finish = file_addr.finish();


    // if let Some( (cid, _block) ) = finish.next() {
    //     let data = cid.to_bytes();

    //     let hash = BaseX::new(BITCOIN).encode( &data );

    //     return Some( hash );
    // }

    // None

    // only_hash_chunk(&digest)

    // only_hash_v3(&digest)

    only_hash_v4(&digest)
}

pub fn only_hash_chunk(digest: &Vec<u8>) -> Option<String> {
    let mut file_addr = FileAdder::builder().build();
    file_addr.push( &digest );

    println!("finish: {:#?}", file_addr);

    let mut finish = file_addr.finish();


    let cid = finish.next().unwrap().0;
    println!("cid: {:?}", cid);
     {
        let data = cid.to_bytes();

        let hash = BaseX::new(BITCOIN).encode( &data );

        return Some( hash );
    }

    None
}

pub fn only_hash_v3(digest: &Vec<u8>) -> Option<String> {
    let mut adder = FileAdder::default();
    let mut last_block = String::new();
    // println!("digest.len: {}", digest.len());

    let mut total = 0;
    loop {
        if total >= digest.len() {
            // eprintln!("finishing");
            // println!("adder: {:#?}", adder);

            let mut blocks = adder.finish();

            let next = blocks.next();

            let item = next.unwrap().0;
            // println!("cid : {:?}", item);

            let hash = BaseX::new(BITCOIN).encode( &item.to_bytes() );
            // println!("hash: xxx {:?}", hash);
            last_block = hash;

            break;
        }

        while total < digest.len() {
            let (blocks, consumed) = adder.push(&digest[total..]);
            total += consumed;

            // println!("totoal: {}", total);
            // println!("consumed: {}", consumed);
        }
    }

    Some( last_block )
}

pub fn only_hash_v4(digest: &Vec<u8>) -> Option<String> {
    let mut adder = FileAdder::default();

    let mut amt = 0;
    let mut written = 0;
    let mut blocks_received = Vec::new();

    if amt == 0 {
        amt = digest.len();
    }

    while written < digest.len() {
        let end = written + (digest.len() - written).min(amt);
        let slice = &digest[written..end];

        let (blocks, pushed) = adder.push(slice);
        blocks_received.extend(blocks);
        written += pushed;
    }

    let last_blocks = adder.finish();

    blocks_received.extend(last_blocks);

    let blocks_len = blocks_received.len();
    if blocks_len > 0 {
        let last = &blocks_received[blocks_len - 1].0.to_bytes();
        let hash = BaseX::new(BITCOIN).encode( &last );

        return Some( hash );
    }

    None
}


// ---------------------------------------------------------------------------------------------------------
//
// ipfs 测试用例
//
// ---------------------------------------------------------------------------------------------------------
// cargo test api::ipfs::hash::tests::only_hash_test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_hash_test() {
        let target = "Qmf412jQZiuVUtdgnB36FXFX7xg5V6KEbSJ4dpQuhkLyfD".to_owned();
        let digest = "hello world".as_bytes().to_vec();
        let hash = only_hash( &digest ).unwrap();
        assert_eq!(hash, target);
    }
}