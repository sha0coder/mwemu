use crate::constants;
use crate::emu;
use crate::maps::mem64::Permission;

const PAGE_NOACCESS: u32 = 0x01;
const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_WRITECOPY: u32 = 0x08;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
const PAGE_GUARD: u32 = 0x100;
const PAGE_NOCACHE: u32 = 0x200;

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
    let can_read = (protect
        & (PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_write = (protect
        & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_execute = (protect
        & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

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
            .create_map(
                format!("alloc_{:x}", base).as_str(),
                base,
                size,
                Permission::from_flags(can_read, can_write, can_execute),
            )
            .expect("kernel32!VirtualAlloc out of memory");
    } else {
        if mem_commit && emu.maps.is_allocated(addr) {
            base = addr
        } else {
            base = 0
        }
    };

    log_red!(
        emu,
        "kernel32!VirtualAlloc sz: {} addr: 0x{:x} mem_reserve: {}",
        size,
        base,
        mem_reserve
    );

    emu.regs_mut().rax = base;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
