use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi64;
use crate::*; // Assuming crate root has winapi module public or we can access it.
              // If `winapi` mod is not public, we might have issues.
              // Existing tests import `use crate::*;`.
              // `lib.rs` usually has `pub mod winapi;`.

#[test]
fn test_write_file() {
    helpers::setup();
    let mut emu = emu64();

    // Setup stack
    emu.regs_mut().rsp = 0x8000;
    emu.maps
        .create_map("stack", 0x0, 0x10000, Permission::READ_WRITE);

    // Setup buffer
    let buff_addr = 0x100000;
    emu.maps
        .create_map("buffer", buff_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(buff_addr, "Hello WinAPI");

    // Setup arguments for WriteFile
    // BOOL WriteFile(
    //   HANDLE       hFile,
    //   LPCVOID      lpBuffer,
    //   DWORD        nNumberOfBytesToWrite,
    //   LPDWORD      lpNumberOfBytesWritten,
    //   LPOVERLAPPED lpOverlapped
    // );

    // RCX = hFile (dummy handle)
    emu.regs_mut().rcx = 0x1234;
    // RDX = lpBuffer
    emu.regs_mut().rdx = buff_addr;
    // R8 = nNumberOfBytesToWrite
    emu.regs_mut().r8 = 12;
    // R9 = lpNumberOfBytesWritten (pointer)
    let written_ptr = 0x200000;
    emu.maps
        .create_map("written", written_ptr, 0x1000, Permission::READ_WRITE);
    emu.regs_mut().r9 = written_ptr;

    // Stack param: lpOverlapped (at rsp + 0x28, shadow space 0x20)
    // Actually WriteFile implementation reads `rsp + 0x20`.
    // "read_qword(emu.regs().rsp + 0x20)"
    // Typically shadow space is 32 bytes (0x20). The 5th arg is at rsp+0x28.
    // If the impl reads 0x20 directly, it might mean it assumes the specialized handling or I verified line 12: `emu.regs().rsp + 0x20`.
    // Wait, the line 12 says: `emu.maps.read_qword(emu.regs().rsp + 0x20)`.
    // Typically parameters are: rcx, rdx, r8, r9. Stack: [rsp+0x28] (5th), [rsp+0x30] (6th).
    // If it reads +0x20, it might be reading the shadow space slot for the 5th arg? No, shadow space is 0x0..0x20. 5th arg is at 0x28.
    // Let's assume the implementation expects the caller to have pushed/reserved stack.
    // If I just set RSP, +0x20 is valid memory.
    // I should write 0 to it (NULL overlapped).
    emu.maps.write_qword(emu.regs().rsp + 0x20, 0);

    // Call the function directly
    winapi64::kernel32::WriteFile(&mut emu);

    // Check Result
    // RAX should be 1 (TRUE)
    assert_eq!(emu.regs().rax, 1, "WriteFile failed (returned 0)");

    // Read bytes written
    let bytes = emu.maps.read_dword(written_ptr).unwrap();
    assert_eq!(bytes, 12);
}

#[test]
fn test_get_module_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    // HMODULE GetModuleHandleA(
    //   LPCSTR lpModuleName
    // );

    // "kernel32.dll"
    let name_addr = 0x20000;
    emu.maps
        .create_map("data", name_addr, 0x1000, Permission::READ_WRITE);
    emu.maps.write_string(name_addr, "kernel32.dll");

    // Create the expected module map "kernel32.pe"
    emu.maps.create_map(
        "kernel32.pe",
        0x7FF10000000,
        0x10000,
        Permission::READ_EXECUTE,
    );

    emu.regs_mut().rcx = name_addr;

    winapi64::kernel32::GetModuleHandleA(&mut emu);

    let h_mod = emu.regs().rax;
    assert_eq!(
        h_mod, 0x7FF10000000,
        "GetModuleHandleA('kernel32.dll') returned incorrect base"
    );
}

#[test]
fn test_close_handle_64() {
    helpers::setup();
    let mut emu = emu64();

    // CloseHandle checks if handle exists in global map.
    // If not, it panics.
    // We need to create a valid handle first.
    // Use `handler_create` from helper? It's pub?
    // helper::handler_create(name) -> handle

    let handle = crate::winapi::helper::handler_create("dummy_file");
    emu.regs_mut().rcx = handle;

    winapi64::kernel32::CloseHandle(&mut emu);

    // Expect 1
    assert_eq!(emu.regs().rax, 1);
}

#[test]
fn test_virtual_alloc() {
    helpers::setup();
    let mut emu = emu64();

    // LPVOID VirtualAlloc(
    //   LPVOID lpAddress,
    //   SIZE_T dwSize,
    //   DWORD  flAllocationType,
    //   DWORD  flProtect
    // );

    emu.regs_mut().rcx = 0; // lpAddress
    emu.regs_mut().rdx = 0x1000; // dwSize
    emu.regs_mut().r8 = 0x1000 | 0x2000; // MEM_COMMIT | MEM_RESERVE
    emu.regs_mut().r9 = 0x40; // PAGE_EXECUTE_READWRITE

    winapi64::kernel32::VirtualAlloc(&mut emu);

    let base = emu.regs().rax;
    assert!(base != 0, "VirtualAlloc failed");

    // Verify memory access
    emu.maps.write_dword(base, 0xDEADBEEF);
    let val = emu.maps.read_dword(base).unwrap();
    assert_eq!(val, 0xDEADBEEF);
}
