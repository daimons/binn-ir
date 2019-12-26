// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use {
    std::{
        cmp::Ordering,
        mem,
    },

    binn_ir::{
        Result, Value,
        value,
    },
};

#[cfg(feature="std")]
use {
    std::io::{Cursor, ErrorKind},

    binn_ir::{Decoder, Encoder, IoResult, Map, Object, Size},
};

mod cmp;
use cmp::CmpTo;

impl CmpTo<i32> for u32 {

    fn cmp_to(&self, target: &i32) -> Ordering {
        match target < &0 {
            true => Ordering::Greater,
            false => self.cmp(&(*target as Self)),
        }
    }

}

impl CmpTo<u64> for u32 {

    fn cmp_to(&self, target: &u64) -> Ordering {
        (*self as u64).cmp(target)
    }

}

impl CmpTo<usize> for u64 {

    fn cmp_to(&self, target: &usize) -> Ordering {
        match mem::size_of::<Self>() >= mem::size_of::<usize>() {
            true => self.cmp(&(*target as Self)),
            false => (*self as usize).cmp(target),
        }
    }

}

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

    assert_eq!(value::MAX_DATA_SIZE.cmp_to(&i32::max_value()), Ordering::Equal);
    // There are some castings from data's length to u64, so run this test
    assert_ne!(value::MAX_DATA_SIZE.cmp_to(&u64::max_value()), Ordering::Greater);

    assert_eq!(value::OBJECT_KEY_MAX_LEN, u8::max_value() as usize);
}

#[test]
fn basic_type_sizes() -> Result<()> {
    assert_eq!(Value::Null.size()?, 1);
    assert_eq!(Value::True.size()?, 1);
    assert_eq!(Value::False.size()?, 1);
    assert_eq!(Value::U8(0).size()?, 2);
    assert_eq!(Value::I8(0).size()?, 2);
    assert_eq!(Value::U16(0).size()?, 3);
    assert_eq!(Value::I16(0).size()?, 3);
    assert_eq!(Value::U32(0).size()?, 5);
    assert_eq!(Value::I32(0).size()?, 5);
    assert_eq!(Value::Float(0.0).size()?, 5);
    assert_eq!(Value::U64(0).size()?, 9);
    assert_eq!(Value::I64(0).size()?, 9);
    assert_eq!(Value::Double(0.0).size()?, 9);

    Ok(())
}

#[test]
#[cfg(feature="std")]
fn basic_type_encoded_sizes() -> IoResult<()> {
    let mut buf = vec![];
    macro_rules! encode_and_verify { ($($call: expr, $size: expr,)+) => {{
        $(
            $call;
            assert_eq!(buf.len(), $size);
            buf.clear();
        )+
    }};}
    encode_and_verify!(
        buf.encode_null()?, 1,
        buf.encode_bool(true)?, 1,
        buf.encode_bool(false)?, 1,
        buf.encode_u8(0)?, 2,
        buf.encode_i8(0)?, 2,
        buf.encode_u16(0)?, 3,
        buf.encode_i16(0)?, 3,
        buf.encode_u32(0)?, 5,
        buf.encode_i32(0)?, 5,
        buf.encode_u64(0)?, 9,
        buf.encode_i64(0)?, 9,
        buf.encode_float(0.0)?, 5,
        buf.encode_double(0.0)?, 9,
    );

    Ok(())
}

#[test]
#[cfg(feature="std")]
fn basic_types() -> IoResult<()> {
    // Encode
    let mut buf = vec![];
    buf.encode_null()?;
    buf.encode_bool(true)?;
    buf.encode_bool(false)?;
    buf.encode_u8(123_u8)?;
    buf.encode_i8(-123_i8)?;
    buf.encode_u16(12345_u16)?;
    buf.encode_i16(-12345_i16)?;
    buf.encode_u32(123456789_u32)?;
    buf.encode_i32(-123456789_i32)?;
    buf.encode_float(123.0)?;
    buf.encode_float(-123.0)?;
    buf.encode_u64(98765432123_u64)?;
    buf.encode_i64(-98765432123_i64)?;
    buf.encode_double(0xAABB_CCDD_u64 as f64)?;
    buf.encode_double(-0xAABB_CCDD_i64 as f64)?;
    buf.encode_text(String::from("Mr. Reynholm"))?;
    buf.encode_text("hello-jen")?;
    buf.encode_date_time(String::from("hermione"))?;
    buf.encode_date("ron")?;
    buf.encode_time(String::from("harry"))?;
    buf.encode_decimal_str("ginny\t\0\n")?;

    let blob_strings = vec![
        "roy eats moss' orange".repeat(20),
        "moss kisses jen".repeat(30),
        "richmond is a ghost".repeat(40),
    ];
    for s in blob_strings.iter() {
        assert!(s.len() > i8::max_value() as usize);
        buf.encode_blob(s.as_bytes())?;
    }

    // Decode
    let mut cursor = Cursor::new(&buf);
    assert_eq!(cursor.decode_null()?, Some(()));
    assert_eq!(cursor.decode_bool()?, Some(true));
    assert_eq!(cursor.decode_bool()?, Some(false));
    assert_eq!(cursor.decode_u8()?, Some(123));
    assert_eq!(cursor.decode_i8()?, Some(-123));
    assert_eq!(cursor.decode_u16()?, Some(12345));
    assert_eq!(cursor.decode_i16()?, Some(-12345));
    assert_eq!(cursor.decode_u32()?, Some(123456789));
    assert_eq!(cursor.decode_i32()?, Some(-123456789));
    assert_eq!(cursor.decode_float()?, Some(123.0));
    assert_eq!(cursor.decode_float()?, Some(-123.0));
    assert_eq!(cursor.decode_u64()?, Some(98765432123));
    assert_eq!(cursor.decode_i64()?, Some(-98765432123));
    assert_eq!(cursor.decode_double()?, Some(0xAABB_CCDD_u64 as f64));
    assert_eq!(cursor.decode_double()?, Some(-0xAABB_CCDD_i64 as f64));
    assert_eq!(cursor.decode_text()?.unwrap(), "Mr. Reynholm");
    assert_eq!(cursor.decode_text()?.unwrap(), "hello-jen");
    assert_eq!(cursor.decode_date_time()?.unwrap(), "hermione");
    assert_eq!(cursor.decode_date()?.unwrap(), "ron");
    assert_eq!(cursor.decode_time()?.unwrap(), "harry");
    assert_eq!(cursor.decode_decimal_str()?.unwrap(), "ginny\t\0\n");

    for s in blob_strings.iter() {
        assert_eq!(cursor.decode_blob()?.unwrap(), s.as_bytes());
    }

    // Verify position
    assert_eq!(cursor.decode()?, None);
    assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);

    Ok(())
}

/// # Tries to decode from an invalid source, then unwraps an error
#[cfg(feature="std")]
macro_rules! decode_from_invalid_source { ($bytes: expr) => {{
    Cursor::new($bytes).decode().unwrap_err()
}};}

/// # Decodes from invalid source and asserts
///
/// If error kind is not provided, `ErrorKind::UnexpectedEof` will be used.
#[cfg(feature="std")]
macro_rules! decode_from_invalid_source_and_assert {
    ($bytes: expr, $error_kind: expr) => {{
        assert_eq!(decode_from_invalid_source!($bytes).kind(), $error_kind);
    }};
    ($bytes: expr) => {{
        decode_from_invalid_source_and_assert!($bytes, ErrorKind::UnexpectedEof);
    }};
}

#[test]
#[cfg(feature="std")]
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
#[cfg(feature="std")]
fn blobs() -> IoResult<()> {
    // Small blob with 4-byte size
    let buf = vec![
        value::BLOB,
        // Size: 10 bytes
        0x80, 0x00, 0x00, 0x0A,
        // Data
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    ];
    let mut cursor = Cursor::new(&buf);
    assert_eq!(cursor.decode_blob()?.unwrap(), [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    assert_eq!(cursor.decode_null()?, None);
    assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);

    // Small blob with 4-byte size; but data is missing
    match Cursor::new(vec![
        value::BLOB,
        // Size: 15 bytes
        0x80, 0x00, 0x00, 0x0F,
    ]).decode_blob().unwrap_err().into_inner() {
        Some(err) => assert_eq!(err.description().contains(binn_ir::TAG), true),
        None => panic!("value::Decoder::decode_blob() -> input was invalid; expected an inner error, got: None"),
    };

    Ok(())
}

#[test]
#[cfg(feature="std")]
fn lists() -> IoResult<()> {
    let list = Value::List(vec![
        Value::from(123_u8), Value::I16(-456), Value::U16(789), Value::Float(-123_f32), Value::Double(-789_f64),
        Value::from(String::from("Draco Malfoy")), Value::from("Slytherin"),
        Value::Time(String::from(std::u128::MAX.to_string().repeat(100))),
        Value::from(vec![Value::Date(String::from("July 12th, 2018")), Value::DecimalStr(String::from("1234567890"))]),
        Value::from({
            let mut map_data = Map::new();
            map_data.insert(0, Value::Null);
            map_data.insert(-1, Value::from(true));
            map_data.insert(2, Value::False);
            map_data.insert(-3, Value::from("Ravenclaw"));
            map_data.insert(4, Value::from(b"Hogwarts".to_vec()));
            map_data
        }),
    ]);
    let list_size = list.size()?;
    assert!(list_size > i8::max_value() as Size);

    let mut buf = vec![];
    list.encode(&mut buf)?;
    assert_eq!(list_size.cmp_to(&buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    match list {
        Value::List(list) => {
            assert_eq!(cursor.decode_list()?.unwrap(), list);
            println!("Verified: {:?}", &list);

            // Verify position
            assert_eq!(cursor.decode_map()?, None);
            assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };

    Ok(())
}

#[test]
#[cfg(feature="std")]
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
#[cfg(feature="std")]
fn maps() -> IoResult<()> {
    let map = Value::Map({
        let mut map = Map::new();
        map.insert(-1, Value::from("Mars"));
        map.insert(2, Value::List(vec![Value::I16(-12345), Value::U16(6789)]));
        map.insert(-3, Value::List(vec![Value::U16(6789), Value::I8(-89)]));
        map.insert(4, Value::Float(-12345_f32));
        map.insert(-5, Value::Double(6789_f64));
        map.insert(6, ().into());
        map.insert(-7, false.into());
        map.insert(8, true.into());
        map.insert(-9, "SUN".into());
        map.insert(10, String::from("earth").into());
        map.insert(-11, Value::from("Saturn"));
        map.insert(12, Value::from({
            let mut map = Map::new();
            map.insert(0, Value::from(()));
            map.insert(-1, Value::True);
            map.insert(2, Value::from(false));
            map.insert(-3, Value::from(vec![Value::from("Oracle"), Value::Blob(b"Universe, time and space".to_vec())]));
            map
        }));
        map
    });

    let mut buf = vec![];
    map.encode(&mut buf)?;

    assert_eq!(map.size()?.cmp_to(&buf.len()), Ordering::Equal);

    let mut cursor = Cursor::new(&buf);
    match map {
        Value::Map(map) => {
            assert_eq!(cursor.decode_map()?.unwrap(), map);
            println!("Verified: {:?}", &map);

            // Verify position
            assert_eq!(cursor.decode_object()?, None);
            assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };

    Ok(())
}

#[test]
#[cfg(feature="std")]
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
#[cfg(feature="std")]
fn objects() -> IoResult<()> {
    // Make a sample list from specification
    let list = Value::List(vec![
        Value::from({
            let mut map = Object::new();
            map.insert(String::from("id"), Value::U8(1));
            map.insert(String::from("name"), Value::from("John"));
            map
        }),
        Value::from({
            let mut map = Object::new();
            map.insert(String::from("id"), Value::U8(2));
            map.insert(String::from("name"), Value::from("Eric"));
            map
        }),
    ]);

    // Make an object
    let object = Value::from({
        let mut map = Object::new();
        map.insert(String::from("id"), Value::U64(999));
        map.insert(String::from("name"), Value::from("Moon"));
        map
    });

    // Encode
    let mut buf = vec![];
    list.encode(&mut buf)?;
    object.encode(&mut buf)?;

    // Decode
    let mut cursor = Cursor::new(&buf);
    match (list, object) {
        (Value::List(list), Value::Object(object)) => {
            assert_eq!(cursor.decode_list()?.unwrap(), list);
            println!("Verified: {:?}", &list);
            assert_eq!(cursor.decode_object()?.unwrap(), object);
            println!("Verified: {:?}", &object);

            // Verify position
            assert_eq!(cursor.decode_null()?, None);
            assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);
        },
        _ => unreachable!(),
    };

    Ok(())
}

#[test]
#[cfg(feature="std")]
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
