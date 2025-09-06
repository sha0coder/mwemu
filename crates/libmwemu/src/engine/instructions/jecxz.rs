use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    assert!(ins.op_count() == 1);

    if emu.regs_mut().get_cx() == 0 {
        emu.show_instruction_taken(color!("Orange"), ins);
        let addr = match emu.get_jump_value(ins, 0) {
            Some(v) => v,
            None => return false,
        };

        if emu.cfg.is_64bits {
            return emu.set_rip(addr, true);
        } else {
            return emu.set_eip(addr, true);
        }
    } else {
        emu.show_instruction_not_taken(color!("Orange"), ins);
    }
    true
}
