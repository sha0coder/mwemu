use crate::color;
use crate::emu::Emu;
use crate::syscall::syscall64;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    syscall64::gateway(emu);
    true
}
