# misc-bintools: tiny tools
Bart Massey 2025

These are Rust rewrites of tiny tools that sit in my
personal bin and that I use regularly. The C source code for
some of them is semi-lost, and I'd prefer a solid Rust
implementation anyhow.

* `shuffle`: Shuffle lines of a file. Can be used as a
  filter.
* `ipaddr`: Look up IP address from name using local resolver.
* `ipname`: Look up name from IP address using local resolver.

## Installing

The easiest way to build these commands for installation is
`sh collect.sh`. This will build a `bin/` directory
containing compiled versions of the commands.

There is `Cargo.toml` metadata for building a Debian package
using `cargo-deb`.

## License

This work is made available under the "Apache 2.0 or MIT
License". See the file `LICENSE.txt` in this distribution for
license terms.
