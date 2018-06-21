// License: see LICENSE file at root directory of `master` branch

extern crate binnx;

use std::mem;

use binnx::value::{self, Value};

#[test]
fn values() {
    assert!(value::NULL         == 0b_0000_0000 | 0x00 | 0);
    assert!(value::TRUE         == 0b_0000_0001 | 0x01 | 1);
    assert!(value::FALSE        == 0b_0000_0010 | 0x02 | 2);

    assert!(value::U8           == 0b_0010_0000 | 0x20 | 32);
    assert!(value::I8           == 0b_0010_0001 | 0x21 | 33);

    assert!(value::U16          == 0b_0100_0000 | 0x40 | 64);
    assert!(value::I16          == 0b_0100_0001 | 0x41 | 65);

    assert!(value::U32          == 0b_0110_0000 | 0x60 | 96);
    assert!(value::I32          == 0b_0110_0001 | 0x61 | 97);
    assert!(value::FLOAT        == 0b_0110_0010 | 0x62 | 98);

    assert!(value::U64          == 0b_1000_0000 | 0x80 | 128);
    assert!(value::I64          == 0b_1000_0001 | 0x81 | 129);
    assert!(value::DOUBLE       == 0b_1000_0010 | 0x82 | 130);

    assert!(value::TEXT         == 0b_1010_0000 | 0xA0 | 160);
    assert!(value::DATE_TIME    == 0b_1010_0001 | 0xA1 | 161);
    assert!(value::DATE         == 0b_1010_0010 | 0xA2 | 162);
    assert!(value::TIME         == 0b_1010_0011 | 0xA3 | 163);
    assert!(value::DECIMAL_STR  == 0b_1010_0100 | 0xA4 | 164);

    assert!(value::BLOB         == 0b_1100_0000 | 0xC0 | 192);

    assert!(value::LIST         == 0b_1110_0000 | 0xE0 | 224);
    assert!(value::MAP          == 0b_1110_0001 | 0xE1 | 225);
    assert!(value::OBJECT       == 0b_1110_0010 | 0xE2 | 226);
}

#[test]
fn write() {
    let v = Value::U8(123);
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf == [value::U8, 123]);

    let v = Value::I16(-456);
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf == [value::I16, 0xFE, 0x38]);

    let v = Value::U16(789);
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf == [value::U16, 0x03, 0x15]);

    let v = Value::I16(-12345);
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf == [value::I16, 0xCF, 0xC7]);

    let v = Value::U16(6789);
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf == [value::U16, 0x1A, 0x85]);

    let v = Value::Text("Binn-X");
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf[0..2] == [value::TEXT, 0x06]);
    assert!(&buf[2..] == b"Binn-X\0");

    let v = Value::Blob(b"hello-jen");
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    assert!(buf[0..2] == [value::BLOB, 0x09]);
    assert!(&buf[2..] == b"hello-jen");

    let s = "roy eats moss' orange".repeat(100);
    let bytes = s.as_bytes();
    let v = Value::Blob(bytes);
    let mut buf = vec!();
    v.write(&mut buf).unwrap();
    let size: &[u8; mem::size_of::<i32>()] = unsafe { mem::transmute(&(bytes.len() as i32).to_be()) };
    assert!(buf[0] == value::BLOB);
    assert!(&buf[1..5] == size);
    assert!(&buf[5..] == bytes);
}
