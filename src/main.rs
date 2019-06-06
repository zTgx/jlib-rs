
mod remote;
mod server_info;
use remote::Remote;

fn main() {
    println!("Hello, world!");

    let ret = Remote::new("ws://127.0.0.1:5060", false);
    ret.connect();
}
