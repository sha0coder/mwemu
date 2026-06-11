use crate::emu;
use crate::winapi::helper;

fn parse_resource_size(uri: &str) -> Option<usize> {
    let mut parts = uri.splitn(3, '_');
    parts.next()?;
    parts.next()?;
    parts.next()?.parse::<usize>().ok()
}

pub fn SizeofResource(emu: &mut emu::Emu) {
    let hModule = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let hResInfo = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    if helper::handler_exist(hResInfo) {
        let uri = helper::handler_get_uri(hResInfo);
        let size = parse_resource_size(&uri).unwrap_or(0);
        log::trace!(
            "** {} kernel32!SizeofResource {:x} {:x} size: {}",
            emu.pos,
            hModule,
            hResInfo,
            size
        );
        emu.regs_mut().rax = size as u64;
        return;
    }

    log_red!(
        emu,
        "** {} kernel32!SizeofResource {:x} {:x} not found",
        emu.pos,
        hModule,
        hResInfo
    );

    emu.regs_mut().rax = 0;
}
