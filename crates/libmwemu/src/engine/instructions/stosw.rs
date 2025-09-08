use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("LightCyan"), ins);

    if emu.cfg.is_64bits {
        emu.maps
            .write_word(emu.regs().rdi, emu.regs().get_ax() as u16);

        if emu.flags().f_df {
            emu.regs_mut().rdi -= 2;
        } else {
            emu.regs_mut().rdi += 2;
        }
    } else {
        // 32bits
        emu.maps
            .write_word(emu.regs().get_edi(), emu.regs().get_ax() as u16);

        if emu.flags().f_df {
            let edi = emu.regs().get_edi() - 2;
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi() + 2;
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
