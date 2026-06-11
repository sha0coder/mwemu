use crate::tests::helpers;
use crate::*;

#[test]
fn test_mingw32() {
    helpers::setup();

    let sample = match helpers::optional_test_data_path("exe32win_mingw.bin") {
        Some(p) => p,
        None => return,
    };

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();

    emu.load_code(&sample);
    emu.run_to(119)
        .expect("mingw32 should reach the early execution window");
    assert!(emu.pos >= 119);
}

#[test]
fn test_mingw64() {
    helpers::setup();

    let sample = match helpers::optional_test_data_path("exe64win_mingw.bin") {
        Some(p) => p,
        None => return,
    };

    let mut emu = emu64();
    emu.cfg.maps_folder = helpers::win64_maps_folder();

    emu.load_code(&sample);
    emu.run_to(100);
}
