use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    assert!(ins.op_count() == 2);

    let value1 = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => {
            emu.show_instruction_comment(color!("LightCyan"), ins, "error");
            return false;
        }
    };
    if emu.cfg.verbose >= 2 {
        emu.show_instruction_comment(color!("LightCyan"), ins, &format!("0x{:x}", value1));
    }

    /*
    if emu.pos == 189464541 {
        let addr = emu.get_operand_value(ins, 1, false).unwrap();
        log::info!("-----> [0x{:x}]", addr);
    }*/

    if !emu.set_operand_value(ins, 0, value1) {
        return false;
    }
    true
}
