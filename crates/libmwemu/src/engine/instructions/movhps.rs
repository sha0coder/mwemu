use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Green"), ins);
    assert!(ins.op_count() == 2);

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 128 && sz1 == 64 {
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value0");
                return false;
            }
        };

        let value1 = match emu.get_operand_value(ins, 0, true) {
            Some(v) => v,
            None => {
                log::info!("error getting value1");
                return false;
            }
        };

        let lower_value0 = value0 & 0x00000000_FFFFFFFF_00000000_FFFFFFFF;
        let upper_value1 = (value1 as u128) << 64;
        let result = lower_value0 | upper_value1;

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else if sz0 == 64 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value1");
                return false;
            }
        };

        let result = (value1 >> 64) as u64;

        emu.set_operand_value(ins, 0, result);
    } else if sz0 == 128 && sz1 == 32 {
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value0");
                return false;
            }
        };

        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => (v & 0xffffffff) as u32,
            None => {
                log::info!("error getting value1");
                return false;
            }
        };

        let lower_value0 = value0 & 0x00000000_FFFFFFFF_FFFFFFFF_FFFFFFFF;
        let upper_value1 = (value1 as u128) << 96;
        let result = lower_value0 | upper_value1;

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else {
        unimplemented!("case of movhps unimplemented {} {}", sz0, sz1);
    }
    true
}
