
# deco-rs

[![Build status](https://travis-ci.org/anekos/deco-rs.svg?branch=master)](https://travis-ci.org/anekos/deco-rs)
[![crates.io](https://img.shields.io/crates/v/deco.svg)](https://crates.io/crates/deco)
[![Documentation](https://docs.rs/deco/badge.svg)](https://docs.rs/deco)

Easy rust macros to make terminal output colorful and decorative


# Usage

```
[dependencies]
deco = "*"
```

```rust
use std::fmt::Write;
use deco::*;

fn main() {
  tcprintln!([red bold "RED and BOLD TEXT"]);
  tcprintln!([red bold "RED and BOLD TEXT with argument `0x{:x}`"] 0xbeef);

  tcprintln!([red bold "RED and BOLD" reset "NORMAL"]);

  let mut out = "".to_owned();
  tcwriteln!(out, [italic "ITALIC TEXT"]).unwrap();
  println!("out is {}", out);
}
```
