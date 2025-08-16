use crate::emu;

pub fn GetLastError(emu: &mut emu::Emu) {
    let err = LAST_ERROR.lock().unwrap();
    emu.regs_mut().rax = *err as u64;
    log::info!(
        "{}** {} kernel32!GetLastError ={} {}",
        emu.colors.light_red,
        emu.pos,
        emu.regs().rax,
        emu.colors.nc
    );
}