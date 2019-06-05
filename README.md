
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
  dprintln!([red bold "RED and BOLD TEXT"]);
  dprintln!([red bold "RED and BOLD TEXT with argument `0x{:x}`"] 0xbeef);

  dprintln!([red bold "RED and BOLD" reset " ... NORMAL"]);

  let mut out = "".to_owned();
  dwriteln!(out, [italic "ITALIC TEXT"]).unwrap();
  println!("out is {}", out);
}
```

## Color/Decoration specifiers

| Specifier      | Description                     |
|----------------|---------------------------------|
| `red`          | foreground red                  |
| `black`        | foreground black                |
| `green`        | foreground green                |
| `yellow`       | foreground yellow               |
| `blue`         | foreground blue                 |
| `magenta`      | foreground magenta              |
| `cyan`         | foreground cyan                 |
| `white`        | foreground white                |
| `Red`          | background red                  |
| `Black`        | background black                |
| `Green`        | background green                |
| `Yellow`       | background yellow               |
| `Blue`         | background blue                 |
| `Magenta`      | background magenta              |
| `Cyan`         | background cyan                 |
| `White`        | background white                |
| `italic`       |                                 |
| `bold`         |                                 |
| `underscore`   |                                 |
| `blink`        |                                 |
| `reverse`      |                                 |
| `conceal`      |                                 |
| `reset` or `!` | Reset color and all decorations |



# Macros

This crate has

- dformat
- dprint
- dprintln
- dwrite
- dwriteln
