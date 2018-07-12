<!-- License: see LICENSE file at root directory of `master` branch -->

# `0.1.0` _(July 12th, 2018)_

---

- `value::Value`:

    + Moved all static functions `::read_*()` to parent module.
    + Moved constant `::MAX_DATA_SIZE` to parent module.

- `value`:

    + Added new function:

            ::rust
            pub fn read_blob(&mut std::io::Read) -> std::io::Result<std::vec::Vec<u8>>

    + Published constant `::OBJECT_KEY_MAX_LEN`.

- Optimized code.

## Dependencies

No dependencies.

---
