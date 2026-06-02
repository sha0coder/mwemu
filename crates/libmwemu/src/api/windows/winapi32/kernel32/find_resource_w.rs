use crate::emu;
use crate::winapi::helper;

pub fn FindResourceW(emu: &mut emu::Emu) {
    let hModule = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as usize;
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let lpType = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as usize;

    let x: Option<(u64, usize)>;

    if lpName > 0xff && lpType > 0xff {
        let name = emu.maps.read_wide_string(lpName as u64);
        let ntype = emu.maps.read_wide_string(lpType as u64);
        log_red!(
            emu,
            "** {} kernel32!FindResourceW {:x} `{}` `{}`",
            emu.pos,
            hModule,
            name,
            ntype
        );

        x = emu
            .pe32
            .as_ref()
            .unwrap()
            .get_resource(None, None, Some(&ntype), Some(&name));
    } else if lpName > 0xff && lpType <= 0xff {
        let name = emu.maps.read_wide_string(lpName as u64);
        log_red!(
            emu,
            "** {} kernel32!FindResourceW {:x} `{}` {}",
            emu.pos,
            hModule,
            name,
            lpType
        );

        x = emu
            .pe32
            .as_ref()
            .unwrap()
            .get_resource(Some(lpType as u32), None, None, Some(&name));
    } else if lpName <= 0xff && lpType > 0xff {
        let ntype = emu.maps.read_wide_string(lpType as u64);
        log_red!(
            emu,
            "** {} kernel32!FindResourceW {:x} `{}` {}",
            emu.pos,
            hModule,
            lpName,
            ntype
        );

        x = emu
            .pe32
            .as_ref()
            .unwrap()
            .get_resource(None, Some(lpName as u32), Some(&ntype), None);
    } else if lpName <= 0xff && lpType <= 0xff {
        log_red!(
            emu,
            "** {} kernel32!FindResourceW {:x} `{}` {}",
            emu.pos,
            hModule,
            lpName,
            lpType
        );

        x = emu.pe32.as_ref().unwrap().get_resource(
            Some(lpType as u32),
            Some(lpName as u32),
            None,
            None,
        );
    } else {
        unreachable!();
    }

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    if x.is_none() {
        log::trace!("{} resource not found!", emu.pos);
        emu.regs_mut().rax = 0;
        return;
    }

    let (rva, size) = x.unwrap();
    log::trace!("resource rva: 0x{:x} sz: {}", rva, size);
    let hndl = helper::handler_create(&format!("rsrc_{:x}_{}", rva, size));
    emu.regs_mut().rax = hndl;
}
