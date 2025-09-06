use crate::tests::helpers;
use crate::*;

#[test]
// test configuration management
pub fn config_management() {
    helpers::setup();

    let mut cfg = crate::config::Config::new();

    // Test default values
    assert!(!cfg.is_64bits); // should default to 32-bit

    // Test 32/64-bit mode switching
    cfg.is_64bits = true;
    assert!(cfg.is_64bits);

    cfg.is_64bits = false;
    assert!(!cfg.is_64bits);

    // Test maps folder configuration
    cfg.maps_folder = "/test/path".to_string();
    assert_eq!(cfg.maps_folder, "/test/path");

    // Test other configuration options
    cfg.verbose = 3;
    assert_eq!(cfg.verbose, 3);

    // Test emulator with different configs
    let emu32 = emu32();
    assert!(!emu32.cfg.is_64bits);

    let emu64 = emu64();
    assert!(emu64.cfg.is_64bits);
}
