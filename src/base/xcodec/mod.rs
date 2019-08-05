pub mod address_codec;
pub mod util;

use crate::base::xcodec::address_codec::CodecFactory;
use crate::base::xcodec::address_codec::XCodeI;
use crate::base::data::base_data::{
    Address,
};

pub fn is_valid_address(string: &String) -> Option<bool> {
    let arg = Box::new( Address::default() );
    if CodecFactory::decode(&string, arg).is_none() {
        return None;
    }

    Some(true)
}
