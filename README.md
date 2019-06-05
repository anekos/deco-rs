
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
  dprintln!([red bold "RED and BOLD TEXT" !]);
  dprintln!([red bold "RED and BOLD TEXT with argument `0x{:x}`" !] 0xbeef);

  dprintln!([red bold "RED and BOLD" reset " ... NORMAL"]);

  dprintln!([yellow on_red "yellow on red" !]);

  let mut out = "".to_owned();
  dwriteln!(out, [italic "ITALIC TEXT" !]).unwrap();
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
| `on_red`       | background red                  |
| `on_black`     | background black                |
| `on_green`     | background green                |
| `on_yellow`    | background yellow               |
| `on_blue`      | background blue                 |
| `on_magenta`   | background magenta              |
| `on_cyan`      | background cyan                 |
| `on_white`     | background white                |
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
