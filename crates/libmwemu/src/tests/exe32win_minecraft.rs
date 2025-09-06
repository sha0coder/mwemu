use crate::tests::helpers;
use crate::*;

#[test]
// this tests a windows 32bits executable, that require iat binding of multiple libs.
pub fn exe32win_minecraft() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();

    let sample = "../../test/exe32win_minecraft.bin";
    emu.load_code(sample);
    emu.run(Some(0x403740));

    assert_eq!(emu.regs().get_ebx(), 2);
}
