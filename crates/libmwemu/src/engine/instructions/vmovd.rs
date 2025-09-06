use crate::emu::Emu;
use crate::{color, regs64};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), ins);

    assert!(ins.op_count() == 2);
    assert!(emu.get_operand_sz(ins, 1) == 32);

    let value = match emu.get_operand_value(ins, 1, true) {
        Some(v) => v,
        None => {
            log::info!("error reading second operand");
            return false;
        }
    };

    match emu.get_operand_sz(ins, 0) {
        128 => {
            emu.set_operand_xmm_value_128(ins, 0, value as u128);
        }
        256 => {
            let result = regs64::U256::from(value);
            emu.set_operand_ymm_value_256(ins, 0, result);
        }
        _ => unimplemented!(""),
    }
    true
}
