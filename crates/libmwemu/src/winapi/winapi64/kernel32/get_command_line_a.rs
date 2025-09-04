use crate::{constants, emu};
use crate::maps::mem64::Permission;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} kernel32!GetCommandLineA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    let addr = emu.maps.alloc(1024).expect("out of memory");
    let name = format!("alloc_{:x}", addr);
    emu.maps.create_map(&name, addr, 1024, Permission::READ_WRITE);
    emu.maps.write_string(addr, constants::EXE_NAME);
    emu.regs_mut().rax = addr;
}