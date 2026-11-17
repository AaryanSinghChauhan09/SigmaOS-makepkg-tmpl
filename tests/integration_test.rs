// Integration test for SigmaOS-makepkg-tmpl
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_makepkg_tmpl as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
