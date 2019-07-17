

// pub struct Conn {
//     conn: Option<Rc<ws::Sender>>,
// }

// impl Conn {
//     pub fn new(out: Option<Rc<ws::Sender>>) -> Self {
//         Conn {
//             conn: out,
//         }
//     }
//     pub fn request_server_info(&self) {
//         use serde_json::json;
//         let json = json!({ "id": "1", "command": "server_info" });
//         let compact = format!("{}", json);
//         println!("compact : {}", compact);
   
//     }
// }

pub struct Remote {
    // addr: &'static str,
    // local_sign: bool,
    // conn: Option<Conn>,
}

//TODO::Remote 中的请求接口，要单独处理封装～～～（待实现）
impl Remote  {
    // pub fn new(addr: &'static str, local_sign: bool) -> Self {
    //     Remote {
    //         // addr: addr,
    //         // local_sign: local_sign,
    //         // conn: None,
    //     }
    // }


}