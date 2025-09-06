use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    assert!(emu.cfg.is_64bits);

    if emu.rep.is_some() {
        if emu.rep.unwrap() == 0 || emu.cfg.verbose >= 3 {
            emu.show_instruction(color!("LightCyan"), ins);
        }
    } else {
        emu.show_instruction(color!("LightCyan"), ins);
    }

    emu.maps.write_qword(emu.regs().rdi, emu.regs().rax);

    if emu.flags().f_df {
        emu.regs_mut().rdi -= 8;
    } else {
        emu.regs_mut().rdi += 8;
    }
    true
}
