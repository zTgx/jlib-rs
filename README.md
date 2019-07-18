# jlib

Lightweight blockchain lib for Skywelld write in [Rust](http://www.rust-lang.org).
```rust

/// Request blockchain server status
let config: Box<Rc<Config>> = Config::new(TEST_SERVER, true);
ServerInfo::new().request_server_info(config.clone(), |x| match x {
    Ok(response) => {
        println!("build_version : {:?}", response.build_version);
    }

    Err(_) => {
    }
});
```

Introduction
------------
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**[Homepage](https://github.com/zTgx/lib/wiki/Home-Page)**

**[API Documentation](https://github.com/zTgx/lib/wiki/API-Documentation)**



Getting Started
---------------

For detailed installation and usage instructions, check out the [guide](https://github.com/zTgx/lib/wiki/Getting-Started).


Contributing
------------

Please report bugs and make feature requests [here](https://github.com/zTgx/lib/issues).


------------
遗留问题:
- [x] 1，完善Amount数据结构
- [ ] 2，TakerGets和TakerPays在不同情况下的数据类型 （Amount or String）
- [ ] 3，新老服务器接口的更新（涉及到app/brokerage等相关字段）
- [x] 4，添加[[example]]
- [ ] 5，添加[[test]]
- [ ] 6，挂单接口中flags字段根据Sell/Buy的设置
- [ ] 7，代码架构调整
- [ ] 8，参数检查
- [ ] 9，异常处理
- [ ] 10，添加ed25519模块
- [ ] 11, 支持签名算法[ed25519/secp256k1]可配置
- [ ] 12, 交易类型【TX_TRANSACTION_TYPE】参数，根据上边代码赋值。
- [x] 13, tx_json对Amount，Account等String类型的序列化问题。
- [x] 14, 交易blob的签名。
- [ ] 15, 修改pub fn prepare(tx_json: TxJson)，===》 pub fn prepare(tx_json: &mut TxJson)，在原有基础上修改txjson。
- [ ] 16, 重构TypeObject compare方法。
- [x] 17, base/Amount 和 common/Amount映射！！！
- [x] 18, LocalSignX / TransactionX 
