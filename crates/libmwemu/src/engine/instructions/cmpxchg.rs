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

    if emu.cfg.is_64bits {
        if value0 == emu.regs().rax {
            emu.flags_mut().f_zf = true;
            if !emu.set_operand_value(ins, 0, value1) {
                return false;
            }
        } else {
            emu.flags_mut().f_zf = false;
            emu.regs_mut().rax = value1;
        }
    } else {
        // 32bits
        if value0 == emu.regs().get_eax() {
            emu.flags_mut().f_zf = true;
            if !emu.set_operand_value(ins, 0, value1) {
                return false;
            }
        } else {
            emu.flags_mut().f_zf = false;
            emu.regs_mut().set_eax(value1);
        }
    }
    true
}
