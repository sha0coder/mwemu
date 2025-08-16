use crate::emu;
use crate::winapi::helper;

pub fn OpenThread(emu: &mut emu::Emu) {
    let access = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!OpenThread cannot read acess");
    let inherit = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!OpenThread cannot read inherit");
    let tid = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!OpenThread cannot read tid");

    log::info!(
        "{}** {} kernel32!OpenThread tid: {} {}",
        emu.colors.light_red,
        emu.pos,
        tid,
        emu.colors.nc
    );

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    let uri = format!("tid://{}", tid);
    emu.regs_mut().rax = helper::handler_create(&uri);
}