<!-- License: see LICENSE file at root directory of `master` branch -->

# `0.3.0` _(July 13th, 2018)_

---

- `value::Value`:

    + Renamed `::write()` -> `::encode()`, `::read()` -> `::decode()`.
    + Implemented `From<_>` for some data types that its variants hold.

- `value`:

    + Removed helper functions `::read_*()`.
    + Removed type `DataSize`.

- `value::Encoder`: added helper functions `::encode_*()` for data types that `value::Value`'s variants hold.

## Dependencies

No dependencies.

---
