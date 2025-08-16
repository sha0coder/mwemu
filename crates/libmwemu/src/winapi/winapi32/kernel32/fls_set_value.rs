use crate::emu;

pub fn FlsSetValue(emu: &mut emu::Emu) {
    let idx = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FlsSetValue cannot read index");
    let val = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FlsSetValue cannot read value");

    log::info!(
        "{}** {} kernel32!FlsSetValue idx: {} val: {} {}",
        emu.colors.light_red,
        emu.pos,
        idx,
        val,
        emu.colors.nc
    );

    if emu.fls().len() > idx as usize {
        emu.fls_mut()[idx as usize] = val;
    } else {
        for _ in 0..=idx {
            emu.fls_mut().push(0);
        }
        emu.fls_mut()[idx as usize] = val;
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}