use crate::windows::constants::*;
use crate::emu::Emu;

use super::sync;

/// `NtQueryMultipleValueKey` — x64 syscall 0x161.
///
/// RCX = KeyHandle, RDX = ValueEntries (PKEY_VALUE_ENTRY), R8 = EntryCount,
/// R9 = ValueBuffer, [rsp+0x28] = BufferLength ptr, [rsp+0x30] = RequiredBufferLength ptr.
///
/// Writes 0 to *BufferLength so the caller's post-processing loop sees zero bytes
/// in the output buffer and skips the loop body entirely.  Returns STATUS_SUCCESS.
pub fn nt_query_multiple_value_key(emu: &mut Emu) {
    let key_handle       = emu.regs().rcx;
    let value_entries    = emu.regs().rdx;
    let entry_count      = emu.regs().r8;
    let value_buffer     = emu.regs().r9;
    let rsp              = emu.regs().rsp;
    let buf_len_ptr      = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let req_buf_len_ptr  = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryMultipleValueKey handle: 0x{:x} entries: 0x{:x} count: {} buf: 0x{:x}",
        WIN64_NTQUERYMULTIPLEVALUEKEY,
        key_handle,
        value_entries,
        entry_count,
        value_buffer,
    );

    // Write 0 to *BufferLength: the caller uses this field as the loop upper-bound
    // (end = ValueBuffer + *BufferLength).  Zero means "no data written" so the
    // loop condition `cmp start, end` is immediately false and the loop is skipped.
    if buf_len_ptr != 0 && emu.maps.is_mapped(buf_len_ptr) {
        let _ = emu.maps.write_dword(buf_len_ptr, 0);
    }
    if req_buf_len_ptr != 0 && emu.maps.is_mapped(req_buf_len_ptr) {
        let _ = emu.maps.write_dword(req_buf_len_ptr, 0);
    }

    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtCreateDirectoryObject` — x64 syscall 0xac.
///
/// RCX = DirectoryHandle (out), RDX = DesiredAccess, R8 = ObjectAttributes.
/// Returns a fake handle; same pattern as NtOpenDirectoryObject.
pub fn nt_create_directory_object(emu: &mut Emu) {
    let handle_out        = emu.regs().rcx;
    let desired_access    = emu.regs().rdx;
    let object_attributes = emu.regs().r8;

    let dir_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtCreateDirectoryObject out: 0x{:x}, access: 0x{:x}, name: \"{}\"",
        WIN64_NTCREATEDIRECTORYOBJECT,
        handle_out,
        desired_access,
        dir_name,
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let h = sync::next_handle();
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

/// `NtOpenDirectoryObject` — x64 syscall 0x58.
///
/// RCX = DirectoryHandle (out), RDX = DesiredAccess, R8 = ObjectAttributes.
/// Returns a fake handle; callers treat it as an opaque token.
pub fn nt_open_directory_object(emu: &mut Emu) {
    let handle_out        = emu.regs().rcx;
    let desired_access    = emu.regs().rdx;
    let object_attributes = emu.regs().r8;

    let dir_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenDirectoryObject out: 0x{:x}, access: 0x{:x}, name: \"{}\"",
        WIN64_NTOPENDIRECTORYOBJECT,
        handle_out,
        desired_access,
        dir_name,
    );

    if handle_out == 0 || !emu.maps.is_mapped(handle_out) {
        emu.regs_mut().rax = STATUS_INVALID_PARAMETER;
        return;
    }

    let h = sync::next_handle();
    let _ = emu.maps.write_qword(handle_out, h);
    emu.regs_mut().rax = STATUS_SUCCESS;
}

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

/// `NtOpenKeyEx` — RCX `KeyHandle` (out), RDX `DesiredAccess`, R8 `ObjectAttributes`,
/// R9 `OpenOptions`. Same as NtOpenKey with an extra options field; return a fake handle.
pub fn nt_open_key_ex(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attributes = emu.regs().r8;

    let key_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenKeyEx out: 0x{:x}, access: 0x{:x}, name: \"{}\"",
        WIN64_NTOPENKEYEX,
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

/// `NtOpenKeyTransacted` — RCX `KeyHandle` (out), RDX `DesiredAccess`,
/// R8 `ObjectAttributes`, R9 `TransactionHandle`. Return a fake handle.
pub fn nt_open_key_transacted(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attributes = emu.regs().r8;

    let key_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenKeyTransacted out: 0x{:x}, access: 0x{:x}, name: \"{}\"",
        WIN64_NTOPENKEYTRANSACTED,
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

/// `NtOpenKeyTransactedEx` — RCX `KeyHandle` (out), RDX `DesiredAccess`,
/// R8 `ObjectAttributes`, R9 `OpenOptions`, [rsp+0x28] `TransactionHandle`.
/// Return a fake handle.
pub fn nt_open_key_transacted_ex(emu: &mut Emu) {
    let handle_out = emu.regs().rcx;
    let desired_access = emu.regs().rdx;
    let object_attributes = emu.regs().r8;

    let key_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtOpenKeyTransactedEx out: 0x{:x}, access: 0x{:x}, name: \"{}\"",
        WIN64_NTOPENKEYTRANSACTEDEX,
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

/// `NtQueryOpenSubKeysEx` — syscall 0x164.
/// RCX = TargetKey (OBJECT_ATTRIBUTES*), RDX = BufferLength,
/// R8 = Buffer (KEY_OPEN_SUBKEYS_INFORMATION* out), R9 = RequiredSize (out PULONG).
///
/// Returns KEY_OPEN_SUBKEYS_INFORMATION with Count=0 (no open subkeys tracked).
pub fn nt_query_open_subkeys_ex(emu: &mut Emu) {
    let object_attributes = emu.regs().rcx;
    let buffer_length = emu.regs().rdx;
    let buffer = emu.regs().r8;
    let required_size = emu.regs().r9;

    let key_name = read_object_attributes_name(emu, object_attributes);

    log_orange!(
        emu,
        "syscall 0x{:x}: NtQueryOpenSubKeysEx key: \"{}\", buf: 0x{:x}, len: 0x{:x}",
        WIN64_NTQUERYOPENSUBKEYSEX,
        key_name,
        buffer,
        buffer_length,
    );

    // KEY_OPEN_SUBKEYS_INFORMATION: ULONG Count followed by Count*KEY_PID_ARRAY entries.
    // We return Count=0, needing only 4 bytes.
    const NEEDED: u32 = 4;
    if required_size != 0 && emu.maps.is_mapped(required_size) {
        let _ = emu.maps.write_dword(required_size, NEEDED);
    }

    if buffer == 0 || buffer_length < NEEDED as u64 {
        emu.regs_mut().rax = STATUS_BUFFER_TOO_SMALL;
        return;
    }

    if !emu.maps.is_mapped(buffer) {
        emu.regs_mut().rax = STATUS_ACCESS_VIOLATION;
        return;
    }

    // Write Count = 0.
    let _ = emu.maps.write_dword(buffer, 0);
    emu.regs_mut().rax = STATUS_SUCCESS;
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
