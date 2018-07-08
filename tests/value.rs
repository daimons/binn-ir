// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use std::collections::{BTreeMap, HashMap};
use std::mem;

use binn_ir::value::{self, Value};

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
fn write_basic_types() {
    let v = Value::U8(123);
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf, [value::U8, 123]);

    let v = Value::I16(-456);
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf, [value::I16, 0xFE, 0x38]);

    let v = Value::U16(789);
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf, [value::U16, 0x03, 0x15]);

    let v = Value::I16(-12345);
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf, [value::I16, 0xCF, 0xC7]);

    let v = Value::U16(6789);
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf, [value::U16, 0x1A, 0x85]);

    let v = Value::Text("Binn-X");
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf[0..2], [value::TEXT, 0x06]);
    assert_eq!(&buf[2..], b"Binn-X\0");

    let v = Value::Blob(b"hello-jen");
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf[0..2], [value::BLOB, 0x09]);
    assert_eq!(&buf[2..], b"hello-jen");

    let s = "roy eats moss' orange".repeat(100);
    let bytes = s.as_bytes();
    let v = Value::Blob(bytes);
    let mut buf = vec![];
    v.write(&mut buf).unwrap();
    assert_eq!(buf[0], value::BLOB);
    assert_eq!(&buf[1..5], unsafe { mem::transmute::<i32, [u8; mem::size_of::<i32>()]>((bytes.len() as i32).to_be()) });
    assert_eq!(&buf[5..], bytes);
}

#[test]
fn write_lists() {
    let value = Value::List(vec![Value::U8(123), Value::I16(-456), Value::U16(789)]);
    let mut buf = vec![];
    value.write(&mut buf).unwrap();
    println!("Expected {} bytes; got: {} -> {:02x?}", value.len().unwrap(), buf.len(), buf.as_slice());
    assert_eq!(buf.as_slice(), &[
        // Type
        value::LIST,
        // Size
        buf.len() as u8,
        // Count
        3,
        value::U8, 123,
        value::I16, 0xFE, 0x38,
        value::U16, 0x03, 0x15
    ]);
}

#[test]
fn write_maps() {
    let mut map = BTreeMap::new();
    map.insert(1, Value::Text("add"));
    map.insert(2, Value::List(vec![Value::I16(-12345), Value::U16(6789)]));

    let item_count = map.len();

    let value = Value::Map(map);
    let mut buf = vec![];
    value.write(&mut buf).unwrap();
    println!("Expected {} bytes; got: {} -> {:02x?}", value.len().unwrap(), buf.len(), buf.as_slice());
    assert_eq!(buf.as_slice(), &[
        // Type
        value::MAP,
        // Size
        buf.len() as u8,
        // Count
        item_count as u8,
        // Key: 1
        0x00, 0x00, 0x00, 0x01,
        value::TEXT, 0x03, b'a', b'd', b'd', 0x00,
        // Key: 2
        0x00, 0x00, 0x00, 0x02,
        value::LIST, 0x09, 0x02,
        value::I16, 0xCF, 0xC7,
        value::U16, 0x1A, 0x85,
    ]);
}

#[test]
fn write_objects() {
    let mut list = vec![];

    let mut map = HashMap::new();
    map.insert("id", Value::U8(1));
    map.insert("name", Value::Text("John"));
    let object = Value::Object(map);
    let object1_len = object.len().unwrap();
    list.push(object);

    let mut map = HashMap::new();
    map.insert("id", Value::U8(2));
    map.insert("name", Value::Text("Eric"));
    let object = Value::Object(map);
    let object2_len = object.len().unwrap();
    list.push(object);

    let item_count = list.len();

    let value = Value::List(list);
    let mut buf = vec![];
    value.write(&mut buf).unwrap();
    println!("Expected {} bytes; got: {} -> {:02x?}", value.len().unwrap(), buf.len(), buf.as_slice());
    assert_eq!(&buf[0..6], &[
        // Type
        value::LIST,
        // Size
        buf.len() as u8,
        // Count
        item_count as u8,

        value::OBJECT, object1_len as u8, 2,
    ]);
    assert!((
        &buf[6..23] == &[
            0x02, b'i', b'd', value::U8, 1,
            0x04, b'n', b'a', b'm', b'e', value::TEXT, 4, b'J', b'o', b'h', b'n', 0x00,
        ]
        || &buf[6..23] == &[
            0x04, b'n', b'a', b'm', b'e', value::TEXT, 4, b'J', b'o', b'h', b'n', 0x00,
            0x02, b'i', b'd', value::U8, 1,
        ]
    ));
    assert_eq!(&buf[23..26], &[value::OBJECT, object2_len as u8, 2]);
    assert!(
        &buf[26..43] == &[
            0x02, b'i', b'd', value::U8, 2,
            0x04, b'n', b'a', b'm', b'e', value::TEXT, 4, b'E', b'r', b'i', b'c', 0x00,
        ]
        || &buf[26..43] == &[
            0x04, b'n', b'a', b'm', b'e', value::TEXT, 4, b'E', b'r', b'i', b'c', 0x00,
            0x02, b'i', b'd', value::U8, 2,
        ]
    );
}
