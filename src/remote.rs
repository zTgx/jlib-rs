
extern crate futures;

use std::rc::Rc;
use std::any::Any;
use std::cell::Cell;

extern crate ws;
use ws::{connect, CloseCode};
use serde_json::json;
use serde_json::{Value};

use crate::config::*;

use crate::commands::command_trait::*;
use crate::commands::command_subscribe::*;
use crate::commands::command_serverinfo::*;
use crate::commands::command_ledger_closed::*;
use crate::commands::command_request_ledger::*;
use crate::commands::command_request_accountinfo::*;
use crate::commands::command_request_accounttums::*;
use crate::commands::command_request_account_relations::*;
use crate::commands::command_request_account_offer::*;
use crate::commands::command_request_account_tx::*;
use crate::commands::command_request_order_book::*;
use crate::commands::command_request_brokerage::*;
use crate::commands::command_request_tx::*;

use crate::transaction::*;
use crate::common::*;
use crate::relation::*;
use crate::offer_create::*;

pub struct Conn {
    conn: Option<Rc<ws::Sender>>,
}

impl Conn {
    pub fn new(out: Option<Rc<ws::Sender>>) -> Self {
        Conn {
            conn: out,
        }
    }
    pub fn request_server_info(&self) {
        use serde_json::json;
        let json = json!({ "id": "1", "command": "server_info" });
        let compact = format!("{}", json);
        println!("compact : {}", compact);
   
    }
}

pub struct Remote {
    addr: &'static str,
    local_sign: bool,
    conn: Option<Conn>,
}

//TODO::Remote 中的请求接口，要单独处理封装～～～（待实现）
impl Remote  {
    pub fn new(addr: &'static str, local_sign: bool) -> Self {
        Remote {
            addr: addr,
            local_sign: local_sign,
            conn: None,
        }
    }

    pub fn with_config<F>(config: Box<Rc<Config>>, op: F) 
        where F: Fn(Result<SubscribeResponse, &'static str>) {

        let ws_message = Rc::new(Cell::new("".to_string()));

        connect(config.addr, |out| {
            
            let cloned_ws_message = ws_message.clone();

            if let Ok(command) = SubscribeCommand::default().to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let text = msg.as_text()?;
                cloned_ws_message.set(text.to_string());

                out.close(CloseCode::Normal)
            }
        }).unwrap();

        //parse 
        let resp = Remote::print_if(ws_message);
        {
            //TOD::解析的类可以抽象出来～～～
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                //println!("x : {:?}", x["result"]);
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<SubscribeResponse, serde_json::error::Error> {
                    println!("subscribe response : {:?}", x);

                    op(Ok(x))
                }
            }
        }

        //op(Ok(ws::Message::text(resp)))
    } 
    
    pub fn print_if(value: Rc<dyn Any>) -> String {
        match value.downcast::<Cell<String>>() {
            Ok(string) => {
                return string.take();
            },
            Err(_) => { "None".to_string() }
        }
    }
    
    pub fn request_server_info<F> (config: Box<Rc<Config>>, op: F)
        where F: Fn(Result<ServerInfoResponse, &'static str>) {
        
        let info = Rc::new(Cell::new("".to_string()));

        connect(config.addr, |out| { 
            let copy = info.clone();

            if let Ok(command) = ServerInfoCommand::default().to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = Remote::print_if(info);
        println!("resp : {}", &resp);
        {
            //TOD::解析的类可以抽象出来～～～
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                    let x: String = x["info"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<ServerInfoResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }
        }


    }

    pub fn request_ledger_closed<F>(config: Box<Rc<Config>>, op: F)
        where F: Fn(Result<LedgerClosedResponse, &'static str>) {
            let info = Rc::new(Cell::new("".to_string()));

            connect(config.addr, |out| { 
                let copy = info.clone();

                if let Ok(command) = LedgerClosedCommand::default().to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<LedgerClosedResponse, serde_json::error::Error> {
                    op(Ok(x));
                }
            }

    }

    pub fn request_ledger<F>(config: Box<Rc<Config>>, ledger_index: Option<u64>, ledger_hash: Option<String>, transactions: bool, op: F) 
        where F: Fn(Result<RequestLedgerResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let ledger_index_rc = Rc::new(Cell::new(None));
            if ledger_index.is_some() {
                ledger_index_rc.set(ledger_index);
            }
            let ledger_hash_rc = Rc::new(Cell::new(None));
            if ledger_hash.is_some() {
                ledger_hash_rc.set(ledger_hash);
            }
            let transactions_rc = Rc::new(Cell::new(transactions));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let index = ledger_index_rc.clone();
                let hash = ledger_hash_rc.clone();
                let trans = transactions_rc.clone();
                if let Ok(command) = RequestLedgerCommand::with_params(index.take(), hash.take(), trans.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                    let x: String = x["ledger"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestLedgerResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }         
    }

    //此方法调试connect
    pub fn request_account_info<F>(config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountInfoResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                if let Ok(command) = RequestAccountInfoCommand::with_params(account.take()).to_string() {
                    out.send(command).unwrap();
                }

                println!("zhtian@remote.connect.");
                move |msg: ws::Message| {
                    println!("zhtian@msg");

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            //println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(x) = serde_json::from_str(&x) as Result<Value, serde_json::error::Error> {
                    let x: String = x["account_data"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountInfoResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                }
            }         
    }

    pub fn request_account_tums<F>(config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountTumsResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));

            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                if let Ok(command) = RequestAccountTumsCommand::with_params(account.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountTumsResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    //[[[接口@4.8~4.11 参数一直，考虑后期合并。]]]!!!
    pub fn request_account_relations<F>(config: Box<Rc<Config>>, account: String, relation_type: Option<String>, op: F) 
        where F: Fn(Result<RequestAccountRelationsResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));
            let relation_type_rc = Rc::new(Cell::new(relation_type));
            connect(config.addr, |out| { 
                let copy = info.clone();

                let account = account_rc.clone();
                let relation_type = relation_type_rc.clone();
                if let Ok(command) = RequestAccountRelationsCommand::with_params(account.take(), relation_type.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountRelationsResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    pub fn request_account_offer<F>(config: Box<Rc<Config>>, account: String, op: F) 
        where F: Fn(Result<RequestAccountOfferResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));
            connect(config.addr, |out| { 
                let copy = info.clone();
                let account = account_rc.clone();
                if let Ok(command) = RequestAccountOfferCommand::with_params(account.take()).to_string() {
                    out.send(command).unwrap();
                }

                move |msg: ws::Message| {

                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountOfferResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    pub fn request_account_tx<F>(config: Box<Rc<Config>>, account: String, limit: Option<u64>, op: F) 
        where F: Fn(Result<RequestAccountTxResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let account_rc = Rc::new(Cell::new(account));
            let limit_rc = Rc::new(Cell::new(limit));
            connect(config.addr, |out| { 
                let copy = info.clone();
                let account = account_rc.clone();
                let limit = limit_rc.clone();
                if let Ok(command) = RequestAccountTxCommand::with_params(account.take(), limit.take()).to_string() {
                    out.send(command).unwrap();
                }

                //返回一个Handler类型(trait)，等待epoll调用。
                move |msg: ws::Message| {
                    println!("返回Hanler类型closure。");
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            //println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                println!("x : {}", x);
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    pub fn request_order_book<F>(config: Box<Rc<Config>>, gets: OrderBookItem, pays: OrderBookItem, op: F) 
        where F: Fn(Result<RequestOrderBookResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let gets_rc = Rc::new(Cell::new(gets));
            let pays_rc = Rc::new(Cell::new(pays));
            connect(config.addr, |out| { 
                let copy = info.clone();

                let gets = gets_rc.clone();
                let pays = pays_rc.clone();

                //使用take（）的对象要实现Default trait，因为 take（） 调用后，原始值会调用default（）
                //OrderBookItem, add #[derive(Default)] or impl default trait.
                if let Ok(command) = RequestOrderBookCommand::with_params(gets.take(), pays.take()).to_string() {
                    out.send(command).unwrap();
                }

                //返回一个Handler类型(trait)，等待epoll调用。
                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                println!("x : {}", x);
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestOrderBookResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    pub fn request_brokerage<F>(config: Box<Rc<Config>>, issuer: String, app: u64, currency: String, op: F) 
        where F: Fn(Result<RequestBrokerageResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let issuer_rc = Rc::new(Cell::new(issuer));
            let app_rc = Rc::new(Cell::new(app));
            let currency_rc = Rc::new(Cell::new(currency));
            
            connect(config.addr, |out| { 
                let copy = info.clone();

                let issuer = issuer_rc.clone();
                let app = app_rc.clone();
                let currency = currency_rc.clone();

                if let Ok(command) = RequestBrokerageCommand::with_params(issuer.take(), app.take(), currency.take()).to_string() {
                    out.send(command).unwrap();
                }

                //返回一个Handler类型(trait)，等待epoll调用。
                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                println!("x : {}", x);
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestBrokerageResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    pub fn request_tx<F>(config: Box<Rc<Config>>, hash: String,  op: F) 
        where F: Fn(Result<RequestTxResponse, &'static str>) {

            let info = Rc::new(Cell::new("".to_string()));

            let hash_rc = Rc::new(Cell::new(hash));
            
            connect(config.addr, |out| { 
                let copy = info.clone();

                let hash = hash_rc.clone();

                if let Ok(command) = RequestTxCommand::with_params(hash.take()).to_string() {
                    out.send(command).unwrap();
                }

                //返回一个Handler类型(trait)，等待epoll调用。
                move |msg: ws::Message| {
                    let c = msg.as_text()?;
                    copy.set(c.to_string());
                    
                    out.close(CloseCode::Normal) 
                }
            
            }).unwrap();
            
            let resp = Remote::print_if(info);
            println!("resp : {}", &resp);
            if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
                let x: String = x["result"].to_string();
                println!("x : {}", x);
                if let Ok(v) = serde_json::from_str(&x) as Result<RequestTxResponse, serde_json::error::Error> {
                    op(Ok(v))
                }
            }         
    }

    /*
    4.15支付
    */
    pub fn build_payment_tx<F>(config: Box<Rc<Config>>, from: String, to: String, amount: Amount, 
                                                     memo: Option<String>, 
                                                     secret: Option<String>, 
                                                     op: F) 
        where F: Fn(Result<TransactionTxResponse, &'static str>) {
        //params check
        // var tx = new Transaction(this);
        // if (options === null || typeof options !== 'object') {
        //     tx.tx_json.obj = new Error('invalid options type');
        //     return tx;
        // }
        // var src = options.source || options.from || options.account;
        // var dst = options.destination || options.to;
        // var amount = options.amount;
        // if (!utils.isValidAddress(src)) {
        //     tx.tx_json.src = new Error('invalid source address');
        //     return tx;
        // }
        // if (!utils.isValidAddress(dst)) {
        //     tx.tx_json.dst = new Error('invalid destination address');
        //     return tx;
        // }
        // if (!utils.isValidAmount(amount)) {
        //     tx.tx_json.amount = new Error('invalid amount');
        //     return tx;
        // }

        let info = Rc::new(Cell::new("".to_string()));

        let from_rc = Rc::new(Cell::new(from));
        let to_rc = Rc::new(Cell::new(to));
        let amount_rc = Rc::new(Cell::new(amount));
        let memo_rc = Rc::new(Cell::new(None));
        if memo.is_some() {
            let mut v: Vec<Memo> = Vec::new();
            v.push(Memo::new(MemoData::new(string_to_hex(&memo.unwrap()))));

            memo_rc.set(Some(v));
        }

        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let from = from_rc.clone();
            let to   = to_rc.clone();
            let amount = amount_rc.clone();
            let memo   = memo_rc.clone();

            let secret = secret_rc.clone();

            if let Ok(command) = TransactionTx::new(secret.take(), TxJson::new(from.take(), to.take(), amount.take(), memo.take())).to_string() {
                out.send(command).unwrap()
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = Remote::print_if(info);
        println!("resp : {}", &resp);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            //println!("x : {}", x);
            if let Ok(v) = serde_json::from_str(&x) as Result<TransactionTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }

    /*
    4.16设置关系
    */
    pub fn build_relation_tx<F>(config: Box<Rc<Config>>, account: String, target: String, relation_type: u64, amount: Amount, 
                                                     secret: Option<String>, 
                                                     op: F) 
        where F: Fn(Result<RelationTxResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let target_rc = Rc::new(Cell::new(target));
        let relation_type_rc = Rc::new(Cell::new(relation_type));
        let amount_rc = Rc::new(Cell::new(amount));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let account = account_rc.clone();
            let target   = target_rc.clone();
            let relation_type = relation_type_rc.clone();
            let amount = amount_rc.clone();
            let secret = secret_rc.clone();

            if let Ok(command) = RelationTx::new(secret.take(), RelationTxJson::new(account.take(), target.take(), relation_type.take(), amount.take())).to_string() {
                out.send(command).unwrap()
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = Remote::print_if(info);
        println!("resp : {}", &resp);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            //println!("x : {}", x);
            if let Ok(v) = serde_json::from_str(&x) as Result<RelationTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }

    /*
    4.18挂单
    */
    pub fn build_offer_create_tx<F>(config: Box<Rc<Config>>, account: String, taker_gets: AmountTest, taker_pays: AmountTest, 
                                                     secret: Option<String>, 
                                                     op: F) 
        where F: Fn(Result<OfferCreateTxResponse, &'static str>) {

        let info = Rc::new(Cell::new("".to_string()));

        // let typ0_rc = Rc::new(Cell::new(typ0));
        let account_rc = Rc::new(Cell::new(account));
        // let app_rc = Rc::new(Cell::new(app));
        let taker_gets_rc = Rc::new(Cell::new(taker_gets));
        let taker_pays_rc = Rc::new(Cell::new(taker_pays));
        let secret_rc = Rc::new(Cell::new(secret));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            // let typ0 = typ0_rc.clone();
            let account = account_rc.clone();
            // let app   = app_rc.clone();
            let taker_gets = taker_gets_rc.clone();
            let taker_pays = taker_pays_rc.clone();
            let secret = secret_rc.clone();

            if let Ok(command) = OfferCreateTx::new(secret.take(), OfferCreateTxJson::new(account.take(), 
                                                                            taker_gets.take(), "1000000".to_string())).to_string() {
                out.send(command).unwrap()
            }

            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = Remote::print_if(info);
        println!("resp : {}", &resp);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            //println!("x : {}", x);
            if let Ok(v) = serde_json::from_str(&x) as Result<OfferCreateTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         

    }
}