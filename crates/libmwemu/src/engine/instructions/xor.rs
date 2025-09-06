use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), ins);

    assert!(ins.op_count() == 2);
    assert!(emu.get_operand_sz(ins, 0) == emu.get_operand_sz(ins, 1));

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let result = value0 ^ value1;

    emu.flags_mut().calc_flags(result, sz);
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_cf = false;
    emu.flags_mut().calc_pf(result as u8);

    if !emu.set_operand_value(ins, 0, result) {
        return false;
    }
    true
}
