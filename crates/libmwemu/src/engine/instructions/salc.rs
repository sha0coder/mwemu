use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    if emu.flags().f_cf {
        emu.regs_mut().set_al(1);
    } else {
        emu.regs_mut().set_al(0);
    }
    true
}
