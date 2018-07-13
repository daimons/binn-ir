<!-- License: see LICENSE file at root directory of `master` branch -->

# `0.2.0` _(July 13th, 2018)_

---

- `value::Value`:

    + Implemented `AsRef`, `Clone`.
    + Implemented `From<_>` for some data types that its variants hold.

- `value`:

    + Added new traits with all provided helper functions:

            ::rust
            pub trait Encoder: std::io::Write + Sized
            pub trait Decoder: std::io::Read + Sized

    + Implemented `Encoder` for all implementors of `Write`'s.
    + Implemented `Decoder` for all implementors of `Read`'s.

- Optimized code.

## Dependencies

No dependencies.

---
