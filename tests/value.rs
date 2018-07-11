// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;

#[macro_use]
mod cmp_integers;

use binn_ir::value::{self, Value};

#[test]
fn values() {
    assert_eq!(value::NULL,         0b_0000_0000 | 0x00 | 0);
    assert_eq!(value::TRUE,         0b_0000_0001 | 0x01 | 1);
    assert_eq!(value::FALSE,        0b_0000_0010 | 0x02 | 2);

    assert_eq!(value::U8,           0b_0010_0000 | 0x20 | 32);
    assert_eq!(value::I8,           0b_0010_0001 | 0x21 | 33);

    assert_eq!(value::U16,          0b_0100_0000 | 0x40 | 64);
    assert_eq!(value::I16,          0b_0100_0001 | 0x41 | 65);

    assert_eq!(value::U32,          0b_0110_0000 | 0x60 | 96);
    assert_eq!(value::I32,          0b_0110_0001 | 0x61 | 97);
    assert_eq!(value::FLOAT,        0b_0110_0010 | 0x62 | 98);

    assert_eq!(value::U64,          0b_1000_0000 | 0x80 | 128);
    assert_eq!(value::I64,          0b_1000_0001 | 0x81 | 129);
    assert_eq!(value::DOUBLE,       0b_1000_0010 | 0x82 | 130);

    assert_eq!(value::TEXT,         0b_1010_0000 | 0xA0 | 160);
    assert_eq!(value::DATE_TIME,    0b_1010_0001 | 0xA1 | 161);
    assert_eq!(value::DATE,         0b_1010_0010 | 0xA2 | 162);
    assert_eq!(value::TIME,         0b_1010_0011 | 0xA3 | 163);
    assert_eq!(value::DECIMAL_STR,  0b_1010_0100 | 0xA4 | 164);

    assert_eq!(value::BLOB,         0b_1100_0000 | 0xC0 | 192);

    assert_eq!(value::LIST,         0b_1110_0000 | 0xE0 | 224);
    assert_eq!(value::MAP,          0b_1110_0001 | 0xE1 | 225);
    assert_eq!(value::OBJECT,       0b_1110_0010 | 0xE2 | 226);

    assert_eq!(cmp_integers!(Value::MAX_DATA_SIZE, std::u64::MAX), Ordering::Less);
}

#[test]
fn read_write_basic_types() {
    let mut buf = vec![];
    Value::Null.write(&mut buf).unwrap();
    Value::True.write(&mut buf).unwrap();
    Value::False.write(&mut buf).unwrap();
    Value::U8(123).write(&mut buf).unwrap();
    Value::I8(-123).write(&mut buf).unwrap();
    Value::U16(12345).write(&mut buf).unwrap();
    Value::I16(-12345).write(&mut buf).unwrap();
    Value::U32(123456789).write(&mut buf).unwrap();
    Value::I32(-123456789).write(&mut buf).unwrap();
    Value::Float(123.0).write(&mut buf).unwrap();
    Value::Float(-123.0).write(&mut buf).unwrap();
    Value::U64(98765432123).write(&mut buf).unwrap();
    Value::I64(-98765432123).write(&mut buf).unwrap();
    Value::Double(0xAABB_CCDD_u64 as f64).write(&mut buf).unwrap();
    Value::Double(-0xAABB_CCDD_i64 as f64).write(&mut buf).unwrap();
    Value::Text(String::from("Mr. Reynholm")).write(&mut buf).unwrap();
    Value::Text(String::from("hello-jen")).write(&mut buf).unwrap();
    Value::DateTime(String::from("hermione")).write(&mut buf).unwrap();
    Value::Date(String::from("ron")).write(&mut buf).unwrap();
    Value::Time(String::from("harry")).write(&mut buf).unwrap();
    Value::DecimalStr(String::from("ginny\t\0\n")).write(&mut buf).unwrap();

    let blob_str = "roy eats moss' orange".repeat(20);
    assert_eq!(cmp_integers!(blob_str.len(), std::i8::MAX), Ordering::Greater);
    Value::Blob(blob_str.as_bytes().to_vec()).write(&mut buf).unwrap();

    let mut cursor = Cursor::new(&buf);
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Null);
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::True);
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::False);
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::U8(123));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::I8(-123));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::U16(12345));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::I16(-12345));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::U32(123456789));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::I32(-123456789));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Float(123.0));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Float(-123.0));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::U64(98765432123));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::I64(-98765432123));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Double(0xAABB_CCDD_u64 as f64));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Double(-0xAABB_CCDD_i64 as f64));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Text(String::from("Mr. Reynholm")));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Text(String::from("hello-jen")));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::DateTime(String::from("hermione")));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Date(String::from("ron")));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::Time(String::from("harry")));
    assert_eq!(Value::read(&mut cursor).unwrap(), Value::DecimalStr(String::from("ginny\t\0\n")));

    match Value::read(&mut cursor) {
        // Ok(Value::Blob(bytes)) => assert_eq!(String::from_utf8(bytes).unwrap(), blob_str),
        Ok(Value::Blob(_)) => { assert_eq!(cursor.position(), buf.len() as u64); },
        Ok(other) => panic!("Expected a blob, got: {}", &other),
        Err(err) => panic!("Expected a blob, got: {}", &err),
    };

    // Verify position
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
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
    map.insert(1, Value::Text(String::from("add")));
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
    map.insert(String::from("id"), Value::U8(1));
    map.insert(String::from("name"), Value::Text(String::from("John")));
    let object = Value::Object(map);
    let object1_len = object.len().unwrap();
    list.push(object);

    let mut map = HashMap::new();
    map.insert(String::from("id"), Value::U8(2));
    map.insert(String::from("name"), Value::Text(String::from("Eric")));
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
