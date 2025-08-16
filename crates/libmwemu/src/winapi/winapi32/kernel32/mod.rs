use crate::peb::peb32;
use crate::{emu, serialization};

use lazy_static::lazy_static;
use std::sync::Mutex;

mod add_vectored_exception_handler;
mod are_file_apis_a_n_s_i;
mod are_file_apis_ansi;
mod close_handle;
mod connect_named_pipe;
mod copy_file_a;
mod copy_file_w;
mod create_event_a;
mod create_file_mapping_a;
mod create_file_mapping_w;
mod create_file_w;
mod create_mutex_a;
mod create_mutex_w;
mod create_named_pipe_a;
mod create_process_a;
mod create_remote_thread;
mod create_thread;
mod create_toolhelp32_snapshot;
mod crypt_create_hash;
mod decode_pointer;
mod disconnect_named_pipe;
mod encode_pointer;
mod enter_critical_section;
mod exit_process;
mod expand_environment_strings_a;
mod expand_environment_strings_w;
mod file_time_to_dos_date_time;
mod file_time_to_local_file_time;
mod file_time_to_system_time;
mod find_close;
mod find_first_file_a;
mod find_first_file_w;
mod find_next_file_a;
mod find_next_file_w;
mod find_resource_a;
mod find_resource_w;
mod fls_alloc;
mod fls_get_value;
mod fls_set_value;
mod free_library;
mod free_resource;
mod get_a_c_p;
mod get_acp;
mod get_c_p_info;
mod get_command_line_a;
mod get_command_line_w;
mod get_computer_name_a;
mod get_cp_info;
mod get_current_directory_a;
mod get_current_directory_w;
mod get_current_process_id;
mod get_current_process;
mod get_current_thread_id;
mod get_environment_strings_w;
mod get_environment_strings;
mod get_file_attributes_a;
mod get_file_attributes_w;
mod get_file_type;
mod get_full_path_name_a;
mod get_full_path_name_w;
mod get_last_error;
mod get_logical_drives;
mod get_long_path_name_w;
mod get_module_file_name_a;
mod get_module_file_name_w;
mod get_module_handle_a;
mod get_module_handle_w;
mod get_native_system_info;
mod get_o_e_m_c_p;
mod get_oemcp;
mod get_proc_address;
mod get_process_affinity_mask;
mod get_process_heap;
mod get_startup_info_a;
mod get_startup_info_w;
mod get_std_handle;
mod get_string_type_w;
mod get_system_directory_a;
mod get_system_directory_w;
mod get_system_info;
mod get_system_time_as_file_time;
mod get_system_time;
mod get_system_windows_directory_a;
mod get_system_windows_directory_w;
mod get_temp_path_w;
mod get_thread_context;
mod get_thread_preferred_u_i_languages;
mod get_thread_preferred_ui_languages;
mod get_thread_u_i_language;
mod get_thread_ui_language;
mod get_tick_count;
mod get_time_zone_information;
mod get_user_default_lang_i_d;
mod get_user_default_lang_id;
mod get_user_default_u_i_language;
mod get_user_default_ui_language;
mod get_version_ex_w;
mod get_version;
mod get_windows_directory_a;
mod get_windows_directory_w;
mod heap_alloc;
mod heap_create;
mod heap_destroy;
mod heap_free;
mod heap_set_information;
mod initialize_critical_section_and_spin_count;
mod initialize_critical_section_ex;
mod initialize_critical_section;
mod interlocked_increment;
mod is_debugger_present;
mod is_processor_feature_present;
mod is_valid_code_page;
mod is_valid_locale;
mod l_c_map_string_w;
mod lc_map_string_w;
mod leave_critical_section;
mod load_library_a;
mod load_library_ex_a;
mod load_library_ex_w;
mod load_library_w;
mod load_resource;
mod local_alloc;
mod lock_resource;
mod lstrcat;
mod lstrcmp_a;
mod lstrcmp_w;
mod lstrcmp;
mod lstrcpy;
mod lstrlen;
mod map_view_of_file;
mod move_file_a;
mod move_file_w;
mod multi_byte_to_wide_char;
mod open_process_token;
mod open_process;
mod open_thread;
mod query_performance_counter;
mod raise_exception;
mod read_file;
mod read_process_memory;
mod reg_close_key;
mod reg_create_key_ex_a;
mod reg_create_key_ex_w;
mod reg_open_key_a;
mod reg_open_key_ex_w;
mod reg_open_key_w;
mod reg_set_value_ex_a;
mod reg_set_value_ex_w;
mod resume_thread;
mod set_error_mode;
mod set_handle_count;
mod set_last_error;
mod set_thread_context;
mod set_thread_locale;
mod set_unhandled_exception_filter;
mod sizeof_resource;
mod sleep;
mod system_time_to_tz_specific_local_time;
mod terminate_process;
mod thread32_first;
mod thread32_next;
mod tls_alloc;
mod tls_free;
mod tls_get_value;
mod tls_set_value;
mod unhandled_exception_filter;
mod verify_version_info_w;
mod virtual_alloc_ex_numa;
mod virtual_alloc_ex;
mod virtual_alloc;
mod virtual_free;
mod virtual_protect_ex;
mod virtual_protect;
mod virtual_query_ex;
mod virtual_query;
mod wait_for_single_object;
mod wide_char_to_multi_byte;
mod win_exec;
mod write_file;
mod write_process_memory;

pub use add_vectored_exception_handler::*;
pub use are_file_apis_a_n_s_i::*;
pub use close_handle::*;
pub use connect_named_pipe::*;
pub use copy_file_a::*;
pub use copy_file_w::*;
pub use create_event_a::*;
pub use create_file_mapping_a::*;
pub use create_file_mapping_w::*;
pub use create_file_w::*;
pub use create_mutex_a::*;
pub use create_mutex_w::*;
pub use create_named_pipe_a::*;
pub use create_process_a::*;
pub use create_remote_thread::*;
pub use create_thread::*;
pub use create_toolhelp32_snapshot::*;
pub use crypt_create_hash::*;
pub use decode_pointer::*;
pub use disconnect_named_pipe::*;
pub use encode_pointer::*;
pub use enter_critical_section::*;
pub use exit_process::*;
pub use expand_environment_strings_a::*;
pub use expand_environment_strings_w::*;
pub use file_time_to_dos_date_time::*;
pub use file_time_to_local_file_time::*;
pub use file_time_to_system_time::*;
pub use find_close::*;
pub use find_first_file_a::*;
pub use find_first_file_w::*;
pub use find_next_file_a::*;
pub use find_next_file_w::*;
pub use find_resource_a::*;
pub use find_resource_w::*;
pub use fls_alloc::*;
pub use fls_get_value::*;
pub use fls_set_value::*;
pub use free_library::*;
pub use free_resource::*;
pub use get_a_c_p::*;
pub use get_acp::*;
pub use get_c_p_info::*;
pub use get_command_line_a::*;
pub use get_command_line_w::*;
pub use get_computer_name_a::*;
pub use get_current_directory_a::*;
pub use get_current_directory_w::*;
pub use get_current_process_id::*;
pub use get_current_process::*;
pub use get_current_thread_id::*;
pub use get_environment_strings_w::*;
pub use get_environment_strings::*;
pub use get_file_attributes_a::*;
pub use get_file_attributes_w::*;
pub use get_file_type::*;
pub use get_full_path_name_a::*;
pub use get_full_path_name_w::*;
pub use get_last_error::*;
pub use get_logical_drives::*;
pub use get_long_path_name_w::*;
pub use get_module_file_name_a::*;
pub use get_module_file_name_w::*;
pub use get_module_handle_a::*;
pub use get_module_handle_w::*;
pub use get_native_system_info::*;
pub use get_o_e_m_c_p::*;
pub use get_proc_address::*;
pub use get_process_affinity_mask::*;
pub use get_process_heap::*;
pub use get_startup_info_a::*;
pub use get_startup_info_w::*;
pub use get_std_handle::*;
pub use get_string_type_w::*;
pub use get_system_directory_a::*;
pub use get_system_directory_w::*;
pub use get_system_info::*;
pub use get_system_time_as_file_time::*;
pub use get_system_time::*;
pub use get_system_windows_directory_a::*;
pub use get_system_windows_directory_w::*;
pub use get_temp_path_w::*;
pub use get_thread_context::*;
pub use get_thread_preferred_u_i_languages::*;
pub use get_thread_u_i_language::*;
pub use get_tick_count::*;
pub use get_time_zone_information::*;
pub use get_user_default_lang_i_d::*;
pub use get_user_default_u_i_language::*;
//pub use get_user_default_ui_language::*;
pub use get_version_ex_w::*;
pub use get_version::*;
pub use get_windows_directory_a::*;
pub use get_windows_directory_w::*;
pub use heap_alloc::*;
pub use heap_create::*;
pub use heap_destroy::*;
pub use heap_free::*;
pub use heap_set_information::*;
pub use initialize_critical_section_and_spin_count::*;
pub use initialize_critical_section_ex::*;
pub use initialize_critical_section::*;
pub use interlocked_increment::*;
pub use is_debugger_present::*;
pub use is_processor_feature_present::*;
pub use is_valid_code_page::*;
pub use is_valid_locale::*;
pub use l_c_map_string_w::*;
//pub use lc_map_string_w::*;
pub use leave_critical_section::*;
pub use load_library_a::*;
pub use load_library_ex_a::*;
pub use load_library_ex_w::*;
pub use load_library_w::*;
pub use load_resource::*;
pub use local_alloc::*;
pub use lock_resource::*;
pub use lstrcat::*;
pub use lstrcmp_a::*;
pub use lstrcmp_w::*;
//pub use lstrcmp::*;
pub use lstrcpy::*;
pub use lstrlen::*;
pub use map_view_of_file::*;
pub use move_file_a::*;
pub use move_file_w::*;
pub use multi_byte_to_wide_char::*;
pub use open_process_token::*;
pub use open_process::*;
pub use open_thread::*;
pub use query_performance_counter::*;
pub use raise_exception::*;
pub use read_file::*;
pub use read_process_memory::*;
pub use reg_close_key::*;
pub use reg_create_key_ex_a::*;
pub use reg_create_key_ex_w::*;
pub use reg_open_key_a::*;
pub use reg_open_key_ex_w::*;
pub use reg_open_key_w::*;
pub use reg_set_value_ex_a::*;
pub use reg_set_value_ex_w::*;
pub use resume_thread::*;
pub use set_error_mode::*;
pub use set_handle_count::*;
pub use set_last_error::*;
pub use set_thread_context::*;
pub use set_thread_locale::*;
pub use set_unhandled_exception_filter::*;
pub use sizeof_resource::*;
pub use sleep::*;
pub use system_time_to_tz_specific_local_time::*;
pub use terminate_process::*;
pub use thread32_first::*;
pub use thread32_next::*;
pub use tls_alloc::*;
pub use tls_free::*;
pub use tls_get_value::*;
pub use tls_set_value::*;
pub use unhandled_exception_filter::*;
pub use verify_version_info_w::*;
pub use virtual_alloc_ex_numa::*;
pub use virtual_alloc_ex::*;
pub use virtual_alloc::*;
pub use virtual_free::*;
pub use virtual_protect_ex::*;
pub use virtual_protect::*;
pub use virtual_query_ex::*;
pub use virtual_query::*;
pub use wait_for_single_object::*;
pub use wide_char_to_multi_byte::*;
pub use win_exec::*;
pub use write_file::*;
pub use write_process_memory::*;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    match api.as_str() {
        "LoadLibraryA" => LoadLibraryA(emu),
        "LoadLibraryExA" => LoadLibraryExA(emu),
        "LoadLibraryExW" => LoadLibraryExW(emu),
        "GetProcAddress" => GetProcAddress(emu),
        "LoadLibraryW" => LoadLibraryW(emu),
        "WinExec" => WinExec(emu),
        "GetVersion" => GetVersion(emu),
        "CreateProcessA" => CreateProcessA(emu),
        "WaitForSingleObject" => WaitForSingleObject(emu),
        "VirtualAlloc" => VirtualAlloc(emu),
        "VirtualAllocEx" => VirtualAllocEx(emu),
        "WriteProcessMemory" => WriteProcessMemory(emu),
        "CreateRemoteThread" => CreateRemoteThread(emu),
        "CreateNamedPipeA" => CreateNamedPipeA(emu),
        "ConnectNamedPipe" => ConnectNamedPipe(emu),
        "DisconnectNamedPipe" => DisconnectNamedPipe(emu),
        "ReadFile" => ReadFile(emu),
        "WriteFile" => WriteFile(emu),
        "CreateFileW" => CreateFileW(emu),
        "CloseHandle" => CloseHandle(emu),
        "ExitProcess" => ExitProcess(emu),
        "TerminateProcess" => TerminateProcess(emu),
        "GetThreadContext" => GetThreadContext(emu),
        "ReadProcessMemory" => ReadProcessMemory(emu),
        "GetCurrentDirectoryW" => GetCurrentDirectoryW(emu),
        "GetCurrentDirectoryA" => GetCurrentDirectoryA(emu),
        "VirtualProtect" => VirtualProtect(emu),
        "VirtualProtectEx" => VirtualProtectEx(emu),
        "ResumeThread" => ResumeThread(emu),
        "GetFullPathNameA" => GetFullPathNameA(emu),
        "GetFullPathNameW" => GetFullPathNameW(emu),
        "SystemTimeToTzSpecificLocalTime" => SystemTimeToTzSpecificLocalTime(emu),
        "GetLogicalDrives" => GetLogicalDrives(emu),
        "ExpandEnvironmentStringsA" => ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => ExpandEnvironmentStringsW(emu),
        "GetFileAttributesA" => GetFileAttributesA(emu),
        "GetFileAttributesW" => GetFileAttributesW(emu),
        "FileTimeToSystemTime" => FileTimeToSystemTime(emu),
        "FindFirstFileA" => FindFirstFileA(emu),
        "FindNextFileA" => FindNextFileA(emu),
        "FindFirstFileW" => FindFirstFileW(emu),
        "FindNextFileW" => FindNextFileW(emu),
        "CopyFileA" => CopyFileA(emu),
        "CopyFileW" => CopyFileW(emu),
        "FindClose" => FindClose(emu),
        "MoveFileA" => MoveFileA(emu),
        "MoveFileW" => MoveFileW(emu),
        "OpenProcess" => OpenProcess(emu),
        "GetCurrentProcessId" => GetCurrentProcessId(emu),
        "Thread32First" => Thread32First(emu),
        "Thread32Next" => Thread32Next(emu),
        "OpenThread" => OpenThread(emu),
        "CreateToolhelp32Snapshot" => CreateToolhelp32Snapshot(emu),
        "CreateThread" => CreateThread(emu),
        "SetThreadContext" => SetThreadContext(emu),
        "MapViewOfFile" => MapViewOfFile(emu),
        "GetSystemTimeAsFileTime" => GetSystemTimeAsFileTime(emu),
        "GetCurrentThreadId" => GetCurrentThreadId(emu),
        "GetTickCount" => GetTickCount(emu),
        "QueryPerformanceCounter" => QueryPerformanceCounter(emu),
        "HeapCreate" => HeapCreate(emu),
        "HeapDestroy" => HeapDestroy(emu),
        "GetModuleHandleA" => GetModuleHandleA(emu),
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "TlsAlloc" => TlsAlloc(emu),
        "TlsSetValue" => TlsSetValue(emu),
        "TlsGetValue" => TlsGetValue(emu),
        "TlsFree" => TlsFree(emu),
        "EncodePointer" => EncodePointer(emu),
        "DecodePointer" => DecodePointer(emu),
        "Sleep" => Sleep(emu),
        "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount(emu),
        "HeapAlloc" => HeapAlloc(emu),
        "GetProcessAffinityMask" => GetProcessAffinityMask(emu),
        "IsDebuggerPresent" => IsDebuggerPresent(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "UnhandledExceptionFilter" => UnhandledExceptionFilter(emu),
        "GetCurrentProcess" => GetCurrentProcess(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "VirtualAllocExNuma" => VirtualAllocExNuma(emu),
        "GetUserDefaultLangID" => GetUserDefaultLangID(emu),
        "GetProcessHeap" => GetProcessHeap(emu),
        "GetComputerNameA" => GetComputerNameA(emu),
        "CreateMutexA" => CreateMutexA(emu),
        "GetLastError" => GetLastError(emu),
        "CreateFileMappingA" => CreateFileMappingA(emu),
        "CreateFileMappingW" => CreateFileMappingW(emu),
        "GetSystemTime" => GetSystemTime(emu),
        "lstrcat" => lstrcat(emu),
        "SetErrorMode" => SetErrorMode(emu),
        "GetVersionExW" => GetVersionExW(emu),
        "GetSystemDirectoryA" => GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => GetSystemDirectoryW(emu),
        "GetStartupInfoA" => GetStartupInfoA(emu),
        "GetStartupInfoW" => GetStartupInfoW(emu),
        "FlsGetValue" => FlsGetValue(emu),
        "IsProcessorFeaturePresent" => IsProcessorFeaturePresent(emu),
        "InitializeCriticalSection" => InitializeCriticalSection(emu),
        "InitializeCriticalSectionEx" => InitializeCriticalSectionEx(emu),
        "FlsAlloc" => FlsAlloc(emu),
        "FlsSetValue" => FlsSetValue(emu),
        "SetLastError" => SetLastError(emu),
        "lstrlen" => lstrlen(emu),
        "MultiByteToWideChar" => MultiByteToWideChar(emu),
        "GetSystemInfo" => GetSystemInfo(emu),
        "HeapFree" => HeapFree(emu),
        "SetThreadLocale" => SetThreadLocale(emu),
        "GetCommandLineA" => GetCommandLineA(emu),
        "GetCommandLineW" => GetCommandLineW(emu),
        "GetAcp" => GetAcp(emu),
        "GetModuleFileNameW" => GetModuleFileNameW(emu),
        "RegOpenKeyExW" => RegOpenKeyExW(emu),
        "GetUserDefaultUILanguage" => GetUserDefaultUILanguage(emu),
        "EnterCriticalSection" => EnterCriticalSection(emu),
        "LeaveCriticalSection" => LeaveCriticalSection(emu),
        "IsValidLocale" => IsValidLocale(emu),
        "GetThreadUILanguage" => GetThreadUILanguage(emu),
        "GetThreadPreferredUILanguages" => GetThreadPreferredUILanguages(emu),
        "lstrcmp" => lstrcmpA(emu),
        "lstrcmpA" => lstrcmpA(emu),
        "lstrcmpW" => lstrcmpW(emu),
        "GetNativeSystemInfo" => GetNativeSystemInfo(emu),
        "GetTempPathW" => GetTempPathW(emu),
        "FileTimeToLocalFileTime" => FileTimeToLocalFileTime(emu),
        "FileTimeToDosDateTime" => FileTimeToDosDateTime(emu),
        "CreateMutexW" => CreateMutexW(emu),
        "VirtualQuery" => VirtualQuery(emu),
        "VirtualFree" => VirtualFree(emu),
        "RaiseException" => RaiseException(emu),
        "VerifyVersionInfoW" => VerifyVersionInfoW(emu),
        "GetTimeZoneInformation" => GetTimeZoneInformation(emu),
        "VirtualQueryEx" => VirtualQueryEx(emu),
        "InterlockedIncrement" => InterlockedIncrement(emu),
        "GetEnvironmentStrings" => GetEnvironmentStrings(emu),
        "GetEnvironmentStringsW" => GetEnvironmentStringsW(emu),
        "GetStdHandle" => GetStdHandle(emu),
        "GetFileType" => GetFileType(emu),
        "SetHandleCount" => SetHandleCount(emu),
        "IsValidCodePage" => IsValidCodePage(emu),
        "GetCPInfo" => GetCPInfo(emu),
        "GetStringTypeW" => GetStringTypeW(emu),
        "LCMapStringW" => LCMapStringW(emu),
        "WideCharToMultiByte" => WideCharToMultiByte(emu),
        "CryptCreateHash" => CryptCreateHash(emu),
        "HeapSetInformation" => HeapSetInformation(emu),
        "OpenProcessToken" => OpenProcessToken(emu),
        "CreateEventA" => CreateEventA(emu),
        "AddVectoredExceptionHandler" => AddVectoredExceptionHandler(emu),
        "GetLongPathNameW" => GetLongPathNameW(emu),
        "FreeLibrary" => FreeLibrary(emu),
        "AreFileApisANSI" => AreFileApisANSI(emu),
        "GetModuleFileNameA" => GetModuleFileNameA(emu),
        "lstrcpy" => lstrcpy(emu),
        "GetACP" => GetACP(emu),
        "GetOEMCP" => GetOEMCP(emu),
        "GetWindowsDirectoryA" => GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => GetWindowsDirectoryW(emu),
        "GetSystemWindowsDirectoryA" => GetSystemWindowsDirectoryA(emu),
        "GetSystemWindowsDirectoryW" => GetSystemWindowsDirectoryW(emu),
        "RegCreateKeyExA" => RegCreateKeyExA(emu),
        "RegCreateKeyExW" => RegCreateKeyExW(emu),
        "RegSetValueExA" => RegSetValueExA(emu),
        "RegSetValueExW" => RegSetValueExW(emu),
        "RegCloseKey" => RegCloseKey(emu),
        "RegOpenKeyA" => RegOpenKeyA(emu),
        "RegOpenKeyW" => RegOpenKeyW(emu),
        "FindResourceA" => FindResourceA(emu),
        "FindResourceW" => FindResourceW(emu),
        "LoadResource" => LoadResource(emu),
        "SizeofResource" => SizeofResource(emu),
        "LockResource" => LockResource(emu),
        "FreeResource" => FreeResource(emu),

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

lazy_static! {
    static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    static ref LAST_ERROR: Mutex<u32> = Mutex::new(0);
}

/// kernel32 API ////

pub fn dump_module_iat(emu: &mut emu::Emu, module: &str) {
    let mut flink = peb32::Flink::new(emu);
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
    let mut flink = peb32::Flink::new(emu);
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
                    let apiname = ordinal.func_name.to_string();
                    return apiname;
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
    let mut flink = peb32::Flink::new(emu);
    flink.load(emu);
    let first_ptr = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }

                let ordinal = flink.get_function_ordinal(emu, i);
                if ordinal.func_name == name {
                    //if ordinal.func_name.contains(name) {
                    return ordinal.func_va;
                }
            }
        }
        flink.next(emu);

        //log::info!("flink: 0x{:x} first_ptr: 0x{:x} num_of_funcs: {}", flink.get_ptr(), first_ptr, flink.num_of_funcs);

        if flink.get_ptr() == first_ptr {
            break;
        }
    }

    0 //TODO: use Option<>
}

pub fn search_api_name(emu: &mut emu::Emu, name: &str) -> (u64, String, String) {
    let mut flink = peb32::Flink::new(emu);
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

pub fn guess_api_name(emu: &mut emu::Emu, addr: u32) -> String {
    let mut flink = peb32::Flink::new(emu);
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

                if ordinal.func_va == addr as u64 {
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

    match peb32::get_module_base(&dll, emu) {
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
            if std::path::Path::new(dll_path.as_str()).exists() {
                let (base, pe_off) = emu.load_pe32(&dll_path, false, 0);
                peb32::dynamic_link_module(base as u64, pe_off, &dll, emu);
                base as u64
            } else {
                panic!("dll {} not found.", dll_path);
            }
        }
    }
}
