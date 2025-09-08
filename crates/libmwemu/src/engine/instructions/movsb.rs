use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.rep.is_some() {
        if emu.rep.unwrap() == 0 || emu.cfg.verbose >= 3 {
            emu.show_instruction(color!("LightCyan"), ins);
        }
    } else {
        emu.show_instruction(color!("LightCyan"), ins);
    }

    if emu.cfg.is_64bits {
        let val = match emu.maps.read_byte(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::info!("cannot read memory on rsi");
                return false;
            }
        };
        if !emu.maps.write_byte(emu.regs().rdi, val) {
            log::info!("cannot write memoryh on rdi");
            return false;
        }

        if !emu.flags().f_df {
            emu.regs_mut().rsi += 1;
            emu.regs_mut().rdi += 1;
        } else {
            emu.regs_mut().rsi -= 1;
            emu.regs_mut().rdi -= 1;
        }
    } else {
        let val = match emu.maps.read_byte(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                log::info!("cannot read memory on esi");
                return false;
            }
        };
        if !emu.maps.write_byte(emu.regs().get_edi(), val) {
            log::info!("cannot write memory on edi");
            return false;
        }

        if !emu.flags().f_df {
            let esi = emu.regs().get_esi() + 1;
            let edi = emu.regs().get_edi() + 1;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi() - 1;
            let edi = emu.regs().get_edi() - 1;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
