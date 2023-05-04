# [jlib-rs](https://github.com/zTgx/jlib-rs) [![crate](https://img.shields.io/crates/v/jlib.svg)](https://crates.io/crates/jlib) ![](https://tokei.rs/b1/github/zTgx/jlib-rs)

Lightweight blockchain lib for Skywelld write in [Rust](http://www.rust-lang.org).
```rust
    let config = Config::new(TEST_SERVER, true);

    request(config, |x| match x {
        Ok(response) => {
            println!("build_version : {:?}", response.build_version);
        },
        Err(_) => {
            println! ("error occured.");
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

Release notes are available in [CHANGELOG.md](CHANGELOG.md).
