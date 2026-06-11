use crate::tests::helpers;
use crate::*;

#[test]
pub fn sc64win_strgen() {
    helpers::setup();

    let path = match helpers::optional_test_data_path("sc64win_strgen.bin") {
        Some(p) => p,
        None => return,
    };

    let mut emu = emu64();
    emu.set_verbose(3);
    emu.cfg.maps_folder = helpers::win64_maps_folder();
    emu.load_code(&path);
    emu.run_to(231);
    let s = emu.maps.read_string(0x329ec8);
    assert_eq!(s, "http://something.com/");
}
