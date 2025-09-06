use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), ins);
    emu.flags_mut().f_tf = false;
    if !emu.stack_push64(emu.flags().dump() as u64) {
        return false;
    }
    true
}
