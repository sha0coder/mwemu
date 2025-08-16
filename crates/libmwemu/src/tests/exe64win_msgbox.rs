use crate::*;
use crate::tests::helpers;

#[test]
// this test a windows 64bits executable that calculates apis like shellcodes and does basic api calls.
// aso read strings and patch string.
pub fn exe64win_msgbox() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();

    let sample = "../../test/exe64win_msgbox.bin";
    emu.load_code(sample);
    emu.run(Some(0x14000123f));

    let message = emu.maps.read_string(emu.regs().rdx);
    let title = emu.maps.read_string(emu.regs().rdi);

    assert_eq!(message, "message");
    assert_eq!(title, "title");

    emu.maps.write_string(emu.regs().rdx, "inject");

    // launch the msgbox
    emu.step();
}