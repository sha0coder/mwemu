use crate::tests::helpers;
use crate::*;

#[test]
// enigma packer should be emulated at least 102,302,404 insturctions.
// this test is few seconds slow but will verify many cpu instructions.
pub fn exe64win_enigma() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();

    let sample = "../../test/exe64win_enigma.bin";
    emu.load_code(sample);
    emu.run(Some(0x140578ad3));

    assert!(emu.pos > 102302239);
}
