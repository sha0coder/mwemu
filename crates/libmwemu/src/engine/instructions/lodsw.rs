use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), ins);
    //TODO: crash if rsi arrive to zero or max value

    if emu.cfg.is_64bits {
        let val = match emu.maps.read_word(emu.regs().rsi) {
            Some(v) => v,
            None => return false,
        };

        emu.regs_mut().set_ax(val as u64);
        if emu.flags().f_df {
            emu.regs_mut().rsi -= 2;
        } else {
            emu.regs_mut().rsi += 2;
        }
    } else {
        let val = match emu.maps.read_word(emu.regs().get_esi()) {
            Some(v) => v,
            None => return false,
        };

        emu.regs_mut().set_ax(val as u64);
        if emu.flags().f_df {
            let esi = emu.regs().get_esi() - 2;
            emu.regs_mut().set_esi(esi);
        } else {
            let esi = emu.regs().get_esi() + 2;
            emu.regs_mut().set_esi(esi);
        }
    }
    true
}
