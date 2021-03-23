
use ipfs_unixfs::file::adder::FileAdder;
use basex_rs::{BaseX, BITCOIN, Encode}; 

/*
    【输入】 digest 需要计算hash的文件字节流
    【输出】 Option类型、计算后的hash
*/
pub fn only_hash(digest : &Vec<u8>) -> Option<String> { 
    let mut file_addr = FileAdder::builder().build();
    file_addr.push( &digest );
    let mut finish = file_addr.finish();

    if let Some( (cid, _block) ) = finish.next() {
        let data = cid.to_bytes();

        let hash = BaseX::new(BITCOIN).encode( &data );

        return Some( hash );
    }

    None
}
