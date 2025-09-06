use crate::{tests::helpers, *};

#[test]
// the donut shellcode generator, with a 32bits truncated payload, emulate 30_862_819
// instructions and check.
pub fn sc32win_donut() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();

    let sample = "../../test/sc32win_donut.bin";
    emu.load_code(sample);
    emu.run_to(30_862_819);

    assert_eq!(emu.regs().get_eax(), 0xF5B24B1D); // used to be 0x7f937230?
    assert_eq!(emu.regs().get_ebx(), 0x12); // used to be 0x0c
}
