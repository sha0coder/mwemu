use crate::color;
use crate::emu::Emu;
use crate::syscall::linux;
use crate::syscall::windows;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    if emu.cfg.trace_calls && !emu.linux {
        log::trace!(
            "{} 0x{:x} SYSCALL nr=0x{:x}",
            emu.pos,
            ins.ip(),
            emu.regs().rax
        );
    }

    if emu.linux {
        linux::syscall64::gateway(emu);
    } else {
        windows::syscall64::gateway(emu);
    }
    true
}
