use crate::emu::Emu;
use crate::{color, get_bit};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), ins);

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0) as u64;
    let mut temp: u64 = 0;
    let mut dest: u64 = 0;

    while temp < sz && get_bit!(value1, temp) == 0 {
        temp += 1;
        dest += 1;
    }

    emu.flags_mut().f_cf = dest == sz;
    emu.flags_mut().f_zf = dest == 0;

    emu.set_operand_value(ins, 1, dest);
    true
}
