use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), ins);

    let value0 = emu.get_operand_xmm_value_128(ins, 0, true).unwrap_or(0);
    let shift_amount = emu.get_operand_xmm_value_128(ins, 1, true).unwrap_or(0) as u32;
    let byte_shift = (shift_amount % 16) * 8; // Desplazamiento en bits

    let result = if byte_shift < 128 {
        value0 << byte_shift
    } else {
        0u128
    };

    emu.set_operand_xmm_value_128(ins, 0, result);
    true
}
