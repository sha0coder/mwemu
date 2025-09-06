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
        if !emu
            .maps
            .write_byte(emu.regs().rdi, emu.regs().get_al() as u8)
        {
            return false;
        }
        if emu.flags().f_df {
            emu.regs_mut().rdi -= 1;
        } else {
            emu.regs_mut().rdi += 1;
        }
    } else {
        // 32bits
        if !emu
            .maps
            .write_byte(emu.regs().get_edi(), emu.regs().get_al() as u8)
        {
            return false;
        }
        if emu.flags().f_df {
            let edi = emu.regs().get_edi() - 1;
            emu.regs_mut().set_edi(edi);
        } else {
            let edi = emu.regs().get_edi() + 1;
            emu.regs_mut().set_edi(edi);
        }
    }
    true
}
