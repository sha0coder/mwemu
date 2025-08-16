mod kernel32;
use kernel32::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::constants;
use crate::emu;
use crate::thread_context::ThreadContext;
use crate::peb::peb64;
use crate::serialization;
use crate::structures;
use crate::winapi::winapi32::helper;
use crate::context::context64;

// a in RCX, b in RDX, c in R8, d in R9, then e pushed on stack

fn clear_last_error(emu: &mut emu::Emu) {
    let mut err = LAST_ERROR.lock().unwrap();
    *err = constants::ERROR_SUCCESS;
}

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    match api.as_str() {
        "FindActCtxSectionStringW" => FindActCtxSectionStringW(emu),
        "lstrcmpiW" => LStrCmpIW(emu),
        "VirtualAlloc" => VirtualAlloc(emu),
        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(&emu, emu.cfg.dump_filename.as_ref().unwrap());
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!("calling unimplemented API 0x{:x} {} at 0x{:x}", addr, api, emu.regs().rip);
            return api;
        }
    }

    String::new()
}

lazy_static! {
    static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    static ref LAST_ERROR: Mutex<u64> = Mutex::new(0);
}

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.mod_name.to_lowercase().contains(module) && flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                log::info!(
                    "0x{:x} {}!{}",
                    ordinal.func_va,
                    &flink.mod_name,
                    &ordinal.func_name
                );
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }
}

pub fn resolve_api_addr_to_name(emu: &mut emu::Emu, addr: u64) -> String {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_va == addr {
                    let s = ordinal.func_name.to_string();
                    return s;
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    "".to_string()
}

pub fn resolve_api_name(emu: &mut emu::Emu, name: &str) -> u64 {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            //println!("export_table_rva: 0x{:x}", flink.export_table_rva);
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.to_lowercase() == name.to_lowercase() {
                    //if ordinal.func_name.contains(name) {
                    //println!("found api id:{} name: 0x{:x} {}!{}", i, ordinal.func_va, flink.mod_name, ordinal.func_name);
                    return ordinal.func_va;
                }
            }
        }
        flink.next(emu);

        //log::info!("flink: 0x{:x} first_ptr: 0x{:x}", flink.get_ptr(), first_ptr);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0 //TODO: use Option<>
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name.contains(name) {
                    return (
                        ordinal.func_va,
                        flink.mod_name.clone(),
                        ordinal.func_name.clone(),
                    );
                }
            }
        }
        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    (0, String::new(), String::new()) //TODO: use Option<>
}

pub fn guess_api_name(emu: &mut emu::Emu, addr: u64) -> String {
    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        //let mod_name = flink.mod_name.clone();

        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);

                if ordinal.func_va == addr {
                    return ordinal.func_name.clone();
                }
            }
        }

        flink.next(emu);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    "function not found".to_string()
}

pub fn load_library(emu: &mut emu::Emu, libname: &str) -> u64 {
    // log::info!("kern32!load_library: {}", libname);

    let mut dll = libname.to_string().to_lowercase();

    if dll.is_empty() {
        emu.regs_mut().rax = 0;
        return 0;
    }

    if !dll.ends_with(".dll") {
        dll.push_str(".dll");
    }

    let mut dll_path = emu.cfg.maps_folder.clone();
    dll_path.push('/');
    dll_path.push_str(&dll);

    println!("dll_path: {} dll: {}", dll_path, dll);

    match peb64::get_module_base(&dll, emu) {
        Some(base) => {
            // already linked
            /*
            if emu.cfg.verbose > 0 {
                log::info!("dll {} already linked.", dll);
            }*/
            base
        }
        None => {
            // do link
            if std::path::Path::new(&dll_path).exists() {
                let (base, pe_off) = emu.load_pe64(&dll_path, false, 0);
                peb64::dynamic_link_module(base, pe_off, &dll, emu);
                return base;
            } else {
                panic!("dll {} not found.", dll_path);
            }
        }
    }
}

pub fn get_library_handle(emu: &mut emu::Emu, libname: &str) -> u64 {
    // log::info!("kern32!load_library: {}", libname);

    let mut dll = libname.to_string().to_lowercase();

    if dll.is_empty() {
        emu.regs_mut().rax = 0;
        return 0;
    }

    if !dll.ends_with(".dll") {
        dll.push_str(".dll");
    }

    let mut dll_path = emu.cfg.maps_folder.clone();
    dll_path.push('/');
    dll_path.push_str(&dll);

    match peb64::get_module_base(&dll, emu) {
        Some(base) => {
            return base;
        }
        None => {
            // if is not linked, don't link, this is not a load_library
            return 0;
        }
    }
}


















pub fn VirtualAlloc(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let typ = emu.regs().r8 as u32;
    let prot = emu.regs().r9 as u32;
    let mem_reserve = (typ & constants::MEM_RESERVE) != 0;
    let mem_commit = (typ & constants::MEM_COMMIT) != 0;
    let mut base:u64 = 0;

    if size == 0 {
        log::info!(
            "{}** {} kernel32!VirtualAlloc addr: 0x{:x} sz: {} = 0 flags: {} {}",
            emu.colors.light_red,
            emu.pos,
            addr,
            size,
            typ,
            emu.colors.nc
        );
        emu.regs_mut().rax = 0
    } else {

        let is_allocated = emu.maps.is_allocated(addr);
        let status_already_allocated = mem_commit && addr > 0 && is_allocated;
        let status_error = !status_already_allocated && mem_commit && addr > 0 && !is_allocated;
        let status_need_allocate = mem_reserve || (mem_commit && addr == 0);
        let status_other = !status_already_allocated && !status_error && !status_need_allocate;


        if status_need_allocate {
            if addr == 0 {
                base = emu
                    .maps
                    .alloc(size)
                    .unwrap_or_else(|| panic!("kernel32!VirtualAlloc out of memory size:{}", size));
            } else {
                base = addr;
            }

            emu.maps
                .create_map(format!("alloc_{:x}", base).as_str(), base, size)
                .expect("kernel32!VirtualAlloc out of memory");

        } else if status_already_allocated {
            base = addr;
        } else if status_error {
            base = 0;
        } else if status_other {
            log::info!("Weird flags on VirtualAlloc");
            base = 0;
        }

        log::info!(
            "{}** {} kernel32!VirtualAlloc addr: 0x{:x} sz: {}  flags: {} =0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            addr,
            size,
            typ,
            base,
            emu.colors.nc
        );

        emu.regs_mut().rax = base;
    }
}














fn advance_tick(emu: &mut emu::Emu, millis: u64) {
    let time_advance = if millis == 0 {
        // Sleep(0) just yields CPU - advance by small amount (microseconds)
        1 + (emu.tick % 3) // 1-3 ticks of variance
    } else {
        millis as usize
    };

    emu.tick += time_advance;
}










/*
BOOL ReadFile(
  [in]                HANDLE       hFile,
  [out]               LPVOID       lpBuffer,
  [in]                DWORD        nNumberOfBytesToRead,
  [out, optional]     LPDWORD      lpNumberOfBytesRead,
  [in, out, optional] LPOVERLAPPED lpOverlapped
);
*/









/*
DWORD GetCurrentDirectoryW(
  [in]  DWORD  nBufferLength,     // ← First parameter
  [out] LPWSTR lpBuffer          // ← Second parameter  
);
*/





































































pub fn FindActCtxSectionStringW(emu: &mut emu::Emu) {
    let actctx = emu.regs().rcx;
    let section_name_ptr = emu.regs().rdx;
    let string_name_ptr = emu.regs().r8;
    let string_value_ptr = emu.regs().r9;
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20)
        .expect("error reading out_ptr");

    let mut section_name = String::new();
    let mut string_name = String::new();
    let mut string_value = String::new();

    if section_name_ptr > 0 {
        section_name = emu.maps.read_wide_string(section_name_ptr);
    }
    if string_name_ptr > 0 {
        string_name = emu.maps.read_wide_string(string_name_ptr);
    }
    if string_value_ptr > 0 {
        string_value = emu.maps.read_wide_string(string_value_ptr);
    }

    let actctx_section_keyed_data = structures::ActCtxSectionKeyedData64::new();
    actctx_section_keyed_data.save(out_ptr, &mut emu.maps);

    log::info!(
        "{}** {} kernel32!FindActCtxSectionStringW section_name: {} string_name: {} string_value: {} {}",
        emu.colors.light_red, emu.pos, section_name, string_name, string_value, emu.colors.nc
    );

    emu.regs_mut().rax = 1;
}



/*
DWORD TlsAlloc();
*/

/*
BOOL TlsFree(
  [in] DWORD dwTlsIndex
);
*/

/*
BOOL TlsSetValue(
  [in]           DWORD  dwTlsIndex,
  [in, optional] LPVOID lpTlsValue
);
*/

/*
DWORD TlsGetValue(
  [in] DWORD dwTlsIndex
);
*/

/*
UINT GetACP();
*/
// TODO: there is GetAcp and GetACP?

/*
HANDLE GetStdHandle(
  [in] DWORD nStdHandle
);
*/





/*
BOOL GetCPInfo(
  [in]  UINT     CodePage,
  [out] LPCPINFO lpCPInfo
);
*/

/*
DWORD GetUserDefaultLCID();
*/

/*
BOOL SetThreadLocale(
  [in] LCID Locale
);
*/

/*
DWORD GetThreadLocale();
*/

/*
int GetLocaleInfoA(
  [in]            LCID   Locale,
  [in]            LCTYPE LCType,
  [out, optional] LPSTR  lpLCData,
  [in]            int    cchData
);
*/

/*
int GetLocaleInfoW(
  [in]            LCID   Locale,
  [in]            LCTYPE LCType,
  [out, optional] LPWSTR lpLCData,
  [in]            int    cchData
);
*/

/*
int WideCharToMultiByte(
  [in]            UINT                               CodePage, rcx
  [in]            DWORD                              dwFlags, rdx
  [in]            _In_NLS_string_(cchWideChar)LPCWCH lpWideCharStr, r8
  [in]            int                                cchWideChar, r9
  [out, optional] LPSTR                              lpMultiByteStr, rsp
  [in]            int                                cbMultiByte, rsp+8
  [in, optional]  LPCCH                              lpDefaultChar, rsp+16
  [out, optional] LPBOOL                             lpUsedDefaultChar, rsp+24
);
*/

/*
int MultiByteToWideChar(
  [in]            UINT                              CodePage,
  [in]            DWORD                             dwFlags,
  [in]            _In_NLS_string_(cbMultiByte)LPCCH lpMultiByteStr,
  [in]            int                               cbMultiByte,
  [out, optional] LPWSTR                            lpWideCharStr,
  [in]            int                               cchWideChar
);

TODO: recheck return logic, and test with enigma packed binary.
https://learn.microsoft.com/en-us/windows/win32/api/stringapiset/nf-stringapiset-multibytetowidechar
*/


/*
UINT GetWindowsDirectoryA(
  [out] LPSTR lpBuffer,
  [in]  UINT  uSize
);
*/


/*
BOOL ResetEvent(
  [in] HANDLE hEvent
);
*/

/*
BOOL VirtualFree(
  [in] LPVOID lpAddress,
  [in] SIZE_T dwSize,
  [in] DWORD  dwFreeType
);
*/

/*
DWORD GetModuleFileNameW(
  [in, optional] HMODULE hModule,
  [out]          LPWSTR  lpFilename,
  [in]           DWORD   nSize
);
*/



/*
ATOM GlobalAddAtomA(
  [in] LPCSTR lpString
);
*/


/*
HRSRC FindResourceA(
  [in, optional] HMODULE hModule,
  [in]           LPCSTR  lpName,
  [in]           LPCSTR  lpType
);
*/







/*
BOOL SetCurrentDirectory(
  [in] LPCTSTR lpPathName
);
*/

/*
HANDLE CreateFileA(
  [in]           LPCSTR                lpFileName,
  [in]           DWORD                 dwDesiredAccess,
  [in]           DWORD                 dwShareMode,
  [in, optional] LPSECURITY_ATTRIBUTES lpSecurityAttributes,
  [in]           DWORD                 dwCreationDisposition,
  [in]           DWORD                 dwFlagsAndAttributes,
  [in, optional] HANDLE                hTemplateFile
);
*/

/*
DWORD GetFileSize(
  [in]            HANDLE  hFile,
  [out, optional] LPDWORD lpFileSizeHigh
);
*/

/*
BOOL IsBadReadPtr(
  [in] const VOID *lp,
  [in] UINT_PTR   ucb
);
*/


/*
BOOL SetThreadStackGuarantee(
  [in, out] PULONG StackSizeInBytes
);
*/

/*
HANDLE GetCurrentThread();
*/

/*
BOOL WINAPI GetConsoleMode(
  _In_  HANDLE  hConsoleHandle,
  _Out_ LPDWORD lpMode
);
*/


/*
BOOL WINAPI WriteConsoleW(
  _In_             HANDLE  hConsoleOutput,
  _In_       const wchar_t    *lpBuffer,
  _In_             DWORD   nNumberOfCharsToWrite,
  _Out_opt_        LPDWORD lpNumberOfCharsWritten,
  _Reserved_       LPVOID  lpReserved
);
*/

/*
HANDLE CreateActCtxA(
  [in, out] PCACTCTXA pActCtx
);
*/

/*
BOOL ActivateActCtx(
  [in]  HANDLE    hActCtx,
  [out] ULONG_PTR *lpCookie
);
*/

/*
DECLSPEC_ALLOCATOR LPVOID HeapReAlloc(
  [in] HANDLE                 hHeap,
  [in] DWORD                  dwFlags,
  [in] _Frees_ptr_opt_ LPVOID lpMem,
  [in] SIZE_T                 dwBytes
);
*/

/*
BOOL InitOnceBeginInitialize(
  [in, out]       LPINIT_ONCE lpInitOnce,
  [in]            DWORD       dwFlags,
  [out]           PBOOL       fPending,
  [out, optional] LPVOID      *lpContext
);
*/

/*
DWORD GetEnvironmentVariableW(
  [in, optional]  LPCWSTR lpName,
  [out, optional] LPWSTR  lpBuffer,
  [in]            DWORD   nSize
);
*/





