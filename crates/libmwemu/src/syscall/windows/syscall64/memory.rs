use crate::windows::constants::*;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::windows::structures::MemoryBasicInformation64;
use iced_x86::{Instruction, Mnemonic, OpKind, Register};

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

/// ntdll heap expansion (`RtlAllocateHeap` internals) can leave a `LIST_ENTRY` at
/// `allocBase + 0xb000` zeroed; the next walk does `mov rax,[rsi]` / follow Flink and faults.
/// Prime a self-linked sentinel at that offset for large private regions (observed with
/// `maps64/ntdll.dll` during `LdrInitializeThunk` under `--ssdt --init`).
fn patch_ldr_heap_list_sentinel(emu: &mut Emu, base: u64, size: u64) {
    if !emu.cfg.ssdt_use_ldr_initialize_thunk {
        return;
    }
    const OFF: u64 = 0xb000;
    if size < OFF + 0x10 {
        return;
    }
    let p = base.saturating_add(OFF);
    if !emu.maps.is_mapped(p) || !emu.maps.is_mapped(p + 8) {
        return;
    }
    let _ = emu.maps.write_qword(p, p);
    let _ = emu.maps.write_qword(p + 8, p);
}

/// Runs immediately before `mov rax, qword ptr [rsi]` inside **ntdll** while `RSI` points at
/// emulated heap backing (`ALLOC64_*`).  ntdll `memset`s after `NtAllocateVirtualMemory*` and
/// clears Flink/Blink; self-link when both are still zero.
pub fn ntdll_heap_list_walk_fixup(emu: &mut Emu, ins: &Instruction, rip: u64) {
    if !emu.cfg.ssdt_use_ldr_initialize_thunk || !emu.cfg.emulate_winapi {
        return;
    }
    // Code lives in per-section maps (`ntdll.text`, …), not only the small `ntdll.pe` header map.
    let in_ntdll = emu
        .maps
        .get_mem_by_addr(rip)
        .map(|m| {
            let n = m.get_name();
            n == "ntdll.pe" || n.starts_with("ntdll.")
        })
        .unwrap_or(false);
    if !in_ntdll {
        return;
    }
    if ins.mnemonic() != Mnemonic::Mov || ins.op_count() < 2 {
        return;
    }
    if ins.op1_kind() != OpKind::Memory {
        return;
    }
    if ins.memory_base() != Register::RSI || ins.memory_index() != Register::None {
        return;
    }
    if ins.memory_displacement64() != 0 {
        return;
    }

    let rsi = emu.regs().rsi;
    if rsi < ALLOC64_MIN || rsi >= ALLOC64_MAX {
        return;
    }
    if !emu.maps.is_mapped(rsi) || !emu.maps.is_mapped(rsi + 8) {
        return;
    }
    if emu.maps.read_qword(rsi).unwrap_or(1) != 0
        || emu.maps.read_qword(rsi + 8).unwrap_or(1) != 0
    {
        return;
    }
    let _ = emu.maps.write_qword(rsi, rsi);
    let _ = emu.maps.write_qword(rsi + 8, rsi);
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

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryVirtualMemory process: 0x{:x}, base: 0x{:x}, class: 0x{:x}, out: 0x{:x}, len: 0x{:x}, ret_len_ptr: 0x{:x}",
        WIN64_NTQUERYVIRTUALMEMORY,
        process_handle,
        base_address,
        memory_information_class,
        memory_information,
        memory_information_length,
        return_length_ptr
    );

    if memory_information == 0 || !emu.maps.is_mapped(memory_information) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    // MemoryImageInformation (class 6): returns {ImageBase: u64, SizeOfImage: u64, ImageFlags: u32}
    if memory_information_class == MEMORY_INFORMATION_CLASS_MEMORY_IMAGE_INFORMATION {
        const MEMORY_IMAGE_INFO_SIZE: u64 = 0x18;
        if memory_information_length < MEMORY_IMAGE_INFO_SIZE {
            emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
            return;
        }

        match emu.maps.find_pe_image_info(base_address) {
            Some((image_base, size_of_image)) => {
                emu.maps.write_qword(memory_information, image_base);
                emu.maps.write_qword(memory_information + 8, size_of_image);
                emu.maps.write_dword(memory_information + 16, 0);
                if return_length_ptr != 0 {
                    emu.maps.write_qword(return_length_ptr, MEMORY_IMAGE_INFO_SIZE);
                }
                emu.regs_mut().rax = STATUS_SUCCESS;
            }
            None => {
                emu.regs_mut().rax = STATUS_INVALID_ADDRESS;
            }
        }
        return;
    }

    if memory_information_class != MEMORY_INFORMATION_CLASS_MEMORY_BASIC_INFORMATION
        && memory_information_class != MEMORY_INFORMATION_CLASS_MEMORY_PRIVILEGED_BASIC_INFORMATION
    {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if memory_information_length < MemoryBasicInformation64::SIZE {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if !emu.maps.is_mapped(base_address) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.get_addr_base(base_address).unwrap_or(0);
    let region_size = emu
        .maps
        .get_mem_by_addr(base_address)
        .map(|m| m.size() as u64)
        .unwrap_or(0);
    let alloc_protect = PAGE_EXECUTE | PAGE_READWRITE;
    let mem_info = MemoryBasicInformation64 {
        base_address: base,
        allocation_base: base,
        allocation_protect: alloc_protect,
        partition_id: 0,
        reserved: 0,
        region_size,
        state: MEM_COMMIT,
        protect: alloc_protect,
        typ: MEM_PRIVATE,
    };

    mem_info.save(memory_information, &mut emu.maps);

    if return_length_ptr != 0 {
        if !emu
            .maps
            .write_qword(return_length_ptr, MemoryBasicInformation64::SIZE)
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

    let size_in = if region_sz_ptr != 0 && emu.maps.is_mapped(region_sz_ptr) {
        emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
    } else {
        0
    };
    let pref_in = if base_ptr != 0 && emu.maps.is_mapped(base_ptr) {
        emu.maps.read_qword(base_ptr).unwrap_or(0)
    } else {
        0
    };
    log_orange!(
        emu,
        "syscall 0x{:x}: NtAllocateVirtualMemory rcx/h: 0x{:x} rdx/base_ptr: 0x{:x} *Base(in): 0x{:x} r9/region_sz_ptr: 0x{:x} *Size(in): 0x{:x} [rsp+28]/type: 0x{:x} [rsp+30]/prot: 0x{:x}",
        WIN64_NTALLOCATEVIRTUALMEMORY,
        process_handle,
        base_ptr,
        pref_in,
        region_sz_ptr,
        size_in,
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

    patch_ldr_heap_list_sentinel(emu, base, size);

    // Kernel writes back the actual (page-rounded) region size; ntdll uses *RegionSize after return.
    if !emu.maps.write_qword(region_sz_ptr, size) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtAllocateVirtualMemoryEx` — extended version with MEM_EXTENDED_PARAMETER support.
/// Parameters: RCX ProcessHandle, RDX *BaseAddress, R8 *RegionSize, R9 AllocationType,
/// [RSP+28h] Protect, [RSP+30h] ExtendedParameters, [RSP+38h] ExtendedParameterCount.
pub fn nt_allocate_virtual_memory_ex(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let alloc_type = emu.regs().r9 as u32;
    let rsp = emu.regs().rsp;
    let protect = emu.maps.read_dword(rsp + 0x28).unwrap_or(0);
    let ext_count = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);

    let size_in = if region_sz_ptr != 0 && emu.maps.is_mapped(region_sz_ptr) {
        emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
    } else {
        0
    };
    let pref_in = if base_ptr != 0 && emu.maps.is_mapped(base_ptr) {
        emu.maps.read_qword(base_ptr).unwrap_or(0)
    } else {
        0
    };
    log_orange!(
        emu,
        "syscall 0x{:x}: NtAllocateVirtualMemoryEx rcx/h: 0x{:x} rdx/base_ptr: 0x{:x} *Base(in): 0x{:x} r8/region_sz_ptr: 0x{:x} *Size(in): 0x{:x} r9/type: 0x{:x} [rsp+28]/prot: 0x{:x} [rsp+38]/ext_count: 0x{:x}",
        WIN64_NTALLOCATEVIRTUALMEMORYEX,
        process_handle,
        base_ptr,
        pref_in,
        region_sz_ptr,
        size_in,
        alloc_type,
        protect,
        ext_count
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

    patch_ldr_heap_list_sentinel(emu, base, size);

    if !emu.maps.write_qword(region_sz_ptr, size) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtFreeVirtualMemory` — x64: RCX, RDX, R8, R9 (all in registers).
pub fn nt_free_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let free_type = emu.regs().r9 as u32;

    let base_disp: String = if base_ptr == 0 {
        "—".to_string()
    } else if emu.maps.is_mapped(base_ptr) {
        format!(
            "0x{:x}",
            emu.maps.read_qword(base_ptr).unwrap_or(0)
        )
    } else {
        "? (unmapped rdx)".to_string()
    };
    let region_size_disp: String = if region_sz_ptr == 0 {
        "— (r8=0)".to_string()
    } else if emu.maps.is_mapped(region_sz_ptr) {
        format!(
            "0x{:x}",
            emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
        )
    } else {
        "? (unmapped r8)".to_string()
    };

    log_orange!(
        emu,
        "syscall 0x{:x}: NtFreeVirtualMemory rcx/h: 0x{:x} rdx/base_ptr: 0x{:x} *BaseAddress: {} r8/region_sz_ptr: 0x{:x} *RegionSize: {} r9/free_type: 0x{:x}",
        WIN64_NTFREEVIRTUALMEMORY,
        process_handle,
        base_ptr,
        base_disp,
        region_sz_ptr,
        region_size_disp,
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

    if (free_type & MEM_RELEASE) != 0 && (free_type & MEM_DECOMMIT) != 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if (free_type & MEM_RELEASE) != 0 {
        // `Maps::dealloc` keys regions by allocation base. ntdll passes interior pointers,
        // exclusive ends (`addr == map.bottom`), etc.; use `alloc_region_base_for_free`.
        let alloc_base = emu.maps.alloc_region_base_for_free(base);
        let base_mapped = emu.maps.is_mapped(base);
        let released = if base_mapped {
            emu.maps.dealloc(base);
            true
        } else if let Some(ab) = alloc_base {
            emu.maps.dealloc(ab);
            true
        } else {
            false
        };
        if !released {
            // ntdll may call MEM_RELEASE on a range already torn down by a prior successful free or
            // on an address our single-map model treats as unmapped; real kernel often accepts the
            // no-op. Match `trace_LdrInitializeThunk.txt` forward progress under `--ssdt --init`.
            if emu.cfg.ssdt_use_ldr_initialize_thunk
                && base >= ALLOC64_MIN
                && base < ALLOC64_MAX
            {
                let _ = emu.maps.write_qword(base_ptr, 0);
                if region_sz_ptr != 0 {
                    let _ = emu.maps.write_qword(region_sz_ptr, 0);
                }
                emu.regs_mut().rax = STATUS_SUCCESS;
                return;
            }
            emu.regs_mut().rax = STATUS_INVALID_ADDRESS;
            return;
        }
        let _ = emu.maps.write_qword(base_ptr, 0);
        if region_sz_ptr != 0 {
            let _ = emu.maps.write_qword(region_sz_ptr, 0);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if (free_type & MEM_DECOMMIT) != 0 {
        // ZwFreeVirtualMemory: with MEM_DECOMMIT, a NULL `RegionSize` (or *RegionSize == 0) means
        // decommit the whole allocation that contains `BaseAddress`. Our old stub treated that as
        // sz==0 → STATUS_INVALID_PARAMETER, which broke ntdll heap/Ldr scratch frees (then
        // STATUS_APP_INIT_FAILURE / terminate with 0xC000000d).
        let mut sz = if region_sz_ptr != 0 {
            emu.maps.read_qword(region_sz_ptr).unwrap_or(0)
        } else {
            0
        };
        if !emu.maps.is_mapped(base) {
            emu.regs_mut().rax = STATUS_INVALID_ADDRESS;
            return;
        }
        if region_sz_ptr == 0 || sz == 0 {
            sz = emu
                .maps
                .get_mem_by_addr(base)
                .map(|m| {
                    let mb = m.get_base();
                    let top = mb.saturating_add(m.size() as u64);
                    top.saturating_sub(base)
                })
                .unwrap_or(0);
        }
        if sz == 0 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        let cap = sz.min(0x1000_0000) as usize;
        emu.maps.memset(base, 0, cap);
        if region_sz_ptr != 0 {
            let _ = emu.maps.write_qword(region_sz_ptr, cap as u64);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
}

/// `NtProtectVirtualMemory` — x64: RCX..R9 + 5th (`OldProtect`) at `[rsp+0x28]`.
pub fn nt_protect_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_ptr = emu.regs().rdx;
    let region_sz_ptr = emu.regs().r8;
    let new_protect = emu.regs().r9 as u32;
    let rsp = emu.regs().rsp;
    let old_protect_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtProtectVirtualMemory h: 0x{:x} base_ptr: 0x{:x} new_prot: 0x{:x}",
        WIN64_NTPROTECTVIRTUALMEMORY,
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

    log_orange!(
        emu,
        "syscall 0x{:x}: NtReadVirtualMemory h: 0x{:x} from: 0x{:x} to: 0x{:x} len: 0x{:x}",
        WIN64_NTREADVIRTUALMEMORY,
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

    log_orange!(
        emu,
        "syscall 0x{:x}: NtWriteVirtualMemory h: 0x{:x} to: 0x{:x} from: 0x{:x} len: 0x{:x}",
        WIN64_NTWRITEVIRTUALMEMORY,
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

    log_orange!(
        emu,
        "syscall 0x{:x}: NtUnmapViewOfSection h: 0x{:x} base: 0x{:x}",
        WIN64_NTUNMAPVIEWOFSECTION,
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
    log_orange!(
        emu,
        "syscall 0x{:x}: NtMapViewOfSection (stub)",
        WIN64_NTMAPVIEWOFSECTION
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}
