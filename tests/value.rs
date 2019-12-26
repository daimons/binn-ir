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

    binn_ir::{IoResult, Map, Object, Size},
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
        binn_ir::encode_null(&mut buf, )?, 1,
        binn_ir::encode_bool(&mut buf, true)?, 1,
        binn_ir::encode_bool(&mut buf, false)?, 1,
        binn_ir::encode_u8(&mut buf, 0_u8)?, 2,
        binn_ir::encode_i8(&mut buf, 0_i8)?, 2,
        binn_ir::encode_u16(&mut buf, 0_u16)?, 3,
        binn_ir::encode_i16(&mut buf, 0_i16)?, 3,
        binn_ir::encode_u32(&mut buf, 0_u32)?, 5,
        binn_ir::encode_i32(&mut buf, 0_i32)?, 5,
        binn_ir::encode_u64(&mut buf, 0_u64)?, 9,
        binn_ir::encode_i64(&mut buf, 0_i64)?, 9,
        binn_ir::encode_float(&mut buf, 0.0)?, 5,
        binn_ir::encode_double(&mut buf, 0.0)?, 9,
    );

    Ok(())
}

#[test]
#[cfg(feature="std")]
fn basic_types() -> IoResult<()> {
    // Encode
    let mut buf = vec![];
    binn_ir::encode_null(&mut buf)?;
    binn_ir::encode_bool(&mut buf, true)?;
    binn_ir::encode_bool(&mut buf, false)?;
    binn_ir::encode_u8(&mut buf, 123_u8)?;
    binn_ir::encode_i8(&mut buf, -123_i8)?;
    binn_ir::encode_u16(&mut buf, 12345_u16)?;
    binn_ir::encode_i16(&mut buf, -12345_i16)?;
    binn_ir::encode_u32(&mut buf, 123456789_u32)?;
    binn_ir::encode_i32(&mut buf, -123456789_i32)?;
    binn_ir::encode_float(&mut buf, 123.0)?;
    binn_ir::encode_float(&mut buf, -123.0)?;
    binn_ir::encode_u64(&mut buf, 98765432123_u64)?;
    binn_ir::encode_i64(&mut buf, -98765432123_i64)?;
    binn_ir::encode_double(&mut buf, 0xAABB_CCDD_u64 as f64)?;
    binn_ir::encode_double(&mut buf, -0xAABB_CCDD_i64 as f64)?;
    binn_ir::encode_text(&mut buf, String::from("Mr. Reynholm"))?;
    binn_ir::encode_text(&mut buf, "hello-jen")?;
    binn_ir::encode_date_time(&mut buf, String::from("hermione"))?;
    binn_ir::encode_date(&mut buf, "ron")?;
    binn_ir::encode_time(&mut buf, String::from("harry"))?;
    binn_ir::encode_decimal_str(&mut buf, "ginny\t\0\n")?;

    let blob_strings = vec![
        "roy eats moss' orange".repeat(20),
        "moss kisses jen".repeat(30),
        "richmond is a ghost".repeat(40),
    ];
    for s in blob_strings.iter() {
        assert!(s.len() > i8::max_value() as usize);
        binn_ir::encode_blob(&mut buf, s.as_bytes())?;
    }

    // Decode
    let mut cursor = Cursor::new(&buf);
    assert_eq!(binn_ir::decode_null(&mut cursor)?, Some(()));
    assert_eq!(binn_ir::decode_bool(&mut cursor)?, Some(true));
    assert_eq!(binn_ir::decode_bool(&mut cursor)?, Some(false));
    assert_eq!(binn_ir::decode_u8(&mut cursor)?, Some(123));
    assert_eq!(binn_ir::decode_i8(&mut cursor)?, Some(-123));
    assert_eq!(binn_ir::decode_u16(&mut cursor)?, Some(12345));
    assert_eq!(binn_ir::decode_i16(&mut cursor)?, Some(-12345));
    assert_eq!(binn_ir::decode_u32(&mut cursor)?, Some(123456789));
    assert_eq!(binn_ir::decode_i32(&mut cursor)?, Some(-123456789));
    assert_eq!(binn_ir::decode_float(&mut cursor)?, Some(123.0));
    assert_eq!(binn_ir::decode_float(&mut cursor)?, Some(-123.0));
    assert_eq!(binn_ir::decode_u64(&mut cursor)?, Some(98765432123));
    assert_eq!(binn_ir::decode_i64(&mut cursor)?, Some(-98765432123));
    assert_eq!(binn_ir::decode_double(&mut cursor)?, Some(0xAABB_CCDD_u64 as f64));
    assert_eq!(binn_ir::decode_double(&mut cursor)?, Some(-0xAABB_CCDD_i64 as f64));
    assert_eq!(binn_ir::decode_text(&mut cursor)?.unwrap(), "Mr. Reynholm");
    assert_eq!(binn_ir::decode_text(&mut cursor)?.unwrap(), "hello-jen");
    assert_eq!(binn_ir::decode_date_time(&mut cursor)?.unwrap(), "hermione");
    assert_eq!(binn_ir::decode_date(&mut cursor)?.unwrap(), "ron");
    assert_eq!(binn_ir::decode_time(&mut cursor)?.unwrap(), "harry");
    assert_eq!(binn_ir::decode_decimal_str(&mut cursor)?.unwrap(), "ginny\t\0\n");

    for s in blob_strings.iter() {
        assert_eq!(binn_ir::decode_blob(&mut cursor)?.unwrap(), s.as_bytes());
    }

    // Verify position
    assert_eq!(binn_ir::decode(&mut cursor)?, None);
    assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);

    Ok(())
}

/// # Tries to decode from an invalid source, then unwraps an error
#[cfg(feature="std")]
macro_rules! decode_from_invalid_source { ($bytes: expr) => {{
    binn_ir::decode(&mut Cursor::new($bytes)).unwrap_err()
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
    assert_eq!(binn_ir::decode_blob(&mut cursor)?.unwrap(), [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]);
    assert_eq!(binn_ir::decode_null(&mut cursor)?, None);
    assert_eq!(cursor.position().cmp_to(&buf.len()), Ordering::Equal);

    // Small blob with 4-byte size; but data is missing
    match binn_ir::decode_blob(&mut Cursor::new(vec![
        value::BLOB,
        // Size: 15 bytes
        0x80, 0x00, 0x00, 0x0F,
    ])).unwrap_err().into_inner() {
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
            assert_eq!(binn_ir::decode_list(&mut cursor)?.unwrap(), list);
            println!("Verified: {:?}", &list);

            // Verify position
            assert_eq!(binn_ir::decode_map(&mut cursor).unwrap(), None);
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
            assert_eq!(binn_ir::decode_map(&mut cursor)?.unwrap(), map);
            println!("Verified: {:?}", &map);

            // Verify position
            assert_eq!(binn_ir::decode_object(&mut cursor)?, None);
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
            assert_eq!(binn_ir::decode_list(&mut cursor)?.unwrap(), list);
            println!("Verified: {:?}", &list);
            assert_eq!(binn_ir::decode_object(&mut cursor)?.unwrap(), object);
            println!("Verified: {:?}", &object);

            // Verify position
            assert_eq!(binn_ir::decode_null(&mut cursor)?, None);
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
