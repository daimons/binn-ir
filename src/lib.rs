// License: see LICENSE file at root directory of `master` branch

//! # An implementation of Binn - <https://github.com/liteserver/binn>
//!
//! # Project
//!
//! - Repository: <https://bitbucket.org/haibison/binn-ir>
//! - License: [Free Public License 1.0.0](https://opensource.org/licenses/FPL-1.0.0)
//! - _This project follows [Semantic Versioning 2.0.0]_
//!
//! ---
//!
//! # Features
//!
//! - All official types are supported.
//! - User defined types are _not_ yet supported.
//!
//! # Notes
//!
//! - `#![no_std]` _might_ be supported when [`alloc`][crate:alloc] crate is stabilized.
//! - `IR` stands for _implementation in Rust_.
//! - Unless _absolutely necessary_, the project will **not** use any dependency.
//! - Core encoder and decoder are almost done (except ones for user defined types). However API might change, as the project is still in early
//!   development phase.
//! - There will be **no** plan to support secure encoder/decoder via cryptography. The author considers that another field for experts.
//! - However, simple API for safe encoder/decoder will be supported. For example: option to limit container size to be decoded...
//!
//! # Quick examples
//!
//! Most functionalities are provided via 2 traits: [`Encoder`][trait:value::Encoder], [`Decoder`][trait:value::Decoder].
//!
//! This example demonstrates a simple file container:
//!
//! ```
//! extern crate binn_ir;
//!
//! use binn_ir::value::{Value, Encoder, Decoder};
//!
//! const MAGIC_NUMBER: u64 = 0xABCD;
//!
//! // Buffer
//! let mut buf: Vec<u8> = vec![];
//!
//! // Magic number
//! buf.encode_u64(MAGIC_NUMBER).unwrap();
//!
//! // A single file header contains: name and hash
//! let file_header = {
//!     let mut map = std::collections::BTreeMap::new();
//!     map.insert(0_i32, Value::from("the-sun"));  // name
//!     map.insert(1_i32, Value::U64(0));           // hash
//!     map
//! };
//! let file_content = "is hot";
//!
//! // Encode data (using ::clone() to use the variable later in assertions)
//! buf.encode_map(file_header.clone()).unwrap();
//! buf.encode_blob(file_content.as_bytes()).unwrap();
//!
//! // Now decode data back
//! let mut cursor = std::io::Cursor::new(&buf);
//! assert_eq!(cursor.decode_u64().unwrap(), MAGIC_NUMBER);
//! assert_eq!(cursor.decode_map().unwrap(), file_header);
//! assert_eq!(cursor.decode_blob().unwrap(), file_content.as_bytes());
//! assert_eq!(cursor.position(), buf.len() as u64);
//! ```
//!
//! # Thanks
//!
//! Special thanks to [Binn]'s authors for their hard work.
//!
//! # Copy of specification
//!
//! Below is a copy of [Binn]'s specification, with some minor changes of formatting.
//!
//! - License: [Apache License, Version 2.0][Binn:License]
//! - Commit: <https://github.com/liteserver/binn/commit/29f3be8bb9d1014cabe8bd72ef53d2cfa6d443d8>
//! - Date: `Tue Jun 9 16:58:46 2015 -0300`
//!
//! > Binn Specification
//! > ==============
//! >
//! > Format
//! > --------
//! > Each value is stored with 4 possible parameters:
//! >
//! > ```Text
//! > [type][size][count][data]
//! > ```
//! >
//! > But most are optional. Only the type parameter is used in all of them. Here is a list of used parameters for basic data types:
//! > ```Text
//! > boolean, null:
//! > [type]
//! >
//! > int, float (storage: byte, word, dword or qword):
//! > [type][data]
//! >
//! > string, blob:
//! > [type][size][data]
//! >
//! > list, object, map:
//! > [type][size][count][data]
//! > ```
//! >
//! > Each parameter can be stored with polymorphic size:
//! >
//! > Parameter | Size
//! > ------- | ----
//! > [type]  | 1 or 2 bytes
//! > [size]  | 1 or 4 bytes
//! > [count] | 1 or 4 bytes
//! > [data]  | n bytes
//! >
//! >
//! > [Type]
//! > -----
//! > Each value is stored starting with the data type. It can use 1 or 2 bytes. The first byte is divided as follows:
//! >
//! > ```Text
//! >  +-------- Storage type
//! >  |  +----- Sub-type size
//! >  |  |  +-- Sub-type
//! > 000 0 0000
//! > ```
//! > #### Storage
//! >
//! > The 3 most significant bits are used for the **storage type**. It has information about how many bytes the data will use. The storage
//! > type can be any of:
//! >
//! > * No additional bytes
//! > * 1 Byte
//! > * Word (2 bytes, big endian)
//! > * Dword (4 bytes, big endian)
//! > * Qword (8 bytes, big endian)
//! > * String (UTF-8, null terminated)
//! > * Blob
//! > * Container
//! >
//! > And the constants are:
//! >
//! > Storage | Bits      | Hex  | Dec
//! > ------- | --------- | ---- | ---:
//! > NOBYTES | **000** 0 0000 | 0x00 | 0
//! > BYTE    | **001** 0 0000 | 0x20 | 32
//! > WORD    | **010** 0 0000 | 0x40 | 64
//! > DWORD   | **011** 0 0000 | 0x60 | 96
//! > QWORD   | **100** 0 0000 | 0x80 | 128
//! > STRING  | **101** 0 0000 | 0xA0 | 160
//! > BLOB    | **110** 0 0000 | 0xC0 | 192
//! > CONTAINER | **111** 0 0000 | 0xE0 | 224
//! >
//! > #### Sub-type size
//! >
//! > The next bit informs if the type uses 1 or 2 bytes.
//! >
//! > If the bit is 0, the type uses only 1 byte, and the sub-type has 4 bits (0 to 15)
//! > ```Text
//! >  +-------- Storage type
//! >  |  +----- Sub-type size
//! >  |  |  +-- Sub-type
//! > 000 0 0000
//! > ```
//! > When the bit is 1, another byte is used for the type and the sub-type has 12 bits (up to 4096)
//! > ```Text
//! >  +-------- Storage type
//! >  |  +----- Sub-type size
//! >  |  |
//! > 000 1 0000  0000 0000
//! >       |  Sub-type   |
//! >       +-------------+
//! > ```
//! >
//! > #### Sub-type
//! >
//! > Each storage can have up to 4096 sub-types. They hold what kind of value is stored in that storage space.
//! >
//! > > **Example:** a DWORD can contain a signed integer, an unsigned integer, a single precision floating point number, and many more... even
//! > > user defined types
//! >
//! > Here are the values for basic data types, with the sub-type highlighted:
//! >
//! > Type  | Storage | Bits     | Hex  | Dec
//! > ----- | ------- | -------- | ---- | ---:
//! > Null  | NOBYTES | 0000 **0000** | 0x00 | 0
//! > True  | NOBYTES | 0000 **0001** | 0x01 | 1
//! > False | NOBYTES | 0000 **0010** | 0x02 | 2
//! > UInt8 | BYTE    | 0010 **0000** | 0x20 | 32
//! > Int8  | BYTE    | 0010 **0001** | 0x21 | 33
//! > UInt16 | WORD    | 0100 **0000** | 0x40 | 64
//! > Int16  | WORD    | 0100 **0001** | 0x41 | 65
//! > UInt32 | DWORD   | 0110 **0000** | 0x60 | 96
//! > Int32  | DWORD   | 0110 **0001** | 0x61 | 97
//! > Float  | DWORD   | 0110 **0010** | 0x62 | 98
//! > UInt64 | QWORD   | 1000 **0000** | 0x80 |128
//! > Int64  | QWORD   | 1000 **0001** | 0x81 |129
//! > Double | QWORD   | 1000 **0010** | 0x82 |130
//! > Text   | STRING  | 1010 **0000** | 0xA0 |160
//! > DateTime | STRING  | 1010 **0001** | 0xA1 |161
//! > Date   | STRING  | 1010 **0010** | 0xA2 |162
//! > Time   | STRING  | 1010 **0011** | 0xA3 |163
//! > DecimalStr  | STRING  | 1010 **0100** | 0xA4 |164
//! > Blob   | BLOB    | 1100 **0000** | 0xC0 |192
//! > List | CONTAINER | 1110 **0000** | 0xE0 |224
//! > Map  | CONTAINER | 1110 **0001** | 0xE1 |225
//! > Object | CONTAINER | 1110 **0010**| 0xE2|226
//! >
//! > ### User Defined Types
//! >
//! > An application can use a different DateTime type and store the value in a DWORD or QWORD.
//! >
//! > >Storage = QWORD (0x80)<br/>
//! > >Sub-type = 5 (0x05) [choose any unused]
//! > >
//! > >Type DateTime = (0x80 | 0x05 => 0x85)
//! >
//! > An application can send HTML inside a Binn structure and can define a type to differ from plain text.
//! >
//! > >Storage = STRING (0xA0)<br/>
//! > >Sub-type = 9 (0x09) [choose any unused]
//! > >
//! > >Type HTML = (0xA0 | 0x09 => 0xA9)
//! >
//! > If the sub-type is greater than 15, a new byte must be used, and the sub-type size bit must be set:
//! >
//! > >Storage = STRING (0xA000)<br/>
//! > >Sub-type size = (0x0100)<br/>
//! > >Sub-type = 21 (0x0015)
//! > >
//! > >Type HTML = (0xA000 | 0x1000 | 0x0015 => 0xB015)
//! >
//! > The created type parameter must be stored as big-endian.
//! >
//! > [Size]
//! > -------
//! > This parameter is used in strings, blobs and containters. It can have 1 or 4 bytes.
//! >
//! > If the first bit of size is 0, it uses only 1 byte. So when the data size is up to 127 (0x7F) bytes the size parameter will use only 1
//! > byte.
//! >
//! > Otherwise a 4 byte size parameter is used, with the msb 1. Leaving us with a high limit of 2 GigaBytes (0x7FFFFFFF).
//! >
//! > Data size | Size Parameter Uses
//! > ---|--:
//! > &lt;= 127 bytes | 1 byte
//! > &gt; 127 bytes | 4 bytes
//! >
//! > There is no problem if a small size is stored using 4 bytes. The reader must accept both.
//! >
//! > For *strings*, the size parameter does not include the null terminator.
//! >
//! > For *containers*, the size parameter includes the type parameter. It stores the size of the whole structure.
//! >
//! > [Count]
//! > ---------
//! > This parameter is used only in containers to inform the number of items inside them. It can have 1 or 4 bytes, formatted exactly as the
//! > size parameter.
//! >
//! > Count | Count Parameter Uses
//! > ---|--:
//! > &lt;= 127 items | 1 byte
//! > &gt; 127 items | 4 bytes
//! >
//! >
//! > Containers
//! > -------------
//! >
//! > #### **List**
//! > Lists are containers that store values one after another.
//! >
//! > The count parameter informs the number of values inside the container.
//! >
//! > >[123, "test", 2.5, true]
//! >
//! > #### **Map**
//! > Maps are associative arrays using **integer numbers** for the keys.
//! >
//! > The keys are stored using a big-endian DWORD (4 bytes) that are read as a signed integer.
//! >
//! > So the current limits are from INT32_MIN to INT32_MAX. But there is room for increase if needed.
//! >
//! > The count parameter informs the number of key/value pairs inside the container.
//! >
//! > >{**1:** 10, **5:** "the value", **7:** true}
//! >
//! > #### **Object**
//! > Objects are associative arrays using **text** for the keys.
//! >
//! > The keys are not null terminated and the limit is 255 bytes long.
//! >
//! > The keys are stored preceded by the key length using a single byte for it.
//! >
//! > The count parameter informs the number of key/value pairs inside the container.
//! >
//! > >{**"id":** 1, **"name":** "John", **"points":** 30.5, **"active":** true}
//! >
//! >
//! > Limits
//! > -------
//! >
//! > Type | Min | Max
//! > -----|-----|----
//! > Integers | INT64_MIN | UINT64_MAX
//! > Floating point numbers | IEEE 754 |
//! > Strings | 0 | 2 GB
//! > Blobs | 0 | 2 GB
//! > Containers | 4 | 2 GB
//! >
//! > Associative Arrays
//! >
//! > Key type | Min | Max
//! > ---------|-----|-----
//! > Number | INT32_MIN | INT32_MAX
//! > Text | 0 | 255 bytes
//! >
//! > Sub-types: up to 4096 for each storage type
//! >
//! >
//! > Example Structures
//! > -----------------------
//! > #### A json data such as {"hello":"world"} is serialized as:
//! >
//! > **Binn:** (17 bytes)
//! > ```Text
//! >   \xE2           // [type] object (container)
//! >   \x11           // [size] container total size
//! >   \x01           // [count] key/value pairs
//! >   \x05hello      // key
//! >   \xA0           // [type] = string
//! >   \x05           // [size]
//! >   world\x00      // [data] (null terminated)
//! > ```
//! >
//! > #### A list of 3 integers:
//! >
//! > **Json:**  (14 bytes)
//! > >[123, -456, 789]
//! >
//! > **Binn:** (11 bytes)
//! > ```Text
//! >   \xE0           // [type] list (container)
//! >   \x0B           // [size] container total size
//! >   \x03           // [count] items
//! >   \x20           // [type] = uint8
//! >   \x7B           // [data] (123)
//! >   \x41           // [type] = int16
//! >   \xFE\x38       // [data] (-456)
//! >   \x40           // [type] = uint16
//! >   \x03\x15       // [data] (789)
//! > ```
//! >
//! > #### A list inside a map:
//! >
//! > **Json:**  (25 bytes)
//! > >{1: "add", 2: [-12345, 6789]}
//! >
//! > **Binn:** (26 bytes)
//! > ```Text
//! >  \xE1             // [type] map (container)
//! >  \x1A             // [size] container total size
//! >  \x02             // [count] key/value pairs
//! >  \x00\x00\x00\x01 // key
//! >  \xA0             // [type] = string
//! >  \x03             // [size]
//! >  add\x00          // [data] (null terminated)
//! >  \x00\x00\x00\x02 // key
//! >  \xE0             // [type] list (container)
//! >  \x09             // [size] container total size
//! >  \x02             // [count] items
//! >  \x41             // [type] = int16
//! >  \xCF\xC7         // [data] (-12345)
//! >  \x40             // [type] = uint16
//! >  \x1A\x85         // [data] (6789)
//! > ```
//! >
//! >
//! > #### A list of objects:
//! >
//! > **Json:** (47 bytes)
//! > >[
//! > {"id": 1, "name": "John"},
//! > {"id": 2, "name": "Eric"}
//! > ]
//! >
//! > **Binn:** (43 bytes)
//! > ```Text
//! >  \xE0           // [type] list (container)
//! >  \x2B           // [size] container total size
//! >  \x02           // [count] items
//! >
//! >  \xE2           // [type] object (container)
//! >  \x14           // [size] container total size
//! >  \x02           // [count] key/value pairs
//! >
//! >  \x02id         // key
//! >  \x20           // [type] = uint8
//! >  \x01           // [data] (1)
//! >
//! >  \x04name       // key
//! >  \xA0           // [type] = string
//! >  \x04           // [size]
//! >  John\x00       // [data] (null terminated)
//! >
//! >  \xE2           // [type] object (container)
//! >  \x14           // [size] container total size
//! >  \x02           // [count] key/value pairs
//! >
//! >  \x02id         // key
//! >  \x20           // [type] = uint8
//! >  \x02           // [data] (2)
//! >
//! >  \x04name       // key
//! >  \xA0           // [type] = string
//! >  \x04           // [size]
//! >  Eric\x00       // [data] (null terminated)
//! > ```
//! >
//!
//! [Semantic Versioning 2.0.0]: https://semver.org/spec/v2.0.0.html
//! [Binn]: https://github.com/liteserver/binn
//! [Binn:License]: https://github.com/liteserver/binn/blob/master/LICENSE
//! [crate:alloc]: https://doc.rust-lang.org/alloc/
//! [trait:value::Encoder]: value/trait.Encoder.html
//! [trait:value::Decoder]: value/trait.Decoder.html

// TODO: enable this flag when `alloc` crate is stabilized
// #![no_std]

// ╔═════════════════╗
// ║   IDENTIFIERS   ║
// ╚═════════════════╝

macro_rules! crate_code_name    { () => { "binn-ir" }}
macro_rules! crate_version      { () => { "0.5.1" }}

/// # Crate name
///
/// IR stands for _implementation in Rust_.
pub const CRATE_NAME: &'static str = "Binn-IR";

/// # Crate code name
pub const CRATE_CODE_NAME: &'static str = crate_code_name!();

/// # Crate version
pub const CRATE_VERSION: &'static str = crate_version!();

/// # Crate release date (year/month/day)
pub const CRATE_RELEASE_DATE: (u16, u8, u8) = (2018, 7, 15);

/// # Unique universally identifier of this crate. Its CRC-32 is `149dc8a5`.
pub const UUID: &'static str = "acea8479-f233-4686-af1c-fe198f506ddb";

/// # Tag, which can be used for logging...
pub const TAG: &'static str = concat!(crate_code_name!(), "_149dc8a5_", crate_version!());

// ╔════════════════════╗
// ║   IMPLEMENTATION   ║
// ╚════════════════════╝

#[test]
fn test_crate_version() {
    assert_eq!(CRATE_VERSION, env!("CARGO_PKG_VERSION"));
}

#[macro_use]
mod cmp_integers;

pub mod storage;
pub mod value;
