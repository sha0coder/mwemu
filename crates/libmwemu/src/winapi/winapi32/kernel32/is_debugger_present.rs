use crate::emu;

pub fn IsDebuggerPresent(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!IsDebuggerPresent {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.regs_mut().rax = 0; // of course :p
}