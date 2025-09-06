use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    // we keep the high part of xmm destination

    emu.show_instruction(color!("Cyan"), ins);

    let sz0 = emu.get_operand_sz(ins, 0);
    let sz1 = emu.get_operand_sz(ins, 1);

    if sz0 == 128 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value1");
                return false;
            }
        };
        emu.set_operand_xmm_value_128(ins, 0, value1);
    } else if sz0 == 128 && sz1 == 32 {
        let value1 = match emu.get_operand_value(ins, 1, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value1");
                return false;
            }
        };
        unimplemented!("mov 32bits to the 64bits highest part of the xmm1 u128");
        //emu.set_operand_xmm_value_128(&ins, 0, value1 as u128);
    } else if sz0 == 32 && sz1 == 128 {
        let value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value1");
                return false;
            }
        };
        unimplemented!("mov 32bits to the 64bits highest part of the xmm1 u128");
        //emu.set_operand_value(&ins, 0, value1 as u64);
    } else if sz0 == 128 && sz1 == 64 {
        let value0 = match emu.get_operand_xmm_value_128(ins, 0, false) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm address value1");
                return false;
            }
        };
        let addr = match emu.get_operand_value(ins, 1, false) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm address value1");
                return false;
            }
        };
        let value1 = match emu.maps.read_qword(addr) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm qword value1");
                return false;
            }
        };

        let result: u128 = (value1 as u128) << 64 | value0 & 0xffffffffffffffff;

        emu.set_operand_xmm_value_128(ins, 0, result);
    } else if sz0 == 64 && sz1 == 128 {
        let mut value1 = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => {
                log::info!("error getting xmm value1");
                return false;
            }
        };
        value1 >>= 64;

        emu.set_operand_value(ins, 0, value1 as u64);
    } else {
        log::info!("SSE with other size combinations sz0:{} sz1:{}", sz0, sz1);
        return false;
    }
    true
}
