use crate::tests::helpers;
use crate::*;

#[test]
// context objects for exception recovering
#[ignore]
pub fn exception_handler32() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();
    emu.load_code("../../test/exe32win_exception_handler.bin");
}
