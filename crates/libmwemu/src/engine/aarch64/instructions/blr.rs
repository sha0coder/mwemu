use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::Instruction;

use super::super::helpers::*;

pub fn execute(emu: &mut Emu, ins: &Instruction) -> bool {
    let pc = emu.regs_aarch64().pc;
    let target = read_reg(emu, &ins.operands[0]);
    emu.regs_aarch64_mut().x[30] = pc + 4;
    emu.regs_aarch64_mut().pc = target;
    emu.force_reload = true;
    true
}
