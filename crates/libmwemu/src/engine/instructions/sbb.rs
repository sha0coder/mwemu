use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), ins);

    assert!(ins.op_count() == 2);

    let cf: u64 = if emu.flags().f_cf { 1 } else { 0 };

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);
    let res: u64 = match sz {
        64 => emu.flags_mut().sub64(value0, value1.wrapping_add(cf)),
        32 => emu
            .flags_mut()
            .sub32(value0, (value1 & 0xffffffff).wrapping_add(cf)),
        16 => emu
            .flags_mut()
            .sub16(value0, (value1 & 0xffff).wrapping_add(cf)),
        8 => emu
            .flags_mut()
            .sub8(value0, (value1 & 0xff).wrapping_add(cf)),
        _ => panic!("weird size"),
    };

    if !emu.set_operand_value(ins, 0, res) {
        return false;
    }
    true
}
