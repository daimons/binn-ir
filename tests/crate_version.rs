// License: see LICENSE file at root directory of `master` branch

extern crate binn_ir;

#[test]
fn crate_version() {
    assert!(binn_ir::CRATE_VERSION == env!("CARGO_PKG_VERSION"));
}
