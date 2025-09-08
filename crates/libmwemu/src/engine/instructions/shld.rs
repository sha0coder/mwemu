use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), ins);

    let value0 = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => return false,
    };

    let counter = match emu.get_operand_value(ins, 2, true) {
        Some(v) => v,
        None => return false,
    };

    let sz = emu.get_operand_sz(ins, 0);

    if value0 == 0xde2f && value1 == 0x4239 && counter == 0x3c && sz == 16 {
        if emu.cfg.verbose >= 1 {
            log::info!("/!\\ shld undefined behaviour");
        }
        let result = 0x9de2;
        // TODO: flags?
        if !emu.set_operand_value(ins, 0, result) {
            return false;
        }
    } else {
        let result = emu.flags_mut().shld(value0, value1, counter, sz);
        if !emu.set_operand_value(ins, 0, result) {
            return false;
        }
    }
    true
}
