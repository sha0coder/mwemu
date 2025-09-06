use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    let value0: u16;
    let value1: u16;

    if emu.rep.is_some() {
        if emu.rep.unwrap() == 0 || emu.cfg.verbose >= 3 {
            emu.show_instruction(color!("LightCyan"), ins);
        }
    } else {
        emu.show_instruction(color!("LightCyan"), ins);
    }

    if emu.cfg.is_64bits {
        value0 = match emu.maps.read_word(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::info!("cannot read rsi");
                return false;
            }
        };
        value1 = match emu.maps.read_word(emu.regs().rdi) {
            Some(v) => v,
            None => {
                log::info!("cannot read rdi");
                return false;
            }
        };

        if emu.flags().f_df {
            emu.regs_mut().rsi -= 2;
            emu.regs_mut().rdi -= 2;
        } else {
            emu.regs_mut().rsi += 2;
            emu.regs_mut().rdi += 2;
        }
    } else {
        // 32bits
        value0 = match emu.maps.read_word(emu.regs().get_esi()) {
            Some(v) => v,
            None => {
                log::info!("cannot read esi");
                return false;
            }
        };
        value1 = match emu.maps.read_word(emu.regs().get_edi()) {
            Some(v) => v,
            None => {
                log::info!("cannot read edi");
                return false;
            }
        };

        if emu.flags().f_df {
            let esi = emu.regs().get_esi() - 2;
            let edi = emu.regs().get_edi() - 2;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        } else {
            let esi = emu.regs().get_esi() + 2;
            let edi = emu.regs().get_edi() + 2;
            emu.regs_mut().set_esi(esi);
            emu.regs_mut().set_edi(edi);
        }
    }

    emu.flags_mut().sub16(value0 as u64, value1 as u64);

    if emu.cfg.verbose >= 2 {
        if value0 > value1 {
            log::info!("\tcmp: 0x{:x} > 0x{:x}", value0, value1);
        } else if value0 < value1 {
            log::info!("\tcmp: 0x{:x} < 0x{:x}", value0, value1);
        } else {
            log::info!("\tcmp: 0x{:x} == 0x{:x}", value0, value1);
        }
    }
    true
}
