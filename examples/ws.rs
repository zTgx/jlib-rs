extern crate ws;
extern crate mylib;
use mylib::misc::config::*;
use std::rc::Rc;



use ws::{Sender, Factory, Handler, connect, WebSocket, CloseCode, Handshake, Message, Error};



struct MyHandler {
    ws: Sender,
    // command: String,
}
impl MyHandler {
    pub fn send(&self) -> Result<(), ws::Error> {
        Ok(())
    }
}
impl Handler for MyHandler {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        println!("on_pen...");

        self.send()
    }

    fn on_message(&mut self, msg: Message) -> Result<(), ws::Error> {
        println!("Client got message '{}'. ", msg);

        // All of the data has been sent, let's close
        // self.ws.close(CloseCode::Normal)

        // Err(ws::Error::new(ws::ErrorKind::Internal, "Invalid encountered!"))

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closing due to ({:?}) {}", code, reason);
    }

    fn on_error(&mut self, err: Error) {
        // self.ws.close();
        println!("on_error.");
    }
}

struct MyFactory {
    // pub command: String,
}

impl MyFactory {
    pub fn new(_command: String) -> Self {
        MyFactory {
            // command: command,
        }
    }
}

impl Factory for MyFactory {
    type Handler = MyHandler;

    fn connection_made(&mut self, ws: Sender) -> MyHandler {
        println!("connect_made....");
        MyHandler {
            ws: ws,
            // command: "".to_owned() + self.command.as_str(),
        }
    }

    fn client_connected(&mut self, ws: Sender) -> MyHandler {
        println!("connected.........");

        MyHandler {
            ws: ws,
            // command: "".to_owned() + self.command.as_str(),
        }
    }
}

struct Remote {
    pub ws: Box<Rc<WebSocket<MyFactory>>>,
    pub config: Box<Rc<Config>>,
}
impl Remote {
    pub fn with_config(config: Box<Rc<Config>>) -> Self {
        let factory = MyFactory::new("dummy".to_string());
        Remote {
            ws:  Box::new( Rc::new( WebSocket::new(factory).unwrap() )).clone(),
            config: config,
        }
    }

    pub fn connect(&mut self) {   
        
        let url = url::Url::parse(self.config.addr).map_err(|err| {
            // Error::new(
            //     ErrorKind::Internal,
            //     format!("Unable to parse {} as url due to {:?}", url, err),
            // )
        }).unwrap();

        // self.ws.connect(url).unwrap();
        Rc::get_mut(&mut self.ws).unwrap().connect(url).unwrap();

        // if let Ok(ref mut x) = self.ws {
        //     println!("connect...{:?}", url);
        //     x.connect(url).unwrap();
        // }
    }

    pub fn send(&mut self, command: String) {
        
        // if let Ok(ref mut x) = self.ws {
        //     x.run();
        // }
        // if let Ok(x) = &self.ws {
        //     println!("send..........{}", &command);
        //     println!("sent : {:?}", x.broadcaster().send(command));
        // }
        
        self.ws.broadcaster().send(command);
    }

    pub fn run(&mut self) {
        // let x = &self.ws;
        // x.run().unwrap();

        // if let Some(x) = Rc::get_mut(&mut self.ws.clone()) {
        //     match self.ws.clone().run() {
        //         Ok(y) => self.ws = Box::new( Rc::new( y ) ),
        //         Err(_) => {},
        //     }
        // }
    }
}
fn main() {
    let factory = MyFactory{};
    let mut ws = WebSocket::new(factory).unwrap();
    let url = "ws://ts5.jingtum.com:5020";
    let url = url::Url::parse(url).map_err(|err| { }).unwrap();
    ws.connect(url).unwrap();

    let command = "{\"id\":1,\"command\":\"server_info\"}".to_string();

    let broacaster = ws.broadcaster();
    broacaster.send(command);

    ws.run().unwrap();


    //------------------
    // let config: Box<Rc<Config>> = Config::new(TEST1, true);
    // println!("config : {:?}", config);

    // let mut ret = Remote::with_config(config);

    // let command = "{\"id\":1,\"command\":\"server_info\"}".to_string();

    // ret.connect();
    // ret.send(command);
    // ret.run();

    // println!("---------------");

    //-----
    // let mut me = ws::WebSocket::new(|_| {
    //     move |msg| {
    //         println!("got message: {}", msg);
    //         Ok(())
    //     }
    // }).unwrap();
    // me.connect(url::Url::parse(TEST1).unwrap()).unwrap();
    // let broacaster = me.broadcaster();


    
    // let command = "{\"id\":1,\"command\":\"server_info\"}".to_string();
    // println!("command : {}", command);
    // broacaster.send(command).unwrap();

    // let command = "{\"id\":2,\"command\":\"server_info\"}".to_string();
    // println!("command : {}", command);
    // broacaster.send(command).unwrap();



    // me.run();
}