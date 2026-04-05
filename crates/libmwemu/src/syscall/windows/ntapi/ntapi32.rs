use crate::emu;

pub fn gateway(syscall: u64, argv: u64, emu: &mut emu::Emu) {
    match syscall {
        0xdc => {
            log::trace!("/!\\ direct syscall: NtAlpcSendWaitReceivePort");
            emu.regs_mut().rax = 0;
        }

        0x10f => {
            log::trace!("/!\\ direct syscall: NtOpenFile {:x}", argv);
            emu.regs_mut().rax = 0;
        }

        _ => {
            let ins = emu.x86_instruction().unwrap();
            let output = emu.x86_format_instruction(&ins);
            log::trace!(
                "{}{} 0x{:x}: {}{}",
                emu.colors.red,
                emu.pos,
                emu.regs().rip,
                output,
                emu.colors.nc
            );
            unimplemented!();
        }
    }
}
