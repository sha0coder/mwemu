use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Cyan"), ins);
    //TODO: crash if arrive to zero or max value

    if emu.cfg.is_64bits {
        let val = match emu.maps.read_qword(emu.regs().rsi) {
            Some(v) => v,
            None => panic!("lodsq: memory read error"),
        };

        emu.regs_mut().rax = val;
        if emu.flags().f_df {
            emu.regs_mut().rsi -= 8;
        } else {
            emu.regs_mut().rsi += 8;
        }
    } else {
        unreachable!("lodsq dont exists in 32bit");
    }
    true
}
