use crate::emu;
use crate::winapi::helper;

fn parse_resource_uri(uri: &str) -> Option<(u64, usize)> {
    let mut parts = uri.splitn(3, '_');
    parts.next()?;
    let rva = u64::from_str_radix(parts.next()?, 16).ok()?;
    let size = parts.next()?.parse::<usize>().ok()?;
    Some((rva, size))
}

pub fn LockResource(emu: &mut emu::Emu) {
    let hResData = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    emu.stack_pop32(false);

    if helper::handler_exist(hResData) {
        let uri = helper::handler_get_uri(hResData);
        let ptr = match parse_resource_uri(&uri) {
            Some((rva, _size)) => rva.saturating_add(emu.base as u64),
            None => 0,
        };

        log_red!(
            emu,
            "** {} kernel32!LockResource {:x} {:x}",
            emu.pos,
            hResData,
            ptr
        );
        emu.regs_mut().rax = ptr;
        return;
    }

    log_red!(
        emu,
        "** {} kernel32!LockResource {:x} not found",
        emu.pos,
        hResData
    );

    emu.regs_mut().rax = 0;
}
