use std::fs::File;
use std::io::Read as _;
use std::io::Seek as _;
use std::io::SeekFrom;

use crate::console::Console;
use crate::constants;
use crate::context::context64::Context64;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::structures;
use crate::winapi::helper;
use crate::winapi::winapi64::kernel32;
use crate::winapi::winapi64::kernel32::InitializeCriticalSection;

const PAGE_NOACCESS: u32 = 0x01;
const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_WRITECOPY: u32 = 0x08;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_EXECUTE_READ: u32 = 0x20;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;
const PAGE_GUARD: u32 = 0x100;
const PAGE_NOCACHE: u32 = 0x200;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "ZwQueueApcThread" => ZwQueueApcThread(emu),
        "NtAllocateVirtualMemory" => NtAllocateVirtualMemory(emu),
        "NtGetContextThread" => NtGetContextThread(emu),
        "RtlAddVectoredExceptionHandler" => RtlAddVectoredExceptionHandler(emu),
        "RtlRemoveVectoredExceptionHandler" => RtlRemoveVectoredExceptionHandler(emu),
        "LdrLoadDll" => LdrLoadDll(emu),
        "NtQueryVirtualMemory" => NtQueryVirtualMemory(emu),
        "stricmp" => stricmp(emu),
        "RtlExitUserThread" => RtlExitUserThread(emu),
        "RtlAllocateHeap" => RtlAllocateHeap(emu),
        "RtlQueueWorkItem" => RtlQueueWorkItem(emu),
        "NtWaitForSingleObject" => NtWaitForSingleObject(emu),
        "sscanf" => sscanf(emu),
        "NtGetTickCount" => NtGetTickCount(emu),
        "NtQueryPerformanceCounter" => NtQueryPerformanceCounter(emu),
        "RtlGetProcessHeaps" => RtlGetProcessHeaps(emu),
        "RtlDosPathNameToNtPathName_U" => RtlDosPathNameToNtPathName_U(emu),
        "RtlInitializeCriticalSection" => InitializeCriticalSection(emu),
        "NtCreateFile" => NtCreateFile(emu),
        "RtlFreeHeap" => RtlFreeHeap(emu),
        "NtQueryInformationFile" => NtQueryInformationFile(emu),
        "NtSetInformationFile" => NtSetInformationFile(emu),
        "NtReadFile" => NtReadFile(emu),
        "NtClose" => NtClose(emu),
        "RtlInitializeCriticalSectionAndSpinCount" => RtlInitializeCriticalSectionAndSpinCount(emu),
        "NtProtectVirtualMemory" => NtProtectVirtualMemory(emu),
        "RtlEnterCriticalSection" => RtlEnterCriticalSection(emu),
        "RtlGetVersion" => RtlGetVersion(emu),
        "RtlInitializeCriticalSectionEx" => RtlInitializeCriticalSectionEx(emu),
        "RtlFreeAnsiString" => RtlFreeAnsiString(emu),
        "memset" => memset(emu),
        "RtlSetUnhandledExceptionFilter" => RtlSetUnhandledExceptionFilter(emu),
        "RtlCopyMemory" => RtlCopyMemory(emu),
        "RtlReAllocateHeap" => RtlReAllocateHeap(emu),
        "NtFlushInstructionCache" => NtFlushInstructionCache(emu),
        "LdrGetDllHandleEx" => LdrGetDllHandleEx(emu),
        "NtTerminateThread" => NtTerminateThread(emu),
        "RtlAddFunctionTable" => RtlAddFunctionTable(emu),
        "RtlCaptureContext" => RtlCaptureContext(emu),
        "RtlMoveMemory" => RtlMoveMemory(emu),
        "RtlZeroMemory" => RtlZeroMemory(emu),
        "RtlLookupFunctionEntry" => RtlLookupFunctionEntry(emu),
        "strlen" => strlen(emu),
        "NtSetInformationThread" => NtSetInformationThread(emu),
        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(
                        &emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api;
        }
    }

    String::new()
}
fn RtlZeroMemory(emu: &mut emu::Emu) {
    let dest = emu.regs().rcx;
    let length = emu.regs().rdx;

    log_red!(
    emu,
    "ntdll!RtlZeroMemory dest: 0x{:x} length: {}",
    dest,
    length
    );

    emu.maps.memset(dest, 0, length as usize);
}


fn RtlMoveMemory(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let sz = emu.regs().r8 as usize;

    let result = emu.maps.memcpy(dst, src, sz);
    if result == false {
        panic!("RtlMoveMemory failed to copy");
    }

    log_red!(
        emu,
        "** {} ntdll!RtlMoveMemory dst = {:x} src = {:x} sz = {}",
        emu.pos,
        dst,
        src,
        sz
    );
}

fn NtAllocateVirtualMemory(emu: &mut emu::Emu) {
    /*
        __kernel_entry NTSYSCALLAPI NTSTATUS NtAllocateVirtualMemory(
            [in]      HANDLE    ProcessHandle,
            [in, out] PVOID     *BaseAddress,
            [in]      ULONG_PTR ZeroBits,
            [in, out] PSIZE_T   RegionSize,
            [in]      ULONG     AllocationType,
            [in]      ULONG     Protect
    */

    let addr_ptr = emu.regs().rcx;
    let size_ptr = emu.regs().rdx;
    let protection_offset = 0x30;
    let protection_addr = emu.regs().rsp + protection_offset;
    let protect_value = emu
        .maps
        .read_dword(protection_addr)
        .expect("Failed to read Protection argument at NtAllocateVirtualMemory");

    let can_read = (protect_value
        & (PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_write = (protect_value
        & (PAGE_READWRITE | PAGE_WRITECOPY | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    let can_execute = (protect_value
        & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY))
        != 0;

    // Special cases
    let no_access = (protect_value & PAGE_NOACCESS) != 0;
    let guard_page = (protect_value & PAGE_GUARD) != 0;
    let no_cache = (protect_value & PAGE_NOCACHE) != 0;

    let addr = emu
        .maps
        .read_qword(addr_ptr)
        .expect("bad NtAllocateVirtualMemory address parameter");
    let size = emu
        .maps
        .read_qword(size_ptr)
        .expect("bad NtAllocateVirtualMemory size parameter");

    let do_alloc: bool = if addr == 0 {
        true
    } else {
        emu.maps.is_mapped(addr)
    };

    if size == 0 {
        panic!("NtAllocateVirtualMemory mapping zero bytes.")
    }

    let alloc_addr: u64 = if do_alloc {
        match emu.maps.alloc(size) {
            Some(a) => a,
            None => panic!("/!\\ out of memory cannot allocate ntdll!NtAllocateVirtualMemory "),
        }
    } else {
        addr
    };

    log_red!(
        emu,
        "ntdll!NtAllocateVirtualMemory  addr: 0x{:x} sz: {} alloc: 0x{:x}",
        addr,
        size,
        alloc_addr
    );

    emu.maps
        .create_map(
            format!("valloc_{:x}", alloc_addr).as_str(),
            alloc_addr,
            size,
            Permission::from_flags(can_read, can_write, can_execute),
        )
        .expect("ntdll!NtAllocateVirtualMemory cannot create map");

    if !emu.maps.write_qword(addr_ptr, alloc_addr) {
        panic!("NtAllocateVirtualMemory: cannot write on address pointer");
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn stricmp(emu: &mut emu::Emu) {
    let str1ptr = emu.regs().rcx;
    let str2ptr = emu.regs().rdx;
    let str1 = emu.maps.read_string(str1ptr);
    let str2 = emu.maps.read_string(str2ptr);

    log_red!(emu, "ntdll!stricmp  '{}'=='{}'?", str1, str2);

    if str1 == str2 {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = 1;
    }
}

fn NtQueryVirtualMemory(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let addr = emu.regs().rdx;

    log_red!(emu, "ntdll!NtQueryVirtualMemory addr: 0x{:x}", addr);

    if handle != 0xffffffff {
        log::info!("\tusing handle of remote process {:x}", handle);

        if !helper::handler_exist(handle) {
            log::info!("\nhandler doesnt exist.");
        }
    }

    let out_meminfo_ptr = emu.regs().r9;

    if !emu.maps.is_mapped(addr) {
        log::info!(
            "/!\\ ntdll!NtQueryVirtualMemory: querying non maped addr: 0x{:x}",
            addr
        );

        emu.regs_mut().rax = constants::STATUS_INVALID_PARAMETER;
        return;
    }

    let base = emu.maps.get_addr_base(addr).unwrap_or(0);

    let mut mem_info = structures::MemoryBasicInformation::load(out_meminfo_ptr, &emu.maps);
    mem_info.base_address = base as u32; //addr & 0xfff;
    mem_info.allocation_base = base as u32; //  addr & 0xfff;
    mem_info.allocation_protect = constants::PAGE_EXECUTE | constants::PAGE_READWRITE;
    mem_info.state = constants::MEM_COMMIT;
    mem_info.typ = constants::MEM_PRIVATE;
    mem_info.save(out_meminfo_ptr, &mut emu.maps);

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn LdrLoadDll(emu: &mut emu::Emu) {
    // NTSTATUS NTAPI DECLSPEC_HOTPATCH 	LdrLoadDll (
    //      IN PWSTR SearchPath OPTIONAL,
    //      IN PULONG DllCharacteristics OPTIONAL,
    //      IN PUNICODE_STRING DllName,
    //      OUT PVOID *BaseAddress)

    let libname_ptr = emu.regs().r8;
    let libaddr_ptr = emu.regs().r9;

    let libname = emu.maps.read_wide_string(libname_ptr);
    log_red!(emu, "ntdll!LdrLoadDll   lib: {}", libname);

    if libname == "user32.dll" {
        let user32 = emu
            .maps
            .create_map("user32", 0x773b0000, 0x1000, Permission::READ_WRITE)
            .expect("ntdll!LdrLoadDll_gul cannot create map");
        user32.load("maps32/user32.bin");
        let user32_text = emu
            .maps
            .create_map(
                "user32_text",
                0x773b1000,
                0x1000,
                Permission::READ_WRITE_EXECUTE,
            )
            .expect("ntdll!LdrLoadDll_gul cannot create map");
        user32_text.load("maps32/user32_text.bin");

        if !emu.maps.write_qword(libaddr_ptr, 0x773b0000) {
            panic!("ntdll_LdrLoadDll: cannot write in addr param");
        }
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlAddVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu.regs().rcx;
    let fptr = emu.regs().rdx;

    log_red!(
        emu,
        "ntdll!RtlAddVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(fptr);
    emu.regs_mut().rax = 0x2c2878;
}

fn RtlRemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let p1 = emu.regs().rcx;
    let fptr = emu.regs().rdx;

    log_red!(
        emu,
        "ntdll!RtlRemoveVectoredExceptionHandler  {} callback: 0x{:x}",
        p1,
        fptr
    );

    emu.set_veh(0);
    emu.regs_mut().rax = 0;
}

fn NtGetContextThread(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let ctx_ptr = emu.regs().rdx;
    let ctx_ptr2 = emu
        .maps
        .read_qword(ctx_ptr)
        .expect("ntdll_NtGetContextThread: error reading context ptr");

    log_red!(emu, "ntdll_NtGetContextThread   ctx:");

    let ctx = Context64::new(&emu.regs());
    ctx.save(ctx_ptr2, &mut emu.maps);

    emu.regs_mut().rax = 0;
}

fn RtlExitUserThread(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!RtlExitUserThread");
    Console::spawn_console(emu);
    std::process::exit(1);
}

fn ZwQueueApcThread(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    let apc_routine = emu.regs().rdx;
    let apc_ctx = emu.regs().r8;
    let arg1 = emu.regs().r9;
    let arg2 = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("kernel32!ZwQueueApcThread cannot read arg2");

    log_red!(
        emu,
        "ntdll!ZwQueueApcThread hndl: {} routine: {} ctx: {} arg1: {} arg2: {}",
        thread_handle,
        apc_routine,
        apc_ctx,
        arg1,
        arg2
    );

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlAllocateHeap(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let mut size = emu.regs().r8;
    let alloc_addr: u64;

    /*
    if emu.maps.exists_mapname(&map_name) {
        let map = emu.maps.get_map_by_name_mut(&map_name).unwrap();
        alloc_addr = map.get_base();
        if size as usize > map.size() {
            map.set_size(size+1024);
        }
    } else {
    */

    if size < 1024 {
        size = 1024
    }
    let alloc_addr = match emu.maps.alloc(size) {
        Some(a) => a,
        None => panic!("/!\\ out of memory cannot allocate ntdll!RtlAllocateHeap"),
    };

    // TODO: fix this from READ_WRITE_EXECUTE to maybe READ_WRITE
    let map_name = format!("valloc_{:x}", alloc_addr);
    emu.maps
        .create_map(&map_name, alloc_addr, size, Permission::READ_WRITE)
        .expect("ntdll!RtlAllocateHeap cannot create map");
    //}

    log_red!(
        emu,
        "ntdll!RtlAllocateHeap  hndl: {:x} sz: {}   =addr: 0x{:x}",
        handle,
        size,
        alloc_addr
    );

    emu.regs_mut().rax = alloc_addr;
}

fn RtlQueueWorkItem(emu: &mut emu::Emu) {
    let fptr = emu.regs().rcx;
    let ctx = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!RtlQueueWorkItem  fptr: 0x{:x} ctx: 0x{:x} flags: {}",
        fptr,
        ctx,
        flags
    );

    if fptr > constants::LIBS_BARRIER64 {
        let name = kernel32::guess_api_name(emu, fptr);
        log::info!("api: {} ", name);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtWaitForSingleObject(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let bAlert = emu.regs().rdx;
    let timeout = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!NtWaitForSingleObject  hndl: 0x{:x} timeout: {}",
        handle,
        timeout
    );

    emu.regs_mut().rax = 0x102; //constants::STATUS_SUCCESS;
}

fn sscanf(emu: &mut emu::Emu) {
    let buffer_ptr = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let list = emu.regs().r8;

    let buffer = emu.maps.read_string(buffer_ptr);
    let fmt = emu.maps.read_string(fmt_ptr);

    log_red!(emu, "ntdll!sscanf out_buff: `{}` fmt: `{}`", buffer, fmt);

    let rust_fmt = fmt
        .replace("%x", "{x}")
        .replace("%d", "{}")
        .replace("%s", "{}")
        .replace("%hu", "{u16}")
        .replace("%i", "{}")
        .replace("%o", "{o}")
        .replace("%f", "{}");
    let params = rust_fmt.matches("{").count();

    unimplemented!("sscanf is unimplemented for now.");
}

fn NtGetTickCount(emu: &mut emu::Emu) {
    log_red!(emu, "ntdll!NtGetTickCount");
    emu.regs_mut().rax = emu.tick as u64;
}

fn NtQueryPerformanceCounter(emu: &mut emu::Emu) {
    let perf_counter_ptr = emu.regs().rcx;
    let perf_freq_ptr = emu.regs().rdx;

    log_red!(emu, "ntdll!NtQueryPerformanceCounter");

    emu.maps.write_dword(perf_counter_ptr, 0);
    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlGetProcessHeaps(emu: &mut emu::Emu) {
    let num_of_heaps = emu.regs().rcx;
    let out_process_heaps = emu.regs().rcx;

    log_red!(
        emu,
        "ntdll!RtlGetProcessHeaps num: {} out: 0x{:x}",
        num_of_heaps,
        out_process_heaps
    );

    emu.regs_mut().rax = 1;
}

struct CurDir {
    DosPath: String, // unicode
    Handle: u64,
}

/*
BOOLEAN
NTAPI
RtlDosPathNameToNtPathName_U(IN PCWSTR DosName,
                             OUT PUNICODE_STRING NtName,
                             OUT PCWSTR *PartName,
                             OUT PRTL_RELATIVE_NAME_U RelativeName)
*/

fn RtlDosPathNameToNtPathName_U(emu: &mut emu::Emu) {
    let dos_path_name_ptr = emu.regs().rcx;
    let nt_path_name_ptr = emu.regs().rdx; // This should point to a UNICODE_STRING structure
    let nt_file_name_part_ptr = emu.regs().r8;
    let curdir_ptr = emu.regs().r9;

    let dos_path_name = emu.maps.read_wide_string(dos_path_name_ptr);

    log_red!(
        emu,
        "ntdll!RtlDosPathNameToNtPathName_U dos_path='{}' dos_path_name_ptr: 0x{dos_path_name_ptr:x} nt_path_name_ptr: 0x{nt_path_name_ptr:x} nt_file_name_part_ptr: 0x{nt_file_name_part_ptr:x} curdir_ptr: 0x{curdir_ptr:x}",
        dos_path_name
    );

    // Convert DOS path to NT path (simple conversion for now)
    let nt_path = if dos_path_name.starts_with("\\\\?\\") {
        // Already in \\?\ format, convert to \??\
        format!("\\??\\{}", &dos_path_name[4..])
    } else if dos_path_name.len() >= 2 && dos_path_name.chars().nth(1) == Some(':') {
        // Drive letter format like C:\path -> \??\C:\path
        format!("\\??\\{}", dos_path_name)
    } else {
        // Other formats, just prepend \??\
        format!("\\??\\{}", dos_path_name)
    };

    log_red!(
        emu,
        "Converted DOS path '{}' to NT path '{}'",
        dos_path_name,
        nt_path
    );

    if nt_path_name_ptr > 0 {
        // nt_path_name_ptr points to a UNICODE_STRING structure that we need to populate

        // Calculate string length in bytes (UTF-16, so chars * 2)
        let string_length_bytes = nt_path.encode_utf16().count() * 2;

        // Allocate buffer for the NT path string
        match emu.maps.alloc((string_length_bytes + 2) as u64) {
            // +2 for null terminator
            Some(string_buffer_addr) => {
                // TODO: only create if it does not already exist
                // Create the string buffer map
                emu.maps
                    .create_map(
                        &format!("nt_path_string_{:x}", string_buffer_addr),
                        string_buffer_addr,
                        (string_length_bytes + 2) as u64,
                        Permission::READ_WRITE,
                    )
                    .expect("Failed to create nt_path_string map");

                // Write the NT path string to the allocated buffer
                emu.maps.write_wide_string(string_buffer_addr, &nt_path);

                // Now populate the UNICODE_STRING structure at nt_path_name_ptr
                // typedef struct _UNICODE_STRING {
                //     USHORT Length;        // +0x00
                //     USHORT MaximumLength; // +0x02
                //     PWSTR  Buffer;        // +0x08
                // } UNICODE_STRING;

                emu.maps
                    .write_word(nt_path_name_ptr, string_length_bytes as u16); // Length
                emu.maps
                    .write_word(nt_path_name_ptr + 2, (string_length_bytes + 2) as u16); // MaximumLength
                emu.maps
                    .write_qword(nt_path_name_ptr + 8, string_buffer_addr); // Buffer

                log_red!(
                    emu,
                    "Created UNICODE_STRING: Length={}, MaxLength={}, Buffer=0x{:x}",
                    string_length_bytes,
                    string_length_bytes + 2,
                    string_buffer_addr
                );

                // Set nt_file_name_part_ptr if requested
                if nt_file_name_part_ptr > 0 {
                    // Find the last backslash to get filename part
                    if let Some(last_backslash_pos) = nt_path.rfind('\\') {
                        let filename_offset = (last_backslash_pos + 1) * 2; // Convert to byte offset
                        let filename_part_addr = string_buffer_addr + filename_offset as u64;
                        emu.maps
                            .write_qword(nt_file_name_part_ptr, filename_part_addr);
                        log_red!(
                            emu,
                            "Set filename part pointer to 0x{:x}",
                            filename_part_addr
                        );
                    } else {
                        // No backslash found, filename part is the whole string
                        emu.maps
                            .write_qword(nt_file_name_part_ptr, string_buffer_addr);
                    }
                }
            }
            None => {
                log_red!(emu, "Failed to allocate memory for NT path string");
                emu.regs_mut().rax = 0; // Return failure
                return;
            }
        }
    }

    // Handle curdir_ptr if needed (simplified for now)
    if curdir_ptr > 0 {
        // This would typically populate a CurDir structure
        log_red!(emu, "CurDir handling not fully implemented");
    }

    emu.regs_mut().rax = 1; // Return success (TRUE)
}

/*
__kernel_entry NTSTATUS NtCreateFile(
  [out]          PHANDLE            FileHandle,
  [in]           ACCESS_MASK        DesiredAccess,
  [in]           POBJECT_ATTRIBUTES ObjectAttributes,
  [out]          PIO_STATUS_BLOCK   IoStatusBlock,
  [in, optional] PLARGE_INTEGER     AllocationSize,
  [in]           ULONG              FileAttributes,
  [in]           ULONG              ShareAccess,
  [in]           ULONG              CreateDisposition,
  [in]           ULONG              CreateOptions,
  [in]           PVOID              EaBuffer,
  [in]           ULONG              EaLength
);
*/
fn NtCreateFile(emu: &mut emu::Emu) {
    let out_hndl_ptr = emu.regs().rcx;
    let access_mask = emu.regs().rdx;
    let oattrib = emu.regs().r8;
    let iostat = emu.regs().r9;
    let alloc_sz = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtCreateFile error reading alloc_sz param");
    let fattrib = emu
        .maps
        .read_dword(emu.regs().rsp + 0x28)
        .expect("ntdll!NtCreateFile error reading fattrib param");
    let share_access = emu
        .maps
        .read_dword(emu.regs().rsp + 0x30)
        .expect("ntdll!NtCreateFile error reading share_access param");
    let create_disp = emu
        .maps
        .read_dword(emu.regs().rsp + 0x38)
        .expect("ntdll!NtCreateFile error reading create_disp param");
    let create_opt = emu
        .maps
        .read_dword(emu.regs().rsp + 0x40)
        .expect("ntdll!NtCreateFile error reading create_opt param");
    let ea_buff = emu
        .maps
        .read_qword(emu.regs().rsp + 0x48)
        .expect("ntdll!NtCreateFile error reading ea_buff param");
    let ea_len = emu
        .maps
        .read_dword(emu.regs().rsp + 0x50)
        .expect("ntdll!NtCreateFile error reading ea_len param");

    log_red!(emu, "** {} ntdll!NtCreateFile | Handle=0x{:x} Access=0x{:x} ObjAttr=0x{:x} IoStat=0x{:x} AllocSz=0x{:x} FileAttr=0x{:x} ShareAccess=0x{:x} CreateDisp=0x{:x} CreateOpt=0x{:x} EaBuff=0x{:x} EaLen=0x{:x}",
        emu.pos,
        out_hndl_ptr,
        access_mask,
        oattrib,
        iostat,
        alloc_sz,
        fattrib,
        share_access,
        create_disp,
        create_opt,
        ea_buff,
        ea_len
    );
    // Handle OBJECT_ATTRIBUTES structure properly
    /*
    typedef struct _OBJECT_ATTRIBUTES {
        ULONG           Length;           // +0x00 (4 bytes)
        HANDLE          RootDirectory;    // +0x08 (8 bytes on x64)
        PUNICODE_STRING ObjectName;       // +0x10 (8 bytes on x64)
        ULONG           Attributes;       // +0x18 (4 bytes)
        PVOID           SecurityDescriptor;        // +0x20 (8 bytes on x64)
        PVOID           SecurityQualityOfService;  // +0x28 (8 bytes on x64)
    } OBJECT_ATTRIBUTES;
    */
    let filename = if oattrib != 0 {
        log_red!(
            emu,
            "** {} Reading OBJECT_ATTRIBUTES at 0x{:x}",
            emu.pos,
            oattrib
        );

        // Dump the OBJECT_ATTRIBUTES structure first
        log_red!(emu, "** {} OBJECT_ATTRIBUTES structure dump:", emu.pos);
        for i in (0..0x30).step_by(8) {
            if let Some(qword_val) = emu.maps.read_qword(oattrib + i) {
                log_red!(emu, "** {}   +0x{:02x}: 0x{:016x}", emu.pos, i, qword_val);
            }
        }

        // Read RootDirectory and ObjectName fields
        let root_directory = emu.maps.read_qword(oattrib + 0x08).unwrap_or(0);
        let obj_name_ptr = emu.maps.read_qword(oattrib + 0x10).unwrap_or(0);

        log_red!(
            emu,
            "** {} RootDirectory: 0x{:x}, ObjectName pointer: 0x{:x}",
            emu.pos,
            root_directory,
            obj_name_ptr
        );

        // Handle different scenarios
        if obj_name_ptr == 0 {
            // Case 1: ObjectName is NULL - unnamed object
            log_red!(
                emu,
                "** {} ObjectName is NULL - creating unnamed object",
                emu.pos
            );

            if root_directory != 0 {
                // Creating unnamed object relative to root directory
                String::from("<unnamed_object_with_root>")
            } else {
                // Creating completely unnamed object
                String::from("<unnamed_object>")
            }
        } else if !emu.maps.is_mapped(obj_name_ptr) {
            // Case 2: ObjectName pointer is invalid
            log_red!(
                emu,
                "** {} ObjectName pointer 0x{:x} is not mapped",
                emu.pos,
                obj_name_ptr
            );
            String::from("<invalid_objname_ptr>")
        } else {
            // Case 3: ObjectName pointer is valid - read UNICODE_STRING
            log_red!(
                emu,
                "** {} Reading UNICODE_STRING at 0x{:x}",
                emu.pos,
                obj_name_ptr
            );

            // Debug: dump UNICODE_STRING structure
            for i in (0..16).step_by(8) {
                if let Some(qword_val) = emu.maps.read_qword(obj_name_ptr + i) {
                    log_red!(
                        emu,
                        "** {} UNICODE_STRING +0x{:02x}: 0x{:016x}",
                        emu.pos,
                        i,
                        qword_val
                    );
                }
            }

            let length = emu.maps.read_word(obj_name_ptr).unwrap_or(0);
            let max_length = emu.maps.read_word(obj_name_ptr + 2).unwrap_or(0);
            let buffer_ptr = emu.maps.read_qword(obj_name_ptr + 8).unwrap_or(0);

            log_red!(
                emu,
                "** {} UNICODE_STRING: Length={} MaxLength={} Buffer=0x{:x}",
                emu.pos,
                length,
                max_length,
                buffer_ptr
            );

            if buffer_ptr == 0 {
                // Case 4: UNICODE_STRING.Buffer is NULL
                log_red!(emu, "** {} UNICODE_STRING Buffer is NULL", emu.pos);

                if root_directory != 0 {
                    String::from("<null_buffer_with_root>")
                } else {
                    String::from("<null_buffer>")
                }
            } else if length == 0 {
                // Case 5: UNICODE_STRING.Length is 0 (empty string)
                log_red!(
                    emu,
                    "** {} UNICODE_STRING Length is 0 (empty string)",
                    emu.pos
                );

                if root_directory != 0 {
                    String::from("<empty_string_with_root>")
                } else {
                    String::from("<empty_string>")
                }
            } else if !emu.maps.is_mapped(buffer_ptr) {
                // Case 6: UNICODE_STRING.Buffer pointer is invalid
                log_red!(
                    emu,
                    "** {} UNICODE_STRING Buffer 0x{:x} is not mapped",
                    emu.pos,
                    buffer_ptr
                );
                String::from("<invalid_buffer_ptr>")
            } else {
                // Case 7: Valid UNICODE_STRING with valid buffer - read the string
                let char_count = (length / 2) as usize;
                let filename_str = emu.maps.read_wide_string_n(buffer_ptr, char_count);

                log_red!(emu, "** {} Filename: '{}'", emu.pos, filename_str);

                if root_directory != 0 {
                    // Relative path to root directory
                    format!("<root_0x{:x}>\\{}", root_directory, filename_str)
                } else {
                    // Absolute path
                    filename_str
                }
            }
        }
    } else {
        log_red!(emu, "** {} OBJECT_ATTRIBUTES pointer is null", emu.pos);
        String::from("<null_oattrib>")
    };

    log_red!(
        emu,
        "** {} ntdll!NtCreateFile resolved filename: '{}'",
        emu.pos,
        filename
    );

    if out_hndl_ptr > 0 {
        emu.maps
            .write_qword(out_hndl_ptr, helper::handler_create(&filename) as u64);
    }

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn RtlFreeHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let base_addr = emu.regs().r8;

    log_red!(emu, "ntdll!RtlFreeHeap 0x{}", base_addr);

    helper::handler_close(hndl);
    let name = emu.maps.get_addr_name(base_addr).unwrap_or("").to_string();
    if name.is_empty() {
        if emu.cfg.verbose >= 1 {
            log::info!("map not allocated, so cannot free it.");
        }
        emu.regs_mut().rax = 0;
        return;
    }

    if name.starts_with("alloc_") {
        emu.maps.dealloc(base_addr);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
        if emu.cfg.verbose >= 1 {
            log::info!("trying to free a systems map {}", name);
        }
    }
}

fn RtlFreeAnsiString(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlFreeAnsiString 0x{}", ptr);

    // TODO: no-op?
}

fn NtQueryInformationFile(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let stat = emu.regs().rdx;
    let fileinfo = emu.regs().r8;
    let len = emu.regs().r9;
    let fileinfoctls = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtQueryInformationFile cannot read fileinfoctls param");

    log_red!(emu, "ntdll!NtQueryInformationFile");

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtSetInformationFile(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let io_status_block = emu.regs().rdx;
    let file_information = emu.regs().r8;
    let length = emu.regs().r9;
    let file_information_class = emu
        .maps
        .read_dword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtSetInformationFile cannot read FileInformationClass param");

    log_red!(
        emu,
        "ntdll!NtSetInformationFile handle: 0x{:x} info_class: {} length: {}",
        file_handle,
        file_information_class,
        length
    );

    // TODO: implement something?

    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

/*
NTSTATUS NtReadFile(
  _In_     HANDLE           FileHandle,
  _In_opt_ HANDLE           Event,
  _In_opt_ PIO_APC_ROUTINE  ApcRoutine,
  _In_opt_ PVOID            ApcContext,
  _Out_    PIO_STATUS_BLOCK IoStatusBlock,
  _Out_    PVOID            Buffer,
  _In_     ULONG            Length,
  _In_opt_ PLARGE_INTEGER   ByteOffset,
  _In_opt_ PULONG           Key
);
*/
fn NtReadFile(emu: &mut emu::Emu) {
    let file_hndl = emu.regs().rcx;
    let ev_hndl = emu.regs().rdx;
    let apc_rout = emu.regs().r8;
    let apc_ctx = emu.regs().r9;
    let stat = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtReadFile error reading stat param");
    let buff = emu
        .maps
        .read_qword(emu.regs().rsp + 0x28)
        .expect("ntdll!NtReadFile error reading buff param");
    let len = emu
        .maps
        .read_qword(emu.regs().rsp + 0x30)
        .expect("ntdll!NtReadFile error reading len param") as usize;
    let off = emu
        .maps
        .read_qword(emu.regs().rsp + 0x38)
        .expect("ntdll!NtReadFile error reading off param");
    let key = emu
        .maps
        .read_qword(emu.regs().rsp + 0x40)
        .expect("ntdll!NtReadFile error reading key param");

    // file offset
    let file_offset = if off != 0 {
        // If off is not null, read the LARGE_INTEGER from that address
        match emu.maps.read_qword(off) {
            Some(offset_value) => offset_value,
            None => {
                log_red!(emu, "Failed to read file offset from 0x{:x}", off);
                emu.regs_mut().rax = constants::STATUS_INVALID_PARAMETER;
                return;
            }
        }
    } else {
        // If off is null, use current file position (start from 0 for simplicity)
        0
    };

    // filename from handle
    let filename = helper::handler_get_uri(file_hndl);

    log_red!(
        emu,
        "ntdll!NtReadFile {} hndl: 0x{:x} buff: 0x{:x} sz: {} off_var: 0x{:x}",
        filename,
        file_hndl,
        buff,
        len,
        off
    );

    emu.maps.memset(buff, 0x90, len);

    if filename == "\\??\\c:\\cwd" {
        let mut file = File::open(&emu.filename).unwrap();
        file.seek(SeekFrom::Start(file_offset));
        let mut file_buffer = vec![0u8; len];
        let bytes_read = file.read(&mut file_buffer).unwrap();
        for i in 0..bytes_read {
            if let Some(byte_val) = file_buffer.get(i) {
                emu.maps.write_byte(buff + i as u64, *byte_val);
            }
        }
        // TODO: Update the IO_STATUS_BLOCK if provided

        // Set return value
        if bytes_read == len {
            emu.regs_mut().rax = constants::STATUS_SUCCESS;
        } else if bytes_read == 0 {
            emu.regs_mut().rax = constants::STATUS_END_OF_FILE;
        } else {
            // Partial read
            emu.regs_mut().rax = constants::STATUS_SUCCESS;
        }
    } else {
        panic!("TODO: read {}", filename);
    }



    emu.regs_mut().rax = constants::STATUS_SUCCESS;
}

fn NtClose(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    let uri = helper::handler_get_uri(hndl);

    log_red!(emu, "ntdll!NtClose hndl: 0x{:x} uri: {}", hndl, uri);

    if uri.is_empty() {
        emu.regs_mut().rax = constants::STATUS_INVALID_HANDLE;
    } else {
        emu.regs_mut().rax = constants::STATUS_SUCCESS;
    }
}

fn RtlInitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu.regs().rcx;
    let spin_count = emu.regs().rdx;

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionAndSpinCount");

    emu.regs_mut().rax = 1;
}

fn NtProtectVirtualMemory(emu: &mut emu::Emu) {
    let sz = emu.regs().rcx;
    let status = emu.regs().rdx;
    let page_number = emu.regs().r8;
    let page = emu.regs().r9;
    let prot = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!NtProtectVirtualMemory error reading old prot param");

    log_red!(emu, "ntdll!NtProtectVirtualMemory sz: {} {}", sz, prot);

    emu.regs_mut().rax = constants::STATUS_SUCCESS
}

fn RtlEnterCriticalSection(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlEnterCriticalSection");

    emu.regs_mut().rax = 1;
}

fn RtlGetVersion(emu: &mut emu::Emu) {
    let versioninfo_ptr = emu.regs().rcx;

    log_red!(emu, "ntdll!RtlGetVersion");

    let versioninfo = structures::OsVersionInfoExA::new(); // TODO: should this be Ex?
    versioninfo.save(versioninfo_ptr, &mut emu.maps);

    emu.regs_mut().rax = 1;
}

fn RtlInitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let crit_sect_ptr = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = emu.regs().r8;

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionEx");

    emu.regs_mut().rax = 1;
}

fn memset(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let byte = emu.regs().rdx;
    let count = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!memset ptr: 0x{:x} byte: {} count: {}",
        ptr,
        byte,
        count
    );

    emu.maps.memset(ptr, byte as u8, count as usize);

    emu.regs_mut().rax = ptr;
}

fn RtlSetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let filter = emu.regs().rcx;

    log_red!(
        emu,
        "ntdll!RtlSetUnhandledExceptionFilter filter: 0x{:x}",
        filter
    );

    emu.set_uef(filter);
    emu.regs_mut().rax = 1;
}

/*
void RtlCopyMemory(
   void*       Destination,
   const void* Source,
   size_t      Length
);
*/
fn RtlCopyMemory(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let sz = emu.regs().r8 as usize;
    let result = emu.maps.memcpy(dst, src, sz);
    if result == false {
        panic!("RtlCopyMemory failed to copy");
    }
    log_red!(
        emu,
        "** {} ntdll!RtlCopyMemory dst = {:x} src = {:x} sz = {}",
        emu.pos,
        dst,
        src,
        sz
    );
}

fn RtlReAllocateHeap(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let sz = emu.regs().r8;

    let mapname = format!("valloc_{:x}", hndl);
    emu.regs_mut().rax = match emu.maps.get_map_by_name_mut(&mapname) {
        Some(mem) => {
            mem.set_size(sz + 1024);
            mem.get_base()
        }
        None => 0,
    };

    log_red!(emu, "ntdll!RtlReAllocateHeap hndl: {:x} sz: {}", hndl, sz);
}

fn NtFlushInstructionCache(emu: &mut emu::Emu) {
    let proc_hndl = emu.regs().rcx;
    let addr = emu.regs().rdx;
    let sz = emu.regs().r8;

    log_red!(
        emu,
        "ntdll!NtFlushInstructionCache hndl: {:x} 0x{:x} sz: {}",
        proc_hndl,
        addr,
        sz
    );

    emu.regs_mut().rax = 0;
}

fn LdrGetDllHandleEx(emu: &mut emu::Emu) {
    //LdrGetDllHandleEx (_In_ ULONG Flags, _In_opt_ PWSTR DllPath, _In_opt_ PULONG DllCharacteristics, _In_ PUNICODE_STRING DllName, _Out_opt_ PVOID *DllHandle)
    let flags = emu.regs().rcx;
    let path_ptr = emu.regs().rdx;
    let characteristics = emu.regs().r8;
    let dll_name_ptr = emu.regs().r9;
    let out_hndl = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("ntdll!LdrGetDllHandleEx error reading out_hdl");

    let dll_name = emu.maps.read_wide_string(dll_name_ptr);

    log_red!(emu, "ntdll!LdrGetDllHandleEx {}", dll_name);

    let result = emu.maps.memcpy(path_ptr, dll_name_ptr, dll_name.len());
    if result == false {
        panic!("LdrGetDllHandleEx failed to copy");
    }

    let handle = helper::handler_create(&dll_name);
    emu.maps.write_qword(out_hndl, handle);

    emu.regs_mut().rax = 1;
}

fn NtTerminateThread(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let exit_status = emu.regs().rdx;

    log_red!(emu, "ntdll!NtTerminateThread {:x} {}", handle, exit_status);

    emu.regs_mut().rax = 0;
}

/*
NTSYSAPI BOOLEAN RtlAddFunctionTable(
  [in] PRUNTIME_FUNCTION FunctionTable,
  [in] DWORD             EntryCount,
  [in] DWORD64           BaseAddress
);
*/
fn RtlAddFunctionTable(emu: &mut emu::Emu) {
    let function_table = emu.regs().rcx;
    let entry_count = emu.regs().rdx;
    let base_address = emu.regs().r8;

    log_red!(emu, "ntdll!RtlAddFunctionTable");

    // TODO: do something with it

    emu.regs_mut().rax = 1;
}

/*
NTSYSAPI VOID RtlCaptureContext(
  [out] PCONTEXT ContextRecord
);
*/
fn RtlCaptureContext(emu: &mut emu::Emu) {
    let context_record = emu.regs().rcx as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlCaptureContext {:x}",
        emu.pos,
        context_record
    );
    // TODO: implement this
}

/*
NTSYSAPI PRUNTIME_FUNCTION RtlLookupFunctionEntry(
  [in]  DWORD64               ControlPc,
  [out] PDWORD64              ImageBase,
  [out] PUNWIND_HISTORY_TABLE HistoryTable
);
*/
fn RtlLookupFunctionEntry(emu: &mut emu::Emu) {
    let control_pc = emu.regs().rcx as usize;
    let image_base = emu.regs().rdx as usize;
    let history_table = emu.regs().r8 as usize;
    log_red!(
        emu,
        "** {} ntdll!RtlLookupFunctionEntry {:x} {:x} {:x}",
        emu.pos,
        control_pc,
        image_base,
        history_table
    );
    // TODO: implement this
    emu.regs_mut().rax = 0;
}

fn strlen(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx as usize;
    log_red!(emu, "** {} ntdll!strlen {:x}", emu.pos, s_ptr);

    if s_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let s = emu.maps.read_string(s_ptr as u64);
    let l = s.len();

    log_red!(emu, "ntdll!strlen: `{}` {}", s, l);

    emu.regs_mut().rax = l as u32 as u64;
}

fn NtSetInformationThread(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    let thread_info_class = emu.regs().rdx;
    let thread_info_ptr = emu.regs().r8;
    let thread_info_length = emu.regs().r9;

    // TODO: Parse ThreadInformationClass values:
    //   - ThreadHideFromDebugger = 17 (common anti-debug technique)
    //   - ThreadBreakOnTermination = 18
    //   - ThreadPriority = 0
    //   - ThreadBasePriority = 3
    //   - ThreadAffinityMask = 4
    //   - ThreadImpersonationToken = 5
    //   - ThreadQuerySetWin32StartAddress = 9
    //   - ThreadZeroTlsCell = 16

    // TODO: Read ThreadInformation data based on class and length
    // TODO: Handle ThreadHideFromDebugger (sets thread to not be debugged)
    // TODO: Handle other thread information classes as needed
    // TODO: Validate thread_handle (GetCurrentThread() = -2, real handles > 0)

    log_red!(
        emu,
        "ntdll!NtSetInformationThread handle: 0x{:x} class: {} info_ptr: 0x{:x} length: {}",
        thread_handle,
        thread_info_class,
        thread_info_ptr,
        thread_info_length
    );

    // TODO: Return appropriate NTSTATUS:
    //   - STATUS_SUCCESS = 0x00000000
    //   - STATUS_INVALID_HANDLE = 0xC0000008
    //   - STATUS_INVALID_PARAMETER = 0xC000000D
    //   - STATUS_INFO_LENGTH_MISMATCH = 0xC0000004
    emu.regs_mut().rax = 0x00000000; // STATUS_SUCCESS for now
}
