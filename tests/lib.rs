
use std::fmt::Write;

use deco::{tcformat, tcprint, tcprintln, tcwrite, tcwriteln};



#[test]
fn test_write() {
    let mut out = "".to_owned();
    tcwrite!(out, [red blink "red"]).unwrap();
    assert_eq!(&out, "\x1b[31m\x1b[5mred");

    let mut out = "".to_owned();
    tcwrite!(out, [red "red" bold "0x{:x}" !] 0xbeef).unwrap();
    assert_eq!(&out, "\x1b[31mred\x1b[1m0xbeef\x1b[0m");
}

#[test]
fn test_writeln() {
    let mut out = "".to_owned();
    tcwriteln!(out, [red blink "red"]).unwrap();
    assert_eq!(&out, "\x1b[31m\x1b[5mred\n");

    let mut out = "".to_owned();
    tcwriteln!(out, [red "red" bold "0x{:x}" !] 0xbeef).unwrap();
    assert_eq!(&out, "\x1b[31mred\x1b[1m0xbeef\x1b[0m\n");
}

#[test]
fn test_format() {
    assert_eq!(
        &tcformat!([red blink "red"]),
        "\x1b[31m\x1b[5mred");

    assert_eq!(
        &tcformat!([red "red" bold "0x{:x}" !] 0xbeef),
        "\x1b[31mred\x1b[1m0xbeef\x1b[0m");
}

#[test]
fn test_print() {
    tcprint!([red blink "red"]);
    tcprint!([red "red" bold "0x{:x}" !] 0xbeef);
}

#[test]
fn test_println() {
    tcprintln!([red blink "red"]);
    tcprintln!([red "red" bold "0x{:x}" !] 0xbeef);
}
