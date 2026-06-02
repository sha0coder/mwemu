use crate::tests::helpers;
use crate::*;

#[test]
pub fn exception_handler32() {
    helpers::setup();

    let path = match helpers::optional_test_data_path("exe32win_exception_handler.bin") {
        Some(p) => p,
        None => return,
    };

    let mut emu = emu32();
    emu.cfg.maps_folder = helpers::win32_maps_folder();
    emu.load_code(&path);
}
