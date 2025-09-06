use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), ins);

    emu.regs_mut().rax = emu.regs().get_eax() as u32 as i32 as i64 as u64;
    // sign extend
    true
}
