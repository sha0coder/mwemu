use crate::emu;

pub fn CopyFileW(emu: &mut emu::Emu) {
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CopyFileW cannot read src_ptr") as u64;
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CopyFileW cannot read dst_ptr") as u64;
    let do_fail = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!CopyFileW cannot read do_fail");

    let src = emu.maps.read_wide_string(src_ptr);
    let dst = emu.maps.read_wide_string(dst_ptr);

    log::info!(
        "{}** {} kernel32!CopyFileW `{}` to `{}` {}",
        emu.colors.light_red,
        emu.pos,
        src,
        dst,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}