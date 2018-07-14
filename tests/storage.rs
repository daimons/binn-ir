// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use binn_ir::storage;

#[test]
fn constants() {
    assert_eq!(storage::NO_BYTES,   0b_000);
    assert_eq!(storage::BYTE,       0b_001);
    assert_eq!(storage::WORD,       0b_010);
    assert_eq!(storage::DWORD,      0b_011);
    assert_eq!(storage::QWORD,      0b_100);
    assert_eq!(storage::STRING,     0b_101);
    assert_eq!(storage::BLOB,       0b_110);
    assert_eq!(storage::CONTAINER,  0b_111);
}
