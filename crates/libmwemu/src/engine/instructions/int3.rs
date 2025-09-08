use crate::emu::Emu;
use crate::{color, exception_type};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);
    log::info!("/!\\ int 3 sigtrap!!!!");
    emu.exception(exception_type::ExceptionType::Int3);
    return true;
}
