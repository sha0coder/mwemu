use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), ins);

    let eflags = match emu.stack_pop64(true) {
        Some(v) => v as u32,
        None => return false,
    };
    emu.flags_mut().load(eflags);
    true
}
