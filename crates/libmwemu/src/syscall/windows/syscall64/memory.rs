use crate::constants::*;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::structures::MemoryBasicInformation;

fn nt_page_protection_to_permission(protect: u32) -> Permission {
    const PAGE_READONLY: u32 = 0x02;
    const PAGE_READWRITE: u32 = 0x04;
    const PAGE_WRITECOPY: u32 = 0x08;
    const PAGE_EXECUTE: u32 = 0x10;
    const PAGE_EXECUTE_READ: u32 = 0x20;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;

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

    Permission::from_flags(can_read, can_write, can_execute)
}

fn align_up_page(size: u64) -> u64 {
    const PAGE: u64 = 0x1000;
    size.saturating_add(PAGE - 1) & !(PAGE - 1)
}

fn is_current_process_handle(h: u64) -> bool {
    h == !0 || h == 0xffff_ffff_ffff_fffe
}

/// `NtQueryVirtualMemory` — x64 register/stack layout matches the ntdll syscall stub
/// (RCX..R9 + 5th/6th on stack at `rsp+0x28` / `rsp+0x30`).
pub fn nt_query_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_address = emu.regs().rdx;
    let memory_information_class = emu.regs().r8;
    let memory_information = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let memory_information_length = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let return_length_ptr = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    log_red!(
        emu,
        "NtQueryVirtualMemory process: 0x{:x}, base: 0x{:x}, class: 0x{:x}, out: 0x{:x}, len: 0x{:x}, ret_len_ptr: 0x{:x}",
        process_handle,
        base_address,
        memory_information_class,
        memory_information,
        memory_information_length,
        return_length_ptr
    );

    if memory_information_class != MEMORY_INFORMATION_CLASS_MEMORY_BASIC_INFORMATION
        && memory_information_class != MEMORY_INFORMATION_CLASS_MEMORY_PRIVILEGED_BASIC_INFORMATION
    {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if memory_information == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if memory_information_length < MemoryBasicInformation::size() {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if !emu.maps.is_mapped(base_address) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if !emu.maps.is_mapped(memory_information) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.get_addr_base(base_address).unwrap_or(0);
    let mut mem_info = MemoryBasicInformation::load(memory_information, &emu.maps);
    mem_info.base_address = base as u32;
    mem_info.allocation_base = base as u32;
    mem_info.allocation_protect = PAGE_EXECUTE | PAGE_READWRITE;
    mem_info.state = MEM_COMMIT;
    mem_info.typ = MEM_PRIVATE;
    if let Some(mem) = emu.maps.get_mem_by_addr(base_address) {
        mem_info.region_size = mem.size() as u32;
    }
    mem_info.protect = mem_info.allocation_protect;

    mem_info.save(memory_information, &mut emu.maps);

    if return_length_ptr != 0 {
        if !emu
            .maps
            .write_qword(return_length_ptr, MemoryBasicInformation::size())
        {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAllocateVirtualMemory` — x64: RCX..R9 + 5th/6th at `[rsp+0x28]` / `[rsp+0x30]`.
pub fn nt_allocate_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let _zero_bits = emu.regs().r8;
    let region_sz_ptr = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let alloc_type = emu.maps.read_dword(rsp + 0x28).unwrap_or(0);
    let protect = emu.maps.read_dword(rsp + 0x30).unwrap_or(0);

    log_red!(
        emu,
        "NtAllocateVirtualMemory h: 0x{:x} base_ptr: 0x{:x} type: 0x{:x} prot: 0x{:x}",
        process_handle,
        base_ptr,
        alloc_type,
        protect
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 || region_sz_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let mut size = match emu.maps.read_qword(region_sz_ptr) {
        Some(s) => s,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    size = align_up_page(size);

    let preferred_base = match emu.maps.read_qword(base_ptr) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    let mem_commit = (alloc_type & MEM_COMMIT) != 0;
    let mem_reserve = (alloc_type & MEM_RESERVE) != 0;
    if !mem_commit && !mem_reserve {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let permission = nt_page_protection_to_permission(protect);

    let base = if preferred_base == 0 {
        match emu.maps.alloc(size) {
            Some(a) => a,
            None => {
                emu.regs_mut().rax = STATUS_NO_MEMORY;
                return;
            }
        }
    } else {
        preferred_base
    };

    if !emu.maps.write_qword(base_ptr, base) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let name = format!("alloc_{:x}", base);
    match emu.maps.create_map(&name, base, size, permission) {
        Ok(_) => {}
        Err(_) => {
            if !emu.maps.is_mapped(base) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtFreeVirtualMemory` — x64: RCX, RDX, R8, R9 (all in registers).
pub fn nt_free_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let free_type = emu.regs().r9 as u32;

    log_red!(
        emu,
        "NtFreeVirtualMemory h: 0x{:x} base_ptr: 0x{:x} free_type: 0x{:x}",
        process_handle,
        base_ptr,
        free_type
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let base = match emu.maps.read_qword(base_ptr) {
        Some(b) => b,
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if (free_type & MEM_RELEASE) != 0 {
        emu.maps.dealloc(base);
        let _ = emu.maps.write_qword(base_ptr, 0);
        if region_sz_ptr != 0 {
            let _ = emu.maps.write_qword(region_sz_ptr, 0);
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtProtectVirtualMemory` — x64: RCX..R9 + 5th (`OldProtect`) at `[rsp+0x28]`.
pub fn nt_protect_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let new_protect = emu.regs().r9 as u32;
    let rsp = emu.regs().rsp;
    let old_protect_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "NtProtectVirtualMemory h: 0x{:x} base_ptr: 0x{:x} new_prot: 0x{:x}",
        process_handle,
        base_ptr,
        new_protect
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if base_ptr == 0 || region_sz_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.read_qword(base_ptr).unwrap_or(0);
    let _region_sz = emu.maps.read_qword(region_sz_ptr).unwrap_or(0);

    if old_protect_ptr != 0 {
        let _ = emu.maps.write_dword(old_protect_ptr as u64, new_protect);
    }

    if let Some(mem) = emu.maps.get_mem_by_addr_mut(base) {
        mem.set_permission(nt_page_protection_to_permission(new_protect));
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtReadVirtualMemory` — x64: 5th arg (`NumberOfBytesRead`) at `[rsp+0x28]`.
pub fn nt_read_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let size = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let bytes_read_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "NtReadVirtualMemory h: 0x{:x} from: 0x{:x} to: 0x{:x} len: 0x{:x}",
        process_handle,
        base,
        buffer,
        size
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if buffer == 0 || size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let sz = size.min(usize::MAX as u64) as usize;
    let data = match emu.maps.try_read_bytes(base, sz) {
        Some(s) => s.to_vec(),
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if !emu.maps.write_bytes(buffer, &data) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if bytes_read_ptr != 0 {
        let _ = emu.maps.write_qword(bytes_read_ptr, data.len() as u64);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtWriteVirtualMemory` — x64: 5th arg (`NumberOfBytesWritten`) at `[rsp+0x28]`.
pub fn nt_write_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let size = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let bytes_written_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_red!(
        emu,
        "NtWriteVirtualMemory h: 0x{:x} to: 0x{:x} from: 0x{:x} len: 0x{:x}",
        process_handle,
        base,
        buffer,
        size
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }
    if buffer == 0 || size == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let sz = size.min(usize::MAX as u64) as usize;
    let data = match emu.maps.try_read_bytes(buffer, sz) {
        Some(s) => s.to_vec(),
        None => {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    };

    if !emu.maps.write_bytes(base, &data) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if bytes_written_ptr != 0 {
        let _ = emu.maps.write_qword(bytes_written_ptr, data.len() as u64);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtUnmapViewOfSection` — stub: succeed if the region is not tracked as a section view.
pub fn nt_unmap_view_of_section(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base = emu.regs().rdx;

    log_red!(
        emu,
        "NtUnmapViewOfSection h: 0x{:x} base: 0x{:x}",
        process_handle,
        base
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtMapViewOfSection` — minimal stub (many args on stack); returns success.
pub fn nt_map_view_of_section(emu: &mut Emu) {
    log_red!(emu, "NtMapViewOfSection (stub)");
    emu.regs_mut().rax = STATUS_SUCCESS;
}
