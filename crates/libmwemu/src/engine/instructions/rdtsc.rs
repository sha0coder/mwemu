use crate::emu::Emu;
use crate::{color};
use iced_x86::{Instruction};

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    let elapsed = emu.now.elapsed();
    let cycles: u64 = elapsed.as_nanos() as u64;
    emu.regs_mut().rax = cycles & 0xffffffff;
    emu.regs_mut().rdx = cycles >> 32;
    true
}
