use crate::tests::helpers;
use crate::*;

#[test]
#[ignore]
fn test_mingw32() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();

    let sample = "../../test/exe32win_mingw.bin";
    emu.load_code(sample);
    emu.run(None);
    assert!(emu.pos > 325);
    assert!(1==2);
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