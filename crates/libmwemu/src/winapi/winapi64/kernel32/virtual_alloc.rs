use crate::{constants, emu};

pub fn VirtualAlloc(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let typ = emu.regs().r8 as u32;
    let prot = emu.regs().r9 as u32;
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let mut base:u64 = 0;

    if size == 0 {
        log::info!(
            "{}** {} kernel32!VirtualAlloc addr: 0x{:x} sz: {} = 0 flags: {} {}",
            emu.colors.light_red,
            emu.pos,
            addr,
            size,
            typ,
            emu.colors.nc
        );
        emu.regs_mut().rax = 0
    } else {

        let is_allocated = emu.maps.is_allocated(addr);
        let status_already_allocated = mem_commit && addr > 0 && is_allocated;
        let status_error = !status_already_allocated && mem_commit && addr > 0 && !is_allocated;
        let status_need_allocate = mem_reserve || (mem_commit && addr == 0);
        let status_other = !status_already_allocated && !status_error && !status_need_allocate;


        if status_need_allocate {
            if addr == 0 {
                base = emu
                    .maps
                    .alloc(size)
                    .unwrap_or_else(|| panic!("kernel32!VirtualAlloc out of memory size:{}", size));
            } else {
                base = addr;
            }

            emu.maps
                .create_map(format!("alloc_{:x}", base).as_str(), base, size)
                .expect("kernel32!VirtualAlloc out of memory");

        } else if status_already_allocated {
            base = addr;
        } else if status_error {
            base = 0;
        } else if status_other {
            log::info!("Weird flags on VirtualAlloc");
            base = 0;
        }

        log::info!(
            "{}** {} kernel32!VirtualAlloc addr: 0x{:x} sz: {}  flags: {} =0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            addr,
            size,
            typ,
            base,
            emu.colors.nc
        );

        emu.regs_mut().rax = base;
    }
}