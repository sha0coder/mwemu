use crate::constants::*;
use crate::emu::Emu;

use super::sync;

/// `NtOpenKey` — RCX `KeyHandle` (out), RDX `DesiredAccess`, R8 `ObjectAttributes`.
/// Returns a fake handle; the loader handles `STATUS_OBJECT_NAME_NOT_FOUND` gracefully.
pub fn nt_open_key(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attributes = emu.regs().r8;

    let key_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenKey out: 0x{:x}, access: 0x{:x}, name: \"{}\"",
        WIN64_NTOPENKEY,
        handle_out,
        desired_access,
        key_name
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let h = sync::next_handle();
    emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtQueryValueKey` — RCX `KeyHandle`, RDX `ValueName` (UNICODE_STRING*),
/// R8 `KeyValueInformationClass`, R9 `KeyValueInformation` (out),
/// 5th `Length` at `[rsp+0x28]`, 6th `ResultLength` at `[rsp+0x30]`.
///
/// The loader probes several registry values during init; returning
/// `STATUS_OBJECT_NAME_NOT_FOUND` is the safest stub — ntdll treats missing
/// values as "use default" and continues.
pub fn nt_query_value_key(emu: &mut Emu) {
    let key_handle = emu.regs().rcx;
    let value_name_ptr = emu.regs().rdx;
    let info_class = emu.regs().r8;
    let info_buf = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let length = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let result_length = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    let value_name = read_unicode_string(emu, value_name_ptr);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryValueKey handle: 0x{:x}, value: \"{}\", class: 0x{:x}, buf: 0x{:x}, len: 0x{:x}",
        WIN64_NTQUERYVALUEKEY,
        key_handle,
        value_name,
        info_class,
        info_buf,
        length
    );

    if result_length != 0 && emu.maps.is_mapped(result_length) {
        emu.maps.write_dword(result_length, 0);
    }

    emu.regs_mut().rax = STATUS_OBJECT_NAME_NOT_FOUND;
}

fn read_unicode_string(emu: &Emu, addr: u64) -> String {
    if addr == 0 || !emu.maps.is_mapped(addr) {
        return String::new();
    }
    let _len = emu.maps.read_word(addr).unwrap_or(0);
    let buf = emu.maps.read_qword(addr + 8).unwrap_or(0);
    if buf == 0 || !emu.maps.is_mapped(buf) {
        return String::new();
    }
    emu.maps.read_wide_string(buf)
}

fn read_object_attributes_name(emu: &Emu, addr: u64) -> String {
    if addr == 0 || !emu.maps.is_mapped(addr) {
        return String::new();
    }
    // OBJECT_ATTRIBUTES64: Length(4) + pad(4) + RootDirectory(8) + ObjectName(8) ...
    let object_name_ptr = emu.maps.read_qword(addr + 0x10).unwrap_or(0);
    read_unicode_string(emu, object_name_ptr)
}
