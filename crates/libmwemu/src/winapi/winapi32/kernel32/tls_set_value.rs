use crate::emu;

pub fn TlsSetValue(emu: &mut emu::Emu) {
    let idx = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!TlsSetValue cannot read idx");
    let val = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!TlsSetValue cannot read val_ptr");

    log::info!(
        "{}** {} kernel32!TlsSetValue idx: {} val: 0x{:x} {}",
        emu.colors.light_red,
        emu.pos,
        idx,
        val,
        emu.colors.nc
    );

    if emu.tls32().len() > idx as usize {
        emu.tls32_mut()[idx as usize] = val;
    } else {
        for _ in 0..=idx {
            emu.tls32_mut().push(0);
        }
        emu.tls32_mut()[idx as usize] = val;
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}