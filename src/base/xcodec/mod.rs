pub mod address_codec;
pub mod util;

use crate::base::xcodec::address_codec::CodecFactory;
use crate::base::xcodec::address_codec::XCodeI;
use crate::base::data::base_data::{
    Address, Seed,
};

pub fn is_valid_address(string: &String) -> Option<bool> {
    let arg = Box::new( Address::default() );
    if CodecFactory::decode(&string, arg).is_none() {
        return None;
    }

    Some(true)
}

pub fn is_valid_seed(string: String) -> Option<bool> {
    let arg = Box::new( Seed::default() );
    if CodecFactory::decode(&string, arg).is_none() {
        return None;
    }

    Some(true)
}
