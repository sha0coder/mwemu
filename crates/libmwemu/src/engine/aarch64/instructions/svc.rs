use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Operand};

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let imm = match ins.operands[0] {
        Operand::Immediate(v) => v as u64,
        Operand::Imm16(v) => v as u64,
        _ => 0,
    };
    log::info!(
        "SVC #{} at 0x{:x} (x8=0x{:x} x0=0x{:x})",
        imm,
        emu.regs_aarch64().pc,
        emu.regs_aarch64().x[8],
        emu.regs_aarch64().x[0],
    );
    // TODO: Linux syscall dispatch based on x8
    true
}
