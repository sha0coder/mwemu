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
    let result1: u64;
    let result2: u64;

    match sz {
        8 => {
            result1 = (value0 & 0xff) | (value1 & 0xff);
            result2 = (value0 & 0xffffffffffffff00) + result1;
        }
        16 => {
            result1 = (value0 & 0xffff) | (value1 & 0xffff);
            result2 = (value0 & 0xffffffffffff0000) + result1;
        }
        32 => {
            result1 = (value0 & 0xffffffff) | (value1 & 0xffffffff);
            result2 = (value0 & 0xffffffff00000000) + result1;
        }
        64 => {
            result1 = value0 | value1;
            result2 = result1;
        }
        _ => unreachable!(""),
    }

    let bits = emu.get_operand_sz(ins, 0);
    emu.flags_mut().calc_flags(result1, bits);
    emu.flags_mut().f_of = false;
    emu.flags_mut().f_cf = false;
    emu.flags_mut().calc_pf(result1 as u8);

    if !emu.set_operand_value(ins, 0, result2) {
        return false;
    }
    true
}
