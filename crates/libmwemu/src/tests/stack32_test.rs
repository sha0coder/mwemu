use crate::{tests::helpers, *};

#[test]
// stack32 tests
pub fn stack32_test() {
    helpers::setup();

    let mut emu = emu32();
    emu.cfg.maps_folder = "../../maps/maps32/".to_string();
    emu.init(false, false);

    let stack_check = emu.maps.get_map_by_name("stack");
    assert!(stack_check.is_some());
    let stack = stack_check.unwrap();
    let base = stack.get_base();

    assert!(emu.regs().get_esp() < emu.regs().get_ebp());
    assert!(emu.regs().get_esp() > stack.get_base());
    assert!(emu.regs().get_esp() < stack.get_bottom());
    assert!(emu.regs().get_ebp() > stack.get_base());
    assert!(emu.regs().get_ebp() < stack.get_bottom());
    assert!(stack.inside(emu.regs().get_esp()));
    assert!(stack.inside(emu.regs().get_ebp()));

    for i in 0..5000 {
        emu.stack_push32(i as u32);
    }
    emu.stack_pop32(false);

    assert!(emu.regs().get_esp() > base);
}
