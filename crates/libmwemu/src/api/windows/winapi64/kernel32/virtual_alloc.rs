use crate::maps::mem64::Permission;
use crate::winapi::winapi64::kernel32::set_last_error;
use crate::windows::constants;
use crate::{emu, emu::Emu};

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

fn permissions(prot: u32) -> (bool, bool, bool) {
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
    (can_read, can_write, can_execute)
}

fn round_up(size: u64) -> u64 {
    const PAGE_SIZE: u64 = 0x1000;
    (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

fn fail(emu: &mut Emu, label: &str, addr: u64, size: u64, orig_size: u64, typ: u32, prot: u32, reason: &str) {
    log_red!(
        emu,
        "kernel32!{} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0 reason: {}",
        label,
        addr,
        orig_size,
        size,
        typ,
        prot,
        reason
    );
    set_last_error(constants::ERROR_INVALID_PARAMETER);
    emu.regs_mut().rax = 0;
}

fn fail_oom(emu: &mut Emu, label: &str, addr: u64, size: u64, orig_size: u64, typ: u32, prot: u32) {
    log_red!(
        emu,
        "kernel32!{} addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0 reason: out of memory",
        label,
        addr,
        orig_size,
        size,
        typ,
        prot
    );
    set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY);
    emu.regs_mut().rax = 0;
}

pub fn VirtualAlloc(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let orig_size = emu.regs().rdx;
    let typ = emu.regs().r8 as u32;
    let prot = emu.regs().r9 as u32;
    let size = round_up(orig_size);
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let (can_read, can_write, can_execute) = permissions(prot);

    if orig_size == 0 {
        fail(emu, "VirtualAlloc", addr, size, orig_size, typ, prot, "zero size");
        return;
    }

    if !mem_reserve && !mem_commit {
        fail(
            emu,
            "VirtualAlloc",
            addr,
            size,
            orig_size,
            typ,
            prot,
            "unsupported allocation type",
        );
        return;
    }

    if mem_commit && !mem_reserve && addr > 0 && !emu.maps.is_allocated(addr) {
        fail(
            emu,
            "VirtualAlloc",
            addr,
            size,
            orig_size,
            typ,
            prot,
            "commit target unmapped",
        );
        return;
    }

    let base: u64 = if mem_commit && !mem_reserve && addr > 0 {
        addr
    } else if addr > 0 {
        if emu.maps.is_allocated(addr) {
            fail(
                emu,
                "VirtualAlloc",
                addr,
                size,
                orig_size,
                typ,
                prot,
                "address already mapped",
            );
            return;
        }
        addr
    } else {
        match emu.maps.alloc(size) {
            Some(b) => b,
            None => {
                fail_oom(emu, "VirtualAlloc", addr, size, orig_size, typ, prot);
                return;
            }
        }
    };

    if mem_commit && !mem_reserve {
        set_last_error(constants::ERROR_SUCCESS);
        log_red!(
            emu,
            "kernel32!VirtualAlloc addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0x{:x}",
            addr,
            orig_size,
            size,
            typ,
            prot,
            base
        );
        emu.regs_mut().rax = base;
        return;
    }

    if let Err(err) = emu.maps.create_map(
        format!("alloc_{:x}", base).as_str(),
        base,
        size,
        Permission::from_flags(can_read, can_write, can_execute),
    ) {
        log_red!(
            emu,
            "kernel32!VirtualAlloc addr: 0x{:x} sz: {} flags: {} prot: {} = 0 reason: create_map failed: {}",
            addr,
            size,
            typ,
            prot,
            err
        );
        set_last_error(constants::ERROR_NOT_ENOUGH_MEMORY);
        emu.regs_mut().rax = 0;
        return;
    }

    set_last_error(constants::ERROR_SUCCESS);

    log_red!(
        emu,
        "kernel32!VirtualAlloc addr: 0x{:x} sz: {} (rounded {}) flags: {} prot: {} = 0x{:x}",
        addr,
        orig_size,
        size,
        typ,
        prot,
        base
    );
    emu.regs_mut().rax = base;
}
