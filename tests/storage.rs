// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

use binn_ir::storage;

#[test]
fn storages() {
    assert!(storage::NO_BYTES   == 0b_000);
    assert!(storage::BYTE       == 0b_001);
    assert!(storage::WORD       == 0b_010);
    assert!(storage::DWORD      == 0b_011);
    assert!(storage::QWORD      == 0b_100);
    assert!(storage::STRING     == 0b_101);
    assert!(storage::BLOB       == 0b_110);
    assert!(storage::CONTAINER  == 0b_111);
}
