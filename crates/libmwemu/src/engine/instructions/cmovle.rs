use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Orange"), ins);

    let value1 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        _ => return false,
    };
    let value = if ins.op0_register().is_gpr64() {
        value1
    } else {
        value1 & 0xffffffff
    };
    emu.set_operand_value(ins, 0, value);
    if emu.flags().f_zf || emu.flags().f_sf != emu.flags().f_of {
        let value2 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            _ => return false,
        };
        let value_new = if ins.op0_register().is_gpr64() {
            value2
        } else {
            value2 & 0xffffffff
        };
        if !emu.set_operand_value(ins, 0, value_new) {
            return false;
        }
    }
    true
}
