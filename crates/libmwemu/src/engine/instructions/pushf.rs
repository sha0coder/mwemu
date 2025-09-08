use crate::emu::Emu;
use crate::{color, exception_type};
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(color!("Blue"), ins);

    let val: u16 = (emu.flags().dump() & 0xffff) as u16;

    emu.regs_mut().rsp -= 2;

    if !emu.maps.write_word(emu.regs().rsp, val) {
        log::info!("/!\\ exception writing word at rsp 0x{:x}", emu.regs().rsp);
        emu.exception(exception_type::ExceptionType::WritingWord);
        return false;
    }
    true
}
