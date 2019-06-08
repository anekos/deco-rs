
use std::fmt::Write;

use deco::{dformat, dprint, dprintln, deprint, deprintln, dwrite, dwriteln};



#[test]
fn test_write() {
    let mut out = "".to_owned();
    dwrite!(out, [red blink "red"]).unwrap();
    assert_eq!(&out, "\x1b[31m\x1b[5mred");

    let mut out = "".to_owned();
    dwrite!(out, [red "red" bold "0x{:x}" !] 0xbeef).unwrap();
    assert_eq!(&out, "\x1b[31mred\x1b[1m0xbeef\x1b[0m");
}

#[test]
fn test_writeln() {
    let mut out = "".to_owned();
    dwriteln!(out, [red blink "red"]).unwrap();
    assert_eq!(&out, "\x1b[31m\x1b[5mred\n");

    let mut out = "".to_owned();
    dwriteln!(out, [red "red" bold "0x{:x}" !] 0xbeef).unwrap();
    assert_eq!(&out, "\x1b[31mred\x1b[1m0xbeef\x1b[0m\n");
}

#[test]
fn test_format() {
    assert_eq!(
        &dformat!([red blink "red"]),
        "\x1b[31m\x1b[5mred");

    assert_eq!(
        &dformat!([red "red" bold "0x{:x}" !] 0xbeef),
        "\x1b[31mred\x1b[1m0xbeef\x1b[0m");
}

#[test]
fn test_print() {
    dprint!([red blink "red"]);
    dprint!([red "red" bold "0x{:x}" !] 0xbeef);
}

#[test]
fn test_println() {
    dprintln!([red blink "red"]);
    dprintln!([red "red" bold "0x{:x}" !] 0xbeef);
}

#[test]
fn test_eprint() {
    deprint!([red blink "red"]);
    deprint!([red "red" bold "0x{:x}" !] 0xbeef);
}

#[test]
fn test_eprintln() {
    deprintln!([red blink "red"]);
    deprintln!([red "red" bold "0x{:x}" !] 0xbeef);
}
