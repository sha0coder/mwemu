use crate::constants::*;
use crate::emu::Emu;

/// `SYSTEM_TIMEOFDAY_INFORMATION` size (56 bytes).
const SYSTEM_TIMEOFDAY_INFORMATION_SIZE: u32 = 56;
/// `SYSTEM_PROCESS_INFORMATION` header size (224 bytes).
const SYSTEM_PROCESS_INFORMATION_SIZE: u32 = 224;

fn write_return_length(emu: &mut Emu, ret_len_ptr: u64, n: u32) {
    if ret_len_ptr == 0 {
        return;
    }
    let _ = emu.maps.write_dword(ret_len_ptr, n);
}

/// `SYSTEM_INFORMATION_CLASS` values commonly hit during ntdll bootstrap (subset).
const SYSTEM_BASIC_INFORMATION: u64 = 0;
const SYSTEM_PROCESSOR_INFORMATION: u64 = 1;
const SYSTEM_PERFORMANCE_INFORMATION: u64 = 2;
const SYSTEM_TIMEOFDAY_INFORMATION: u64 = 3;
const SYSTEM_PROCESS_INFORMATION: u64 = 5;

/// `SYSTEM_KERNEL_DEBUGGER_INFORMATION` (class 0x23 = 35).
const SYSTEM_KERNEL_DEBUGGER_INFORMATION: u64 = 0x23;
/// `SystemKernelDebuggerInformationEx` (class 0x3e = 62) — modern alias.
const SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX: u64 = 0x3e;
/// `SYSTEM_CODE_INTEGRITY_INFORMATION` (Windows 8+).
const SYSTEM_CODE_INTEGRITY_INFORMATION: u64 = 103;
/// `SystemCodeIntegrityPolicyInformation` (0xC0) — ntdll `LdrInitializeThunk` queries this on
/// modern builds; real kernel returns `STATUS_SUCCESS` with a zeroed policy buffer (see
/// `trace_LdrInitializeThunk.txt` around `NtQuerySystemInformation`, `rcx == 0xC0`).
const SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION: u64 = 0xC0;

/// `NtQuerySystemInformation` — x64: RCX `Class`, RDX `Buffer`, R8 `Length`, R9 `ReturnLength`.
pub fn nt_query_system_information(emu: &mut Emu) {
    let class = emu.regs().rcx;
    let info = emu.regs().rdx;
    let len = emu.regs().r8 as u32;
    let ret_len_ptr = emu.regs().r9;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQuerySystemInformation class: 0x{:x}, buf: 0x{:x}, len: 0x{:x}, ret_len: 0x{:x}",
        WIN64_NTQUERYSYSTEMINFORMATION,
        class,
        info,
        len,
        ret_len_ptr
    );

    if info == 0 && len > 0 {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    if len > 0 && info != 0 {
        if !emu.maps.is_mapped(info) || !emu.maps.is_mapped(info + u64::from(len).saturating_sub(1)) {
            emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
            return;
        }
    }

    match class {
        SYSTEM_BASIC_INFORMATION => {
            const NEED: u32 = 0x40;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            // `SYSTEM_BASIC_INFORMATION` (layout per Windows SDK / x64)
            let _ = emu.maps.write_dword(info + 8, 4096); // PageSize
            let _ = emu.maps.write_dword(info + 12, 0x1000); // NumberOfPhysicalPages (stub)
            let _ = emu.maps.write_qword(info + 28, 0x0000_0000_0001_0000); // MinimumUserModeAddress
            let _ = emu.maps.write_qword(info + 36, 0x0000_7fff_ffff_ffff); // MaximumUserModeAddress
            let _ = emu.maps.write_qword(info + 44, 1); // ActiveProcessorsAffinityMask
            let _ = emu.maps.write_byte(info + 52, 1); // NumberOfProcessors
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESSOR_INFORMATION => {
            const NEED: u32 = 0x18;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PERFORMANCE_INFORMATION => {
            if len < 8 {
                write_return_length(emu, ret_len_ptr, 0x100);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, len);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_TIMEOFDAY_INFORMATION => {
            let need = SYSTEM_TIMEOFDAY_INFORMATION_SIZE;
            if len < need {
                write_return_length(emu, ret_len_ptr, need);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            let _ = emu.maps.write_qword(info + 8, 1); // CurrentTime (stub)
            write_return_length(emu, ret_len_ptr, need);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_PROCESS_INFORMATION => {
            let need = SYSTEM_PROCESS_INFORMATION_SIZE;
            if len < need {
                write_return_length(emu, ret_len_ptr, need);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            let _ = emu.maps.write_dword(info, 0);
            let _ = emu.maps.write_dword(info + 4, 0);
            write_return_length(emu, ret_len_ptr, need);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_KERNEL_DEBUGGER_INFORMATION | SYSTEM_KERNEL_DEBUGGER_INFORMATION_EX => {
            // SYSTEM_KERNEL_DEBUGGER_INFORMATION: { DebuggerEnabled: BOOLEAN, DebuggerNotPresent: BOOLEAN }
            const NEED: u32 = 2;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            let _ = emu.maps.write_byte(info, 0);     // DebuggerEnabled = FALSE
            let _ = emu.maps.write_byte(info + 1, 1);  // DebuggerNotPresent = TRUE
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_INFORMATION => {
            const NEED: u32 = 8;
            if len < NEED {
                write_return_length(emu, ret_len_ptr, NEED);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            let _ = emu.maps.write_dword(info, NEED);
            let _ = emu.maps.write_dword(info + 4, 0);
            write_return_length(emu, ret_len_ptr, NEED);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        SYSTEM_CODE_INTEGRITY_POLICY_INFORMATION => {
            // `SYSTEM_CODEINTEGRITYPOLICY_INFORMATION` is variable-length; ntdll passes ~0x20-byte
            // buffers during loader init. Zero-fill and report success like the reference trace.
            const MIN: u32 = 0x10;
            if len < MIN {
                write_return_length(emu, ret_len_ptr, MIN);
                emu.regs_mut().rax = STATUS_INFO_LENGTH_MISMATCH;
                return;
            }
            for off in 0..len {
                let _ = emu.maps.write_byte(info + u64::from(off), 0);
            }
            write_return_length(emu, ret_len_ptr, len);
            emu.regs_mut().rax = STATUS_SUCCESS;
        }

        _ => {
            log_orange!(
                emu,
                "NtQuerySystemInformation: unhandled class 0x{:x}, returning STATUS_INVALID_INFO_CLASS",
                class
            );
            write_return_length(emu, ret_len_ptr, 0);
            emu.regs_mut().rax = STATUS_INVALID_INFO_CLASS;
        }
    }
}

/// `NtManageHotPatch` — RCX `HotPatchInfo` (pointer to struct).
/// Stub: hot-patching is not supported in the emulator.
pub fn nt_manage_hot_patch(emu: &mut Emu) {
    let info = emu.regs().rcx;

    log_orange!(
        emu,
        "syscall 0x{:x}: NtManageHotPatch info: 0x{:x}",
        WIN64_NTMANAGEHOTPATCH,
        info
    );

    emu.regs_mut().rax = STATUS_NOT_SUPPORTED;
}

/// `NtQueryDebugFilterState(ComponentId, Level)` — returns FALSE (0) to indicate
/// that debug output is suppressed for this component/level (no debugger attached).
pub fn nt_query_debug_filter_state(emu: &mut Emu) {
    let component = emu.regs().rcx;
    let level = emu.regs().rdx;
    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryDebugFilterState component: 0x{:x}, level: 0x{:x}",
        WIN64_NTQUERYDEBUGFILTERSTATE,
        component,
        level
    );
    emu.regs_mut().rax = 0; // FALSE — debug output suppressed
}

/// `NtTraceEvent` — stub; ETW tracing is not emulated.
pub fn nt_trace_event(emu: &mut Emu) {
    log_orange!(
        emu,
        "syscall 0x{:x}: NtTraceEvent (stub)",
        WIN64_NTTRACEEVENT
    );
    emu.regs_mut().rax = STATUS_SUCCESS;
}
