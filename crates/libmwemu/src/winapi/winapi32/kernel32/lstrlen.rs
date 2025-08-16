use crate::emu;

pub fn lstrlen(emu: &mut emu::Emu) {
    let s_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!lstrlen cannot read string") as u64;

    emu.stack_pop32(false);
    let s = emu.maps.read_string(s_ptr);
    let len = s.len() as u64;

    log::info!(
        "{}** {} kernel32!lstrlen '{}' ={} {}",
        emu.colors.light_red,
        emu.pos,
        s,
        len,
        emu.colors.nc
    );

    emu.regs_mut().rax = len;
}