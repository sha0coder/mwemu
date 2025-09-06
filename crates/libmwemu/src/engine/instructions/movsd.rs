use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    if ins.op_count() == 2
        && (emu.get_operand_sz(ins, 0) == 128 || emu.get_operand_sz(ins, 1) == 128)
    {
        emu.show_instruction(color!("LightCyan"), ins);
        let src = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v & 0xffffffff_ffffffff,
            None => return false,
        };

        let mut dst = match emu.get_operand_xmm_value_128(ins, 1, true) {
            Some(v) => v,
            None => return false,
        };

        dst = (dst & 0xffffffff_ffffffff_00000000_00000000) | src;

        emu.set_operand_xmm_value_128(ins, 0, dst);
    } else {
        // legacy mode of movsd

        if emu.rep.is_some() {
            if emu.rep.unwrap() == 0 {
                emu.show_instruction(color!("LightCyan"), ins);
            }
        } else {
            emu.show_instruction(color!("LightCyan"), ins);
        }

        if emu.cfg.is_64bits {
            let val = emu
                .maps
                .read_dword(emu.regs().rsi)
                .expect("cannot read memory");

            emu.maps.write_dword(emu.regs().rdi, val);

            if !emu.flags().f_df {
                emu.regs_mut().rsi += 4;
                emu.regs_mut().rdi += 4;
            } else {
                emu.regs_mut().rsi -= 4;
                emu.regs_mut().rdi -= 4;
            }
        } else {
            // 32bits

            let val = match emu.maps.read_dword(emu.regs().get_esi()) {
                Some(v) => v,
                None => {
                    log::info!("cannot read memory at esi");
                    return false;
                }
            };
            emu.maps.write_dword(emu.regs().get_edi(), val);

            if !emu.flags().f_df {
                let esi = emu.regs().get_esi() + 4;
                let edi = emu.regs().get_edi() + 4;
                emu.regs_mut().set_esi(esi);
                emu.regs_mut().set_edi(edi);
            } else {
                let esi = emu.regs().get_esi() - 4;
                let edi = emu.regs().get_edi() - 4;
                emu.regs_mut().set_esi(esi);
                emu.regs_mut().set_edi(edi);
            }
        }
    }
    true
}
