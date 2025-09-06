use crate::maps::mem64::Permission;
use crate::{constants, emu};

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
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let typ = emu.regs().r8 as u32;
    let prot = emu.regs().r9 as u32;
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let mut base: u64 = 0;
    let can_read = (prot
        & (PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_write = (prot
        & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_execute = (prot
        & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    if size == 0 {
        log_red!(
            emu,
            "kernel32!VirtualAlloc addr: 0x{:x} sz: {} = 0 flags: {}",
            addr,
            size,
            typ
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
                .create_map(
                    format!("alloc_{:x}", base).as_str(),
                    base,
                    size,
                    Permission::from_flags(can_read, can_write, can_execute),
                )
                .expect("kernel32!VirtualAlloc out of memory");
        } else if status_already_allocated {
            base = addr;
        } else if status_error {
            base = 0;
        } else if status_other {
            log::info!("Weird flags on VirtualAlloc");
            base = 0;
        }

        log_red!(
            emu,
            "kernel32!VirtualAlloc addr: 0x{:x} sz: {}  flags: {} =0x{:x}",
            addr,
            size,
            typ,
            base
        );

        emu.regs_mut().rax = base;
    }
}
