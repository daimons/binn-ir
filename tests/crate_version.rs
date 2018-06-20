// License: see LICENSE file at root directory of `master` branch

extern crate binnx;

#[test]
fn crate_version() {
    assert!(binnx::CRATE_VERSION == env!("CARGO_PKG_VERSION"));
}
