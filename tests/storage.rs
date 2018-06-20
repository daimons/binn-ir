// License: see LICENSE file at root directory of `master` branch

extern crate binnx;

use binnx::Storage;

#[test]
fn storage() {
    assert!(Storage::NOBYTES    == 0b000);
    assert!(Storage::BYTE       == 0b001);
    assert!(Storage::WORD       == 0b010);
    assert!(Storage::DWORD      == 0b011);
    assert!(Storage::QWORD      == 0b100);
    assert!(Storage::STRING     == 0b101);
    assert!(Storage::BLOB       == 0b110);
    assert!(Storage::CONTAINER  == 0b111);
}
