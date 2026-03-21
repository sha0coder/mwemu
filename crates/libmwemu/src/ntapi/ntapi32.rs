use crate::emu;
use iced_x86::Formatter;

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
            let mut output = String::new();
            emu.formatter.format(&emu.instruction.unwrap(), &mut output);
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
