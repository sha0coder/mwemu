use crate::emu;

pub fn MoveFileA(emu: &mut emu::Emu) {
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!MoveFileA cannot read src_ptr") as u64;
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!MoveFileA cannot read dst_ptr") as u64;

    let src = emu.maps.read_string(src_ptr);
    let dst = emu.maps.read_string(dst_ptr);

    log::info!(
        "{}** {} kernel32!MoveFileA `{}` to `{}` {}",
        emu.colors.light_red,
        emu.pos,
        src,
        dst,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}