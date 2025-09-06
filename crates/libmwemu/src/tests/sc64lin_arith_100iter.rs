use crate::{tests::helpers, *};

#[test]
// this tests a linux 64bits raw arithmetic code.
pub fn sc64lin_arith_100iter() {
    helpers::setup();

    let mut emu = emu64();
    emu.cfg.maps_folder = "../../maps/maps64/".to_string();

    let sample = "../../test/sc64lin_arith_100iter.bin";
    emu.load_code(sample);
    emu.run(Some(0x3c0040));

    assert_eq!(emu.regs().rax, 0x4d9364d94bc0001e);
}
