use crate::emu::Emu;

pub fn gateway(emu: &mut Emu) {
    match emu.regs().rax {
        _ => {
            let nr = emu.regs().rax as u32;
            log_orange!(emu, "syscall 0x{:x}: (unimplemented)", nr);
        }
    }
}
