
use crate::emu;

pub fn FlsGetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx;
    if idx as usize > emu.fls().len() {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = emu.fls()[idx as usize] as u64;
    }

    log::info!(
        "{}** {} kernel32!FlsGetValue idx: {} =0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        idx,
        emu.regs().get_eax() as u32,
        emu.colors.nc
    );
}