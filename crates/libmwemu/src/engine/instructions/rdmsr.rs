use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Red"), ins);

    match emu.regs().rcx {
        0x176 => {
            emu.regs_mut().rdx = 0;
            emu.regs_mut().rax = emu.cfg.code_base_addr + 0x42;
        }
        _ => {
            log::info!("/!\\ unimplemented rdmsr with value {}", emu.regs().rcx);
            return false;
        }
    }
    true
}
