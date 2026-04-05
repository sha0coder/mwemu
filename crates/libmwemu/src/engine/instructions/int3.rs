use crate::emu::Emu;
use crate::{color, exception::types};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);
    log::trace!("/!\\ int 3 sigtrap!!!!");
    emu.exception(types::ExceptionType::Int3);
    return true;
}
