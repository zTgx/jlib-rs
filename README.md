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

**[Homepage](https://github.com/zTgx/lib/wiki)**

**[API Documentation](https://github.com/zTgx/lib/wiki/API-Documentation)**



Getting Started
---------------

For detailed installation and usage instructions, check out the [guide](https://github.com/zTgx/lib/wiki/Getting-Started).


Contributing
------------

Please report bugs and make feature requests [here](https://github.com/zTgx/lib/issues).


