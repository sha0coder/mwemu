use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Orange"), ins);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    if value0 as u8 == (emu.regs().get_al() as u8) {
        emu.flags_mut().f_zf = true;
        if !emu.set_operand_value(ins, 0, value1) {
            return false;
        }
    } else {
        emu.flags_mut().f_zf = false;
        emu.regs_mut().set_al(value1 & 0xff);
    }
    true
}
