// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;

#[macro_use]
mod cmp_integers;

use binn_ir::value::{self, Value, Encoder, Decoder};

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

    assert_eq!(cmp_integers!(value::MAX_DATA_SIZE, std::u64::MAX), Ordering::Less);
}

#[test]
fn read_write_basic_types() {
    let mut buf = vec![];
    buf.encode(Value::Null).unwrap();
    buf.encode(Value::True).unwrap();
    buf.encode(Value::from(false)).unwrap();
    buf.encode(Value::U8(123)).unwrap();
    buf.encode(Value::from(-123_i8)).unwrap();
    buf.encode(Value::U16(12345)).unwrap();
    buf.encode(Value::from(-12345_i16)).unwrap();
    buf.encode(Value::U32(123456789)).unwrap();
    buf.encode(Value::from(-123456789_i32)).unwrap();
    buf.encode(Value::Float(123.0)).unwrap();
    buf.encode(Value::from(-123.0_f32)).unwrap();
    buf.encode(Value::U64(98765432123)).unwrap();
    buf.encode(Value::from(-98765432123_i64)).unwrap();
    buf.encode(Value::Double(0xAABB_CCDD_u64 as f64)).unwrap();
    buf.encode(Value::from(-0xAABB_CCDD_i64 as f64)).unwrap();
    buf.encode(Value::Text(String::from("Mr. Reynholm"))).unwrap();
    buf.encode(Value::from("hello-jen")).unwrap();
    buf.encode(Value::DateTime(String::from("hermione"))).unwrap();
    buf.encode(Value::Date(String::from("ron"))).unwrap();
    buf.encode(Value::Time(String::from("harry"))).unwrap();
    buf.encode(Value::DecimalStr(String::from("ginny\t\0\n"))).unwrap();

    let blob_strings = vec![
        "roy eats moss' orange".repeat(20),
        "moss kisses jen".repeat(30),
        "richmond is a ghost".repeat(40),
    ];
    for s in blob_strings.iter() {
        assert_eq!(cmp_integers!(s.len(), std::i8::MAX), Ordering::Greater);
        buf.encode(Value::Blob(s.as_bytes().to_vec())).unwrap();
    }

    let mut cursor = Cursor::new(&buf);
    cursor.decode_null().unwrap();
    assert_eq!(cursor.decode_bool().unwrap(), true);
    assert_eq!(cursor.decode_bool().unwrap(), false);
    assert_eq!(cursor.decode_u8().unwrap(), 123);
    assert_eq!(cursor.decode_i8().unwrap(), -123);
    assert_eq!(cursor.decode_u16().unwrap(), 12345);
    assert_eq!(cursor.decode_i16().unwrap(), -12345);
    assert_eq!(cursor.decode_u32().unwrap(), 123456789);
    assert_eq!(cursor.decode_i32().unwrap(), -123456789);
    assert_eq!(cursor.decode_float().unwrap(), 123.0);
    assert_eq!(cursor.decode_float().unwrap(), -123.0);
    assert_eq!(cursor.decode_u64().unwrap(), 98765432123);
    assert_eq!(cursor.decode_i64().unwrap(), -98765432123);
    assert_eq!(cursor.decode_double().unwrap(), 0xAABB_CCDD_u64 as f64);
    assert_eq!(cursor.decode_double().unwrap(), -0xAABB_CCDD_i64 as f64);
    assert_eq!(cursor.decode_text().unwrap(), "Mr. Reynholm");
    assert_eq!(cursor.decode_text().unwrap(), "hello-jen");
    assert_eq!(cursor.decode_date_time().unwrap(), "hermione");
    assert_eq!(cursor.decode_date().unwrap(), "ron");
    assert_eq!(cursor.decode_time().unwrap(), "harry");
    assert_eq!(cursor.decode_decimal_str().unwrap(), "ginny\t\0\n");

    for s in blob_strings.iter() {
        assert_eq!(cursor.decode_blob().unwrap(), s.as_bytes());
    }

    // Verify position
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
}

#[test]
fn read_write_lists() {
    let list = Value::List(vec![
        Value::from(123_u8), Value::I16(-456), Value::U16(789), Value::Float(-123_f32), Value::Double(-789_f64),
        Value::from(String::from("Draco Malfoy")), Value::from("Slytherin"),
        Value::Time(String::from(std::u128::MAX.to_string().repeat(100))),
        Value::from(vec![Value::Date(String::from("July 12th, 2018")), Value::DecimalStr(String::from("1234567890"))]),
        Value::from({
            let mut map_data = BTreeMap::new();
            map_data.insert(0, Value::Null);
            map_data.insert(-1, Value::from(true));
            map_data.insert(2, Value::False);
            map_data.insert(-3, Value::from("Ravenclaw"));
            map_data.insert(4, Value::from(b"Hogwarts".to_vec()));
            map_data
        }),
    ]);
    let list_len = list.len().unwrap();
    assert_eq!(cmp_integers!(list_len, std::i8::MAX), Ordering::Greater);

    let mut buf = vec![];
    buf.encode(&list).unwrap();
    assert_eq!(cmp_integers!(list_len, buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    match list {
        Value::List(list) => {
            assert_eq!(cursor.decode_list().unwrap(), list);
            println!("Verified: {:?}", &list);

            // Verify position
            assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };
}

#[test]
fn read_write_maps() {
    let map = Value::Map({
        let mut map_data = BTreeMap::new();
        map_data.insert(-1, Value::from("Mars"));
        map_data.insert(2, Value::List(vec![Value::I16(-12345), Value::U16(6789)]));
        map_data.insert(-3, Value::List(vec![Value::U16(6789), Value::I8(-89)]));
        map_data.insert(4, Value::Float(-12345_f32));
        map_data.insert(-5, Value::Double(6789_f64));
        map_data.insert(6, ().into());
        map_data.insert(-7, false.into());
        map_data.insert(8, true.into());
        map_data.insert(-9, "SUN".into());
        map_data.insert(10, String::from("earth").into());
        map_data.insert(-11, Value::from("Saturn"));
        map_data.insert(12, Value::from({
            let mut map_data = BTreeMap::new();
            map_data.insert(0, Value::from(()));
            map_data.insert(-1, Value::True);
            map_data.insert(2, Value::from(false));
            map_data.insert(-3, Value::from(vec![Value::from("Oracle"), Value::Blob(b"Universe, time and space".to_vec())]));
            map_data
        }));
        map_data
    });

    let mut buf = vec![];
    buf.encode(&map).unwrap();

    assert_eq!(cmp_integers!(map.len().unwrap(), buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    match map {
        Value::Map(map) => {
            assert_eq!(cursor.decode_map().unwrap(), map);
            println!("Verified: {:?}", &map);

            // Verify position
            assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };
}

#[test]
fn read_write_objects() {
    // Make a sample list from specification
    let list = Value::List(vec![
        Value::from({
            let mut map = HashMap::new();
            map.insert(String::from("id"), Value::U8(1));
            map.insert(String::from("name"), Value::from("John"));
            map
        }),
        Value::from({
            let mut map = HashMap::new();
            map.insert(String::from("id"), Value::U8(2));
            map.insert(String::from("name"), Value::from("Eric"));
            map
        }),
    ]);

    // Make an object
    let object = Value::from({
        let mut map = HashMap::new();
        map.insert(String::from("id"), Value::U64(999));
        map.insert(String::from("name"), Value::from("Moon"));
        map
    });

    // Write
    let mut buf = vec![];
    buf.encode(&list).unwrap();
    buf.encode(&object).unwrap();

    // Read
    let mut cursor = Cursor::new(&buf);
    match (list, object) {
        (Value::List(list), Value::Object(object)) => {
            assert_eq!(cursor.decode_list().unwrap(), list);
            println!("Verified: {:?}", &list);
            assert_eq!(cursor.decode_object().unwrap(), object);
            println!("Verified: {:?}", &object);

            // Verify position
            assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };
}
