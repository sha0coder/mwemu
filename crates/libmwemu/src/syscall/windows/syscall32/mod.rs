use crate::emu::Emu;

pub fn gateway(emu: &mut Emu) {
    match emu.regs().rax {
        _ => {
            log_red!(emu,
                "{}** {} syscall {} (unimplemented) {}",
                emu.colors.light_red,
                emu.colors.cyan,
                emu.regs().rax,
                emu.colors.nc
            );
        }
    }
}
