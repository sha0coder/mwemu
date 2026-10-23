use crate::color;
use crate::emu::Emu;
use iced_x86::Instruction;

pub fn execute(emu: &mut Emu, ins: &Instruction, instruction_sz: usize, _rep_step: bool) -> bool {
    emu.show_instruction(
        color!("Cyan"),
        &crate::emu::decoded_instruction::DecodedInstruction::X86(*ins),
    );
    //TODO: crash if arrive to zero or max value

    if emu.cfg.is_x64() {
        let val = match emu.maps.read_qword(emu.regs().rsi) {
            Some(v) => v,
            None => {
                log::warn!("lodsq: memory read error");
                return false;
            }
        };

        emu.regs_mut().rax = val;
        if emu.flag_df() {
            emu.regs_mut().rsi -= 8;
        } else {
            emu.regs_mut().rsi += 8;
        }
    } else {
        log::warn!("lodsq: not available in 32-bit mode");
        return false;
    }
    true
}
