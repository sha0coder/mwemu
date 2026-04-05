
use std::sync::atomic::Ordering;

use crate::windows::constants::*;
use crate::emu::Emu;
use crate::windows::structures::ProcessBasicInformation64;

fn is_current_process_handle(h: u64) -> bool {
    h == !0 || h == 0xffff_ffff_ffff_fffe
}

/// `NtQueryInformationProcess` — x64: RCX..R9 + 5th arg (`ReturnLength`) at `[rsp+0x28]`.
pub fn nt_query_information_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let process_information_class = emu.regs().rdx;
    let process_information = emu.regs().r8;
    let process_information_length = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(emu, "syscall 0x{:x}: NtQueryInformationProcess process_handle: 0x{:x}, process_information_class: 0x{:x}, process_information: 0x{:x}, process_information_length: 0x{:x}, return_length_ptr: 0x{:x}", WIN64_NTQUERYINFORMATIONPROCESS, process_handle, process_information_class, process_information, process_information_length, return_length_ptr);

    if process_information_class == PROCESS_INFORMATION_CLASS_PROCESS_BASIC_INFORMATION {
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
        return;
    }

    if process_information_class == PROCESS_INFORMATION_CLASS_PROCESS_COOKIE {
        // Used by ntdll init paths; output is a 32-bit cookie (see SDK / `ProcessCookie`).
        if process_information == 0 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if process_information_length < 4 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(process_information) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.write_dword(process_information, 0) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if return_length_ptr != 0 {
            if !emu.maps.write_dword(return_length_ptr, 4) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if process_information_class == PROCESS_INFORMATION_CLASS_PROCESS_DEBUG_PORT {
        if process_information_length < 8 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(process_information) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.write_qword(process_information, 0) {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if return_length_ptr != 0 {
            if !emu.maps.write_qword(return_length_ptr, 8) {
                emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
                return;
            }
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
}

/// `NtQueryPerformanceCounter` — RCX = counter (PLARGE_INTEGER), RDX = frequency (PLARGE_INTEGER, optional).
/// Matches the usual x64 syscall stub used from `kernelbase!QueryPerformanceCounter` → ntdll.
pub fn nt_query_performance_counter(emu: &mut Emu) {
    let counter_ptr = emu.regs().rcx;
    let freq_ptr = emu.regs().rdx;

    log_orange!(emu, "syscall 0x{:x}: NtQueryPerformanceCounter counter: 0x{:x} freq: 0x{:x}", WIN64_NTQUERYPERFORMANCECOUNTER, counter_ptr, freq_ptr);

    if counter_ptr == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }
    if !emu.maps.is_mapped(counter_ptr) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    let counter_value = (emu.tick as u64).saturating_mul(1000);
    if !emu.maps.write_qword(counter_ptr, counter_value) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    if freq_ptr != 0 {
        if !emu.maps.is_mapped(freq_ptr) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
        // Fake ~10 MHz; order of magnitude typical for QPC frequency on Windows.
        if !emu.maps.write_qword(freq_ptr, 10_000_000) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryInformationThread` — x64: 5th arg at `[rsp+0x28]`.
pub fn nt_query_information_thread(emu: &mut Emu) {
    let _thread_handle = emu.regs().rcx;
    let thread_class = emu.regs().rdx;
    let thread_info = emu.regs().r8;
    let thread_info_len = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let return_length_ptr = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryInformationThread class: 0x{:x}, out: 0x{:x}, len: 0x{:x}",
        WIN64_NTQUERYINFORMATIONTHREAD,
        thread_class,
        thread_info,
        thread_info_len
    );

    if thread_info == 0 && thread_info_len > 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if thread_class == THREAD_INFORMATION_CLASS_THREAD_BASIC_INFORMATION {
        const TBI_SIZE: u64 = 48;
        if thread_info_len < TBI_SIZE {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(thread_info) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
        for i in 0..TBI_SIZE {
            let _ = emu.maps.write_byte(thread_info + i, 0);
        }
        let teb_base = emu.maps.get_mem("teb").get_base();
        let _ = emu.maps.write_qword(thread_info + 8, teb_base);
        if return_length_ptr != 0 {
            let _ = emu.maps.write_qword(return_length_ptr, TBI_SIZE);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    if thread_class == THREAD_INFORMATION_CLASS_THREAD_QUERY_SET_WIN32_START_ADDRESS {
        if thread_info_len < 8 {
            emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
            return;
        }
        if !emu.maps.is_mapped(thread_info) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
        let start = emu.regs().rip;
        let _ = emu.maps.write_qword(thread_info, start);
        if return_length_ptr != 0 {
            let _ = emu.maps.write_qword(return_length_ptr, 8);
        }
        emu.regs_mut().rax = STATUS_SUCCESS;
        return;
    }

    emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
}

/// `NtSetInformationProcess` — x64: four arguments in RCX..R9.
pub fn nt_set_information_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let class = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtSetInformationProcess h: 0x{:x} class: 0x{:x}",
        WIN64_NTSETINFORMATIONPROCESS,
        process_handle,
        class
    );

    if !is_current_process_handle(process_handle) {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtSetInformationThread` — x64: four arguments in RCX..R9.
pub fn nt_set_information_thread(emu: &mut Emu) {
    let thread_handle = emu.regs().rcx;
    let class = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtSetInformationThread h: 0x{:x} class: 0x{:x}",
        WIN64_NTSETINFORMATIONTHREAD,
        thread_handle,
        class
    );

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtOpenProcess` — x64: PHANDLE, ACCESS_MASK, OBJECT_ATTRIBUTES, CLIENT_ID in RCX..R9.
pub fn nt_open_process(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let _desired_access = emu.regs().rdx;
    let _obj_attr = emu.regs().r8;
    let _client_id = emu.regs().r9;

    log_orange!(emu, "syscall 0x{:x}: NtOpenProcess out: 0x{:x}", WIN64_NTOPENPROCESS, handle_out);

    if handle_out == 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let fake = 0x4u64;
    if emu.maps.write_qword(handle_out, fake) {
        emu.regs_mut().rax = STATUS_SUCCESS;
    } else {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
    }
}

/// `NtTerminateProcess` — x64: ProcessHandle, ExitStatus.
pub fn nt_terminate_process(emu: &mut Emu) {
    let process_handle = emu.regs().rcx;
    let exit_status = emu.regs().rdx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtTerminateProcess h: 0x{:x} status: 0x{:x}",
        WIN64_NTTERMINATEPROCESS,
        process_handle,
        exit_status
    );

    if process_handle != !0 && process_handle != 0 {
        emu.regs_mut().rax = STATUS_ACCESS_DENIED;
        return;
    }

    emu.is_running.store(0, Ordering::Relaxed);
    emu.regs_mut().rax = STATUS_SUCCESS;
}
