
use crate::emu;

pub fn GetCurrentThread(emu: &mut emu::Emu) {
    log_red!(emu, "** {} kernel32!GetCurrentThread", emu.pos);
    panic!("TODO");
}
