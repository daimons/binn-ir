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

    let blob_strings = vec![
        "roy eats moss' orange".repeat(20),
        "moss kisses jen".repeat(30),
        "richmond is a ghost".repeat(40),
    ];
    for s in blob_strings.iter() {
        assert_eq!(cmp_integers!(s.len(), std::i8::MAX), Ordering::Greater);
        Value::Blob(s.as_bytes().to_vec()).write(&mut buf).unwrap();
    }

    let mut cursor = Cursor::new(&buf);
    Value::read_null(&mut cursor).unwrap();
    assert_eq!(Value::read_bool(&mut cursor).unwrap(), true);
    assert_eq!(Value::read_bool(&mut cursor).unwrap(), false);
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

    for s in blob_strings.iter() {
        match Value::read(&mut cursor) {
            Ok(Value::Blob(bytes)) => assert_eq!(bytes.as_slice(), s.as_bytes()),
            Ok(other) => panic!("Expected a blob, got: {}", &other),
            Err(err) => panic!("Expected a blob, got: {}", &err),
        };
    }

    // Verify position
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
}

#[test]
fn read_write_lists() {
    let list = Value::List(vec![
        Value::U8(123), Value::I16(-456), Value::U16(789), Value::Float(-123_f32), Value::Double(-789_f64),
        Value::Text(String::from("Draco Malfoy")), Value::Text(String::from("Slytherin")),
        Value::Time(String::from(std::u128::MAX.to_string().repeat(100))),
        Value::List(vec![Value::Date(String::from("July 12th, 2018")), Value::DecimalStr(String::from("1234567890"))]),
        Value::Map({
            let mut map_data = BTreeMap::new();
            map_data.insert(0, Value::Null);
            map_data.insert(-1, Value::True);
            map_data.insert(2, Value::False);
            map_data.insert(-3, Value::Text(String::from("Ravenclaw")));
            map_data.insert(4, Value::Blob(b"Hogwarts".to_vec()));
            map_data
        }),
    ]);
    let list_len = list.len().unwrap();
    assert_eq!(cmp_integers!(list_len, std::i8::MAX), Ordering::Greater);

    let mut buf = vec![];
    list.write(&mut buf).unwrap();
    assert_eq!(cmp_integers!(list_len, buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    assert_eq!(Value::read(&mut cursor).unwrap(), list);

    // Verify position
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
}

#[test]
fn read_write_maps() {
    let map = Value::Map({
        let mut map_data = BTreeMap::new();
        map_data.insert(-1, Value::Text(String::from("Mars")));
        map_data.insert(2, Value::List(vec![Value::I16(-12345), Value::U16(6789)]));
        map_data.insert(-3, Value::List(vec![Value::U16(6789), Value::I8(-89)]));
        map_data.insert(4, Value::Float(-12345_f32));
        map_data.insert(-5, Value::Double(6789_f64));
        map_data.insert(6, Value::Null);
        map_data.insert(-7, Value::False);
        map_data.insert(8, Value::True);
        map_data.insert(-9, Value::Text(String::from("SUN")));
        map_data.insert(10, Value::Text(String::from("earth")));
        map_data.insert(-11, Value::Text(String::from("Saturn")));
        map_data.insert(-12, Value::Map({
            let mut map_data = BTreeMap::new();
            map_data.insert(0, Value::Null);
            map_data.insert(-1, Value::True);
            map_data.insert(2, Value::False);
            map_data.insert(-3, Value::List(vec![Value::Text(String::from("Oracle")), Value::Blob(b"Universe, time and space".to_vec())]));
            map_data
        }));
        map_data
    });

    let mut buf = vec![];
    map.write(&mut buf).unwrap();

    assert_eq!(cmp_integers!(map.len().unwrap(), buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    assert_eq!(Value::read(&mut cursor).unwrap(), map);

    // Verify position
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
}

#[test]
fn read_write_objects() {
    let list = Value::List(vec![
        Value::Object({
            let mut map = HashMap::new();
            map.insert(String::from("id"), Value::U8(1));
            map.insert(String::from("name"), Value::Text(String::from("John")));
            map
        }),
        Value::Object({
            let mut map = HashMap::new();
            map.insert(String::from("id"), Value::U8(2));
            map.insert(String::from("name"), Value::Text(String::from("Eric")));
            map
        }),
    ]);

    let mut buf = vec![];
    list.write(&mut buf).unwrap();

    assert_eq!(cmp_integers!(list.len().unwrap(), buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    assert_eq!(Value::read(&mut cursor).unwrap(), list);

    // Verify position
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
}
