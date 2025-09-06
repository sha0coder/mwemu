use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), ins);

    let src = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let res = src.count_ones();
    if !emu.set_operand_value(ins, 0, res as u64) {
        return false;
    }
    true
}
