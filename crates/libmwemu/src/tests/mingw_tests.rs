use crate::tests::helpers;
use crate::*;

#[test]
fn test_mingw32() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();

    let sample = "../../test/exe32win_mingw.bin";
    emu.load_code(sample);
    // Keep this test aligned with the CLI check window where execution reaches ~119.
    emu.run_to(119).expect("mingw32 should reach the early execution window");
    assert!(emu.pos >= 119);
}

#[test]
fn test_mingw64() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();

    let sample = "../../test/exe64win_mingw.bin";
    emu.load_code(sample);
    emu.run_to(100);
}