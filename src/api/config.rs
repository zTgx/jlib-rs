
/*
    Config数据结构

    * addr       ：websocket协议的服务器地址。
    * local_sign : 交易数据是否要本地签名后再发送。
    * true -> 本地签名； false -> 不签名。
*/
#[derive(Debug)]
pub struct Config {
    pub addr        : &'static str,
    pub local_sign  : bool,
}

impl Config {
    pub fn new(addr: &'static str, local_sign: bool) -> Self {
        Config {
            addr: addr,
            local_sign: local_sign,
        }
    }
}
