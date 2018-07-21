<!-- License: see LICENSE file at root directory of `master` branch -->

# `0.9.0` _(July 22nd, 2018)_

---

## Changes

`value`: added functions to encode inner data of `Value`'s variants into `io::Write`:

    :::rust
    pub fn encode_null(buf: &mut Write) -> io::Result<u32>
    pub fn encode_bool(buf: &mut Write, b: impl Into<bool>) -> io::Result<u32>
    pub fn encode_u8(buf: &mut Write, u: impl Into<u8>) -> io::Result<u32>
    pub fn encode_i8(buf: &mut Write, i: impl Into<i8>) -> io::Result<u32>
    pub fn encode_u16(buf: &mut Write, u: impl Into<u16>) -> io::Result<u32>
    pub fn encode_i16(buf: &mut Write, i: impl Into<i16>) -> io::Result<u32>
    pub fn encode_u32(buf: &mut Write, u: impl Into<u32>) -> io::Result<u32>
    pub fn encode_i32(buf: &mut Write, i: impl Into<i32>) -> io::Result<u32>
    pub fn encode_u64(buf: &mut Write, u: impl Into<u64>) -> io::Result<u32>
    pub fn encode_i64(buf: &mut Write, i: impl Into<i64>) -> io::Result<u32>
    pub fn encode_float(buf: &mut Write, f: impl Into<f32>) -> io::Result<u32>
    pub fn encode_double(buf: &mut Write, d: impl Into<f64>) -> io::Result<u32>
    pub fn encode_text(buf: &mut Write, s: impl Into<String>) -> io::Result<u32>
    pub fn encode_date_time(buf: &mut Write, s: impl Into<String>) -> io::Result<u32>
    pub fn encode_date(buf: &mut Write, s: impl Into<String>) -> io::Result<u32>
    pub fn encode_time(buf: &mut Write, s: impl Into<String>) -> io::Result<u32>
    pub fn encode_decimal_str(buf: &mut Write, s: impl Into<String>) -> io::Result<u32>
    pub fn encode_blob(buf: &mut Write, bytes: impl Into<Vec<u8>>) -> io::Result<u32>
    pub fn encode_list(buf: &mut Write, list: impl Into<Vec<Value>>) -> io::Result<u32>
    pub fn encode_map(buf: &mut Write, map: impl Into<BTreeMap<i32, Value>>) -> io::Result<u32>
    pub fn encode_object(buf: &mut Write, object: impl Into<HashMap<String, Value>>) -> io::Result<u32>

## Dependencies

No dependencies.

---
