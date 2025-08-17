
use crate::emu;

pub fn GetCurrentThread(emu: &mut emu::Emu) {
    log_red!(emu, "** {} kernel32!GetCurrentThread", emu.pos);
    // TODO: implement this
    panic!("TODO");
    emu.regs_mut().rax = 3;
}