#![allow(unused)]

use std::fmt;
use std::rc::Rc;

pub const TEST1: &'static str = "ws://101.200.230.74:5020";
pub const TEST2: &'static str = "ws://192.168.208.130:5060";
pub const TEST3: &'static str = "ws://42.81.160.87:5020";
pub const TEST_SERVER: &'static str = "ws://42.81.160.87:5020";

pub struct Config {
    pub addr: &'static str, //服务器地址
    pub local_sign: bool,   //本地签名
}

impl Config {
    //修改为Rc,是考虑到其他接口可能要重用config配置
    pub fn new(addr: &'static str, local_sign: bool) -> Box<Rc<Self>> {
        Box::new(Rc::new( Config {
                            addr: addr,
                            local_sign: local_sign,
        } ))
    }

    //调用default，返回值类型统一为：Box<Rc<Self>>
    pub fn default_with_box() -> Box<Rc<Self>> {
        Box::new(Rc::new( Config::default() ))
    }
}

//实现default方法，不推荐直接使用。
impl Default for Config {
    fn default() -> Self {
        Config {
            addr: TEST2,
            local_sign: true,
        }
    }
}

//实现fmt方法，或者使用#[derive(Debug)]
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config {{ addr: {}, local_sign: {} }}", self.addr, self.local_sign)
    }
}
