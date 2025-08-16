use crate::{constants, emu};

pub fn FindNextFileW(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let find_data = emu.regs().rdx;

    log::info!(
        "{}** {} kernel32!FindNextFileW {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    // TODO: implement

    emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}