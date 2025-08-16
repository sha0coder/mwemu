use crate::emu;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!TlsAlloc {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.tls32_mut().push(0);
    let tls_len = emu.tls32().len() as u64;
    emu.regs_mut().set_eax(tls_len);
}