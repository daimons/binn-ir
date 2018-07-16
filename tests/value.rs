// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io::{Cursor, ErrorKind};

#[macro_use]
mod cmp_integers;

use binn_ir::value::{self, Value, Encoder, Decoder};

#[test]
fn constants() {
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
fn basic_type_lengths() {
    // Lengths
    assert_eq!(Value::Null.len().unwrap(), 1);
    assert_eq!(Value::True.len().unwrap(), 1);
    assert_eq!(Value::False.len().unwrap(), 1);
    assert_eq!(Value::U8(0).len().unwrap(), 2);
    assert_eq!(Value::I8(0).len().unwrap(), 2);
    assert_eq!(Value::U16(0).len().unwrap(), 3);
    assert_eq!(Value::I16(0).len().unwrap(), 3);
    assert_eq!(Value::U32(0).len().unwrap(), 5);
    assert_eq!(Value::I32(0).len().unwrap(), 5);
    assert_eq!(Value::Float(0.0).len().unwrap(), 5);
    assert_eq!(Value::U64(0).len().unwrap(), 9);
    assert_eq!(Value::I64(0).len().unwrap(), 9);
    assert_eq!(Value::Double(0.0).len().unwrap(), 9);

    // Encoded lengths
    let mut buf = vec![];
    macro_rules! encode_and_verify { ($($call: expr, $size: expr,)+) => {{
        $(
            $call;
            assert_eq!(buf.len(), $size);
            buf.clear();
        )+
    }};}
    encode_and_verify!(
        buf.encode_null().unwrap(), 1,
        buf.encode_bool(true).unwrap(), 1,
        buf.encode_bool(false).unwrap(), 1,
        buf.encode_u8(0_u8).unwrap(), 2,
        buf.encode_i8(0_i8).unwrap(), 2,
        buf.encode_u16(0_u16).unwrap(), 3,
        buf.encode_i16(0_i16).unwrap(), 3,
        buf.encode_u32(0_u32).unwrap(), 5,
        buf.encode_i32(0_i32).unwrap(), 5,
        buf.encode_u64(0_u64).unwrap(), 9,
        buf.encode_i64(0_i64).unwrap(), 9,
        buf.encode_float(0.0).unwrap(), 5,
        buf.encode_double(0.0).unwrap(), 9,
    );
}

#[test]
fn basic_types() {
    let mut buf = vec![];
    buf.encode_null().unwrap();
    buf.encode_bool(true).unwrap();
    buf.encode_bool(false).unwrap();
    buf.encode_u8(123_u8).unwrap();
    buf.encode_i8(-123_i8).unwrap();
    buf.encode_u16(12345_u16).unwrap();
    buf.encode_i16(-12345_i16).unwrap();
    buf.encode_u32(123456789_u32).unwrap();
    buf.encode_i32(-123456789_i32).unwrap();
    buf.encode_float(123.0).unwrap();
    buf.encode_float(-123.0).unwrap();
    buf.encode_u64(98765432123_u64).unwrap();
    buf.encode_i64(-98765432123_i64).unwrap();
    buf.encode_double(0xAABB_CCDD_u64 as f64).unwrap();
    buf.encode_double(-0xAABB_CCDD_i64 as f64).unwrap();
    buf.encode_text(String::from("Mr. Reynholm")).unwrap();
    buf.encode_text("hello-jen").unwrap();
    buf.encode_date_time(String::from("hermione")).unwrap();
    buf.encode_date("ron").unwrap();
    buf.encode_time(String::from("harry")).unwrap();
    buf.encode_decimal_str("ginny\t\0\n").unwrap();

    let blob_strings = vec![
        "roy eats moss' orange".repeat(20),
        "moss kisses jen".repeat(30),
        "richmond is a ghost".repeat(40),
    ];
    for s in blob_strings.iter() {
        assert_eq!(cmp_integers!(s.len(), std::i8::MAX), Ordering::Greater);
        buf.encode_blob(s.as_bytes()).unwrap();
    }

    let mut cursor = Cursor::new(&buf);
    assert_eq!(cursor.decode_null().unwrap(), Some(()));
    assert_eq!(cursor.decode_bool().unwrap(), Some(true));
    assert_eq!(cursor.decode_bool().unwrap(), Some(false));
    assert_eq!(cursor.decode_u8().unwrap(), Some(123));
    assert_eq!(cursor.decode_i8().unwrap(), Some(-123));
    assert_eq!(cursor.decode_u16().unwrap(), Some(12345));
    assert_eq!(cursor.decode_i16().unwrap(), Some(-12345));
    assert_eq!(cursor.decode_u32().unwrap(), Some(123456789));
    assert_eq!(cursor.decode_i32().unwrap(), Some(-123456789));
    assert_eq!(cursor.decode_float().unwrap(), Some(123.0));
    assert_eq!(cursor.decode_float().unwrap(), Some(-123.0));
    assert_eq!(cursor.decode_u64().unwrap(), Some(98765432123));
    assert_eq!(cursor.decode_i64().unwrap(), Some(-98765432123));
    assert_eq!(cursor.decode_double().unwrap(), Some(0xAABB_CCDD_u64 as f64));
    assert_eq!(cursor.decode_double().unwrap(), Some(-0xAABB_CCDD_i64 as f64));
    assert_eq!(cursor.decode_text().unwrap().unwrap(), "Mr. Reynholm");
    assert_eq!(cursor.decode_text().unwrap().unwrap(), "hello-jen");
    assert_eq!(cursor.decode_date_time().unwrap().unwrap(), "hermione");
    assert_eq!(cursor.decode_date().unwrap().unwrap(), "ron");
    assert_eq!(cursor.decode_time().unwrap().unwrap(), "harry");
    assert_eq!(cursor.decode_decimal_str().unwrap().unwrap(), "ginny\t\0\n");

    for s in blob_strings.iter() {
        assert_eq!(cursor.decode_blob().unwrap().unwrap(), s.as_bytes());
    }

    // Verify position
    assert_eq!(cursor.decode().unwrap(), None);
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
}

/// # Tries to decode from an invalid source, then unwraps an error
macro_rules! decode_from_invalid_source { ($bytes: expr) => {{
    Cursor::new($bytes).decode().unwrap_err()
}};}

/// # Decodes from invalid source and asserts
///
/// If error kind is not provided, `ErrorKind::UnexpectedEof` will be used.
macro_rules! decode_from_invalid_source_and_assert {
    ($bytes: expr, $error_kind: expr) => {{
        assert_eq!(decode_from_invalid_source!($bytes).kind(), $error_kind);
    }};
    ($bytes: expr) => {{
        decode_from_invalid_source_and_assert!($bytes, ErrorKind::UnexpectedEof);
    }};
}

#[test]
fn decode_basic_types_from_invalid_sources() {
    decode_from_invalid_source_and_assert!(vec![value::U8]);
    decode_from_invalid_source_and_assert!(vec![value::I8]);
    decode_from_invalid_source_and_assert!(vec![value::U16, 0]);
    decode_from_invalid_source_and_assert!(vec![value::I16, 0]);
    decode_from_invalid_source_and_assert!(vec![value::U32, 0, 1]);
    decode_from_invalid_source_and_assert!(vec![value::I32, 0, 1, 2]);
    decode_from_invalid_source_and_assert!(vec![value::U64, 0, 1, 2, 3, 4]);
    decode_from_invalid_source_and_assert!(vec![value::I64, 0, 1, 2, 3, 4, 5]);
    decode_from_invalid_source_and_assert!(vec![value::FLOAT, 0, 1]);
    decode_from_invalid_source_and_assert!(vec![value::DOUBLE, 0, 1, 2, 3, 4, 5, 6]);
}

#[test]
fn blobs() {
    // Small blob with 4-byte size
    let buf = vec![
        value::BLOB,
        // Size: 10 bytes
        0x80, 0x00, 0x00, 0x0A,
        // Data
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    ];
    let mut cursor = Cursor::new(&buf);
    assert_eq!(cursor.decode_blob().unwrap().unwrap(), [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    assert_eq!(cursor.decode_null().unwrap(), None);
    assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);

    // Small blob with 4-byte size; but data is missing
    match Cursor::new(vec![
        value::BLOB,
        // Size: 15 bytes
        0x80, 0x00, 0x00, 0x0F,
    ])
        .decode_blob().unwrap_err().into_inner()
    {
        Some(err) => assert_eq!(err.description().starts_with(binn_ir::TAG), true),
        None => panic!("value::Decoder::decode_blob() -> input was invalid; expected an inner error, got: None"),
    };
}

#[test]
fn lists() {
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
            assert_eq!(cursor.decode_list().unwrap().unwrap(), list);
            println!("Verified: {:?}", &list);

            // Verify position
            assert_eq!(cursor.decode_map().unwrap(), None);
            assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };
}

#[test]
fn decode_lists_from_invalid_sources() {
    // Missing size
    decode_from_invalid_source_and_assert!(vec![value::LIST]);
    // Invalid size
    decode_from_invalid_source_and_assert!(vec![value::LIST, 2], ErrorKind::InvalidData);
    // Invalid size
    decode_from_invalid_source_and_assert!(vec![value::LIST, 0x80, 0x00, 0x00, 0x02], ErrorKind::InvalidData);
    // Missing item count
    decode_from_invalid_source_and_assert!(vec![value::LIST, 0x80, 0x00, 0x00, 0x03]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::LIST, 3, 1]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::LIST, 0x80, 0x00, 0x00, 0x03, 1]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::LIST, 3, 0x80, 0x00, 0x00, 0x01]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::LIST, 0x80, 0x00, 0x00, 0x03, 0x80, 0x00, 0x00, 0x01]);
}

#[test]
fn maps() {
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
            assert_eq!(cursor.decode_map().unwrap().unwrap(), map);
            println!("Verified: {:?}", &map);

            // Verify position
            assert_eq!(cursor.decode_object().unwrap(), None);
            assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };
}

#[test]
fn decode_maps_from_invalid_sources() {
    // Missing size
    decode_from_invalid_source_and_assert!(vec![value::MAP]);
    // Invalid size
    decode_from_invalid_source_and_assert!(vec![value::MAP, 2], ErrorKind::InvalidData);
    // Invalid size
    decode_from_invalid_source_and_assert!(vec![value::MAP, 0x80, 0x00, 0x00, 0x02], ErrorKind::InvalidData);
    // Missing item count
    decode_from_invalid_source_and_assert!(vec![value::MAP, 0x80, 0x00, 0x00, 0x03]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::MAP, 3, 1]);
    // Invalid size + missing items
    decode_from_invalid_source_and_assert!(vec![value::MAP, 3, 0x80, 0x00, 0x00, 0x01]);
    // Invalid size + missing items
    decode_from_invalid_source_and_assert!(vec![value::MAP, 0x80, 0x00, 0x00, 0x03, 1]);
    // Invalid size + missing items
    decode_from_invalid_source_and_assert!(vec![value::MAP, 0x80, 0x00, 0x00, 0x03, 0x80, 0x00, 0x00, 0x01]);
}

#[test]
fn objects() {
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

    // Encode
    let mut buf = vec![];
    buf.encode(&list).unwrap();
    buf.encode(&object).unwrap();

    // Decode
    let mut cursor = Cursor::new(&buf);
    match (list, object) {
        (Value::List(list), Value::Object(object)) => {
            assert_eq!(cursor.decode_list().unwrap().unwrap(), list);
            println!("Verified: {:?}", &list);
            assert_eq!(cursor.decode_object().unwrap().unwrap(), object);
            println!("Verified: {:?}", &object);

            // Verify position
            assert_eq!(cursor.decode_null().unwrap(), None);
            assert_eq!(cmp_integers!(cursor.position(), buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };
}

#[test]
fn decode_objects_from_invalid_sources() {
    // Missing size
    decode_from_invalid_source_and_assert!(vec![value::OBJECT]);
    // Invalid size
    decode_from_invalid_source_and_assert!(vec![value::OBJECT, 2], ErrorKind::InvalidData);
    // Invalid size
    decode_from_invalid_source_and_assert!(vec![value::OBJECT, 0x80, 0x00, 0x00, 0x02], ErrorKind::InvalidData);
    // Missing item count
    decode_from_invalid_source_and_assert!(vec![value::OBJECT, 0x80, 0x00, 0x00, 0x03]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::OBJECT, 3, 1]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::OBJECT, 0x80, 0x00, 0x00, 0x03, 1]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::OBJECT, 3, 0x80, 0x00, 0x00, 0x01]);
    // Invalid size + missing items
    decode_from_invalid_source!(vec![value::OBJECT, 0x80, 0x00, 0x00, 0x03, 0x80, 0x00, 0x00, 0x01]);
}
