use crate::emu::Emu;

// https://github.com/j00ru/windows-syscalls



pub fn gateway(emu: &mut Emu) {
    match emu.regs().rax {
        _ => {
            log::info!(
                "{}** {} syscall {} (unimplemented) {}",
                emu.colors.light_red,
                emu.colors.cyan,
                emu.regs().rax,
                emu.colors.nc
            );
        }
    }
}
