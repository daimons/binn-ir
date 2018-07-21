<!-- License: see LICENSE file at root directory of `master` branch -->

# `0.8.0` _(July 22nd, 2018)_

---

## Changes

`value`: added functions to decode from `io::Read` to inner data from `Value`'s variants:

    :::rust
    pub fn decode_null(source: &mut Read) -> io::Result<Option<()>>
    pub fn decode_bool(source: &mut Read) -> io::Result<Option<bool>>
    pub fn decode_u8(source: &mut Read) -> io::Result<Option<u8>>
    pub fn decode_i8(source: &mut Read) -> io::Result<Option<i8>>
    pub fn decode_u16(source: &mut Read) -> io::Result<Option<u16>>
    pub fn decode_i16(source: &mut Read) -> io::Result<Option<i16>>
    pub fn decode_u32(source: &mut Read) -> io::Result<Option<u32>>
    pub fn decode_i32(source: &mut Read) -> io::Result<Option<i32>>
    pub fn decode_u64(source: &mut Read) -> io::Result<Option<u64>>
    pub fn decode_i64(source: &mut Read) -> io::Result<Option<i64>>
    pub fn decode_float(source: &mut Read) -> io::Result<Option<f32>>
    pub fn decode_double(source: &mut Read) -> io::Result<Option<f64>>
    pub fn decode_text(source: &mut Read) -> io::Result<Option<String>>
    pub fn decode_date_time(source: &mut Read) -> io::Result<Option<String>>
    pub fn decode_date(source: &mut Read) -> io::Result<Option<String>>
    pub fn decode_time(source: &mut Read) -> io::Result<Option<String>>
    pub fn decode_decimal_str(source: &mut Read) -> io::Result<Option<String>>
    pub fn decode_blob(source: &mut Read) -> io::Result<Option<Vec<u8>>>
    pub fn decode_list(source: &mut Read) -> io::Result<Option<Vec<Value>>>
    pub fn decode_map(source: &mut Read) -> io::Result<Option<BTreeMap<i32, Value>>>
    pub fn decode_object(source: &mut Read) -> io::Result<Option<HashMap<String, Value>>>

## Dependencies

No dependencies.

---
