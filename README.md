# [jlib-rs](https://github.com/zTgx/jlib-rs) [![Build Status](https://travis-ci.org/zTgx/jlib-rs.svg?branch=master)](https://travis-ci.org/zTgx/jlib-rs) [![crate](https://img.shields.io/crates/v/jlib.svg)](https://crates.io/crates/jlib)

Lightweight blockchain lib for Skywelld write in [Rust](http://www.rust-lang.org).
```rust

/// Request blockchain server status
let config = Config::new(TEST_SERVER, true);
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

**[Homepage](https://github.com/zTgx/jlib-rs/wiki)**

**[API Documentation](https://github.com/zTgx/jlib-rs/wiki/API-Documentation)**


Getting Started
---------------

For detailed installation and usage instructions, check out the [guide](https://github.com/zTgx/jlib-rs/wiki/Getting-Started).  


More examples
---------------
More examples can be found [in the examples directory](examples/) and the full documentation can be [found here](https://github.com/zTgx/jlib-rs/wiki).  


Contributing
------------

Please report bugs and make feature requests [here](https://github.com/zTgx/jlib-rs/issues).

If you're looking for inspiration, there's list of [open issues](https://github.com/zTgx/jlib-rs/issues?state=open) right here on github.

And hey, did you know you can also contribute by just starring the project here on github :)


## Changelog

Release notes are available in [CHANAGELOG.md](CHANAGELOG.md).
