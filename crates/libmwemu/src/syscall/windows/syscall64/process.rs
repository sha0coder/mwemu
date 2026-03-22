
use crate::constants::*;
use crate::emu::Emu;
use crate::structures::ProcessBasicInformation64;

/// `NtQueryInformationProcess` — x64: RCX..R9 + 5th arg (`ReturnLength`) at `[rsp+0x28]`.
pub fn nt_query_information_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let process_information_class = emu.regs().rdx;
    let process_information = emu.regs().r8;
    let process_information_length = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu
        .maps
        .read_qword(rsp + 0x28)
        .expect("NtQueryInformationProcess: ReturnLength");

    log_red!(emu, "NtQueryInformationProcess process_handle: 0x{:x}, process_information_class: 0x{:x}, process_information: 0x{:x}, process_information_length: 0x{:x}, return_length_ptr: 0x{:x}", process_handle, process_information_class, process_information, process_information_length, return_length_ptr);

    if process_information_class != PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if process_information == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if !emu.maps.is_mapped(process_information) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if process_information_length < ProcessBasicInformation64::size() {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let peb_base = emu.maps.get_mem("peb").get_base();
    let process_info = ProcessBasicInformation64 {
        Reserved1: 0,
        PebBaseAddress: peb_base,
        Reserved2: [0, 0],
        UniqueProcessId: 4,
        Reserved3: 0,
    };
    process_info.save(process_information, &mut emu.maps);

    if return_length_ptr != 0 {
        if !emu
            .maps
            .write_qword(return_length_ptr, ProcessBasicInformation64::size())
        {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}
