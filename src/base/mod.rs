
pub mod sign;
pub mod brorand;
pub mod util;
pub mod constants;
pub mod base_data;
pub mod wallet;
pub mod config;
pub mod seed;
pub mod encodex;
pub mod keypair;
pub mod address;

pub mod inverse_fields_map;
pub mod types_map;
pub mod serialized_type;


//Serialize
use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct X {
    pub m: HashMap<&'static str, i32>,
}
impl X {
    pub fn new() -> Self {
    let timber_resources: HashMap<&'static str, i32> =
                                        [("Norway", 100),
                                        ("Denmark", 50),
                                        ("Iceland", 10)]
                                        .iter().cloned().collect();
    X {
        m: timber_resources,
    }
    }
    pub fn get_value_from_key(&self, key: &str) -> Option<&i32> {
    self.m.get(key)
    }

    pub fn get_key_from_value(&self, value: i32) -> Option<&'static str> {
    let mut k = None;
    for (key, val) in self.m.iter() {
        println!("key: {} val: {}", key, val);

        if *val == value {
            k = Some(*key);

            return k;
        }
    }

    k
    }
}

lazy_static! {
    pub static ref GLOBAL_MAP: X = {
        let x = X::new();

        x
    };
}

