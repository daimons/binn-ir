<!-- License: see LICENSE file at root directory of `master` branch -->

# `0.6.0` _(July 16th, 2018)_

---

## Changes

- `value`:

    + `Value::decode()`: changed result type from `io::Result<Self>` to `io::Result<Option<Self>>`. If it returns `Ok(None)`, that means there's
      no more data to decode.

    + Updated `Decoder`'s functions for above change. For examples:

        * `::decode_null()` -> from `io::Result<()>` to `io::Result<Option<()>>`.
        * `::decode_float()` -> from `io::Result<f32>` to `io::Result<Option<f32>>`.
        * ...

    + Fixed bug decoding lists, maps, objects.

    + For maps, objects: beside minor bug fixes, decoders now also check for duplicate keys.

- Error messages are now prefixed with `::TAG`.

- Added more tests.

## Dependencies

No dependencies.

---
