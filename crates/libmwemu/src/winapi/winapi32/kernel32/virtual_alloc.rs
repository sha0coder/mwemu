use crate::emu;
use crate::constants;

pub fn VirtualAlloc(emu: &mut emu::Emu) {
    let addr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!VirtualAlloc error reading addr") as u64;
    let mut size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!VirtualAlloc error reading size ptr") as u64;
    let atype = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!VirtualAlloc error reading type");
    let protect = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!VirtualAlloc error reading protect");


    let mem_reserve = (atype & constants::MEM_RESERVE) != 0;
    let mem_commit = (atype & constants::MEM_COMMIT) != 0;
    let base: u64;
    let page_size = 0x1000;
    size = (size + page_size - 1) & !(page_size - 1);

    if mem_reserve {
        if mem_commit && addr > 0 {
            base = addr;
        } else {
            base = emu
                .maps
                .alloc(size)
                .expect("kernel32!VirtualAlloc out of memory");
        }
        emu.maps
            .create_map(format!("alloc_{:x}", base).as_str(), base, size)
            .expect("kernel32!VirtualAlloc out of memory");
    } else {
        if mem_commit && emu.maps.is_allocated(addr) {
            base = addr
        } else {
            base = 0
        }
    };

    log::info!(
        "{}** {} kernel32!VirtualAlloc sz: {} addr: 0x{:x} mem_reserve: {} {}",
        emu.colors.light_red,
        emu.pos,
        size,
        base,
        mem_reserve,
        emu.colors.nc
    );

    emu.regs_mut().rax = base;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
