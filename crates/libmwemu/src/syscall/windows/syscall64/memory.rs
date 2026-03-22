use crate::constants::*;
use crate::emu::Emu;
use crate::structures::MemoryBasicInformation;

/// `NtQueryVirtualMemory` — x64 register/stack layout matches the ntdll syscall stub
/// (RCX..R9 + 5th/6th on stack at `rsp+0x28` / `rsp+0x30`).
pub fn nt_query_virtual_memory(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let base_address = emu.regs().rdx;
    let memory_information_class = emu.regs().r8;
    let memory_information = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let memory_information_length = emu
        .maps
        .read_qword(rsp + 0x28)
        .expect("NtQueryVirtualMemory: memory_information_length");
    let return_length_ptr = emu
        .maps
        .read_qword(rsp + 0x30)
        .expect("NtQueryVirtualMemory: return_length");

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
