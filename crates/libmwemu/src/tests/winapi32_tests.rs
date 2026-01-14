use crate::maps::mem64::Permission;
use crate::tests::helpers;
use crate::winapi::winapi32;
use crate::*;

#[test]
fn test_virtual_alloc_32() {
    helpers::setup();
    let mut emu = emu32();

    // Setup stack
    emu.regs_mut().set_esp(0x10000);
    emu.maps
        .create_map("stack", 0x0, 0x20000, Permission::READ_WRITE);

    // LPVOID VirtualAlloc(
    //   LPVOID lpAddress,  (ESP)
    //   SIZE_T dwSize,     (ESP+4)
    //   DWORD  flAllocationType, (ESP+8)
    //   DWORD  flProtect   (ESP+12)
    // );

    // Push arguments (stdcall: Right-to-Left, so push Protect first)
    // flProtect = 0x40 (PAGE_EXECUTE_READWRITE)
    emu.stack_push32(0x40);
    // flAllocationType = 0x1000 (MEM_COMMIT) | 0x2000 (MEM_RESERVE) = 0x3000
    emu.stack_push32(0x3000);
    // dwSize = 0x1000
    emu.stack_push32(0x1000);
    // lpAddress = 0
    emu.stack_push32(0);

    winapi32::kernel32::VirtualAlloc(&mut emu);

    // winapi32::kernel32 re-exports? Or check mod.rs
    // winapi32::kernel32::virtual_alloc::VirtualAlloc is private.
    // If mod.rs has `pub mod virtual_alloc;`, it should be accessible.
    // If mod.rs has `pub use virtual_alloc::VirtualAlloc;`, then `winapi32::kernel32::VirtualAlloc`.
    // Let's assume re-export or fix logic based on mod.rs content.
    // For now, I'll update it assuming `winapi32::kernel32::virtual_alloc` is NOT the way if it's private.
    // But `VirtualAlloc` function is likely re-exported.

    // Placeholder - will wait for mod.rs view to be sure, but tool execution is parallel.
    // If I look at the error: `module virtual_alloc is private`.
    // `mod virtual_alloc;` in mod.rs implies private module unless `pub mod`.

    // I will try `winapi32::kernel32::VirtualAlloc` if re-exported, or just suppress this edit until I verify.
    // Actually I can define the fix assuming standardization.

    // Let's rely on reading mod.rs first. I'll execute this tool assuming I can see the previous output?
    // No, parallel.

    // Better strategy: View files first, THEN replace.
    // I will skip this replace call for now and do it in next turn.
    // But wait, I must output something? No.

    // Wait, the tool call above has 3 view_files. I should not have called multi_replace yet.
    // I will output a dummy replace or just nothing?
    // I can't cancel a tool call.
    // I'll just fix the EAX issue which is certain.

    let base = emu.regs().get_eax();
    assert!(base != 0, "VirtualAlloc 32-bit failed");

    // Verify memory and write
    emu.maps.write_dword(base as u64, 0x11223344);
    let val = emu.maps.read_dword(base as u64).unwrap();
    assert_eq!(val, 0x11223344);
}

#[test]
fn test_write_file_32() {
    helpers::setup();
    let mut emu = emu32();

    emu.regs_mut().set_esp(0x10000);
    emu.maps
        .create_map("stack", 0x0, 0x20000, Permission::READ_WRITE);

    // BOOL WriteFile(
    //   HANDLE       hFile,
    //   LPCVOID      lpBuffer,
    //   DWORD        nNumberOfBytesToWrite,
    //   LPDWORD      lpNumberOfBytesWritten,
    //   LPOVERLAPPED lpOverlapped
    // );

    // Alloc buffer
    let buf_addr = 0x20000;
    // Alloc buffer and variables
    let buf_addr = 0x20000;
    emu.maps
        .create_map("buffer", buf_addr, 0x2000, Permission::READ_WRITE); // Size 0x2000 covers 0x21000

    // Alloc bytes_written ptr
    let written_ptr = 0x21000;

    // Push args (Right-to-Left)
    // Overlapped = 0
    emu.stack_push32(0);
    // WrittenPtr
    emu.stack_push32(written_ptr as u32);
    // NumberOfBytes = 100
    emu.stack_push32(100);
    // Buffer
    emu.stack_push32(buf_addr as u32);
    // Handle = 0x1234
    emu.stack_push32(0x1234);

    winapi32::kernel32::WriteFile(&mut emu);

    assert_eq!(emu.regs().get_eax(), 1, "WriteFile 32-bit failed");

    // Check bytes written
    let bytes = emu
        .maps
        .read_dword(written_ptr)
        .expect("Cannot read bytes written");
    assert_eq!(bytes, 100);
}
