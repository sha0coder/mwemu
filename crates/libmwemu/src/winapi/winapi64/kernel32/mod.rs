use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::constants;
use crate::emu;
use crate::emu::Emu;
use crate::peb::peb64;
use crate::serialization;

pub mod activate_act_ctx;
pub mod add_vectored_exception_handler;
pub mod are_file_api_is_ansi;
pub mod begin_update_resource_a;
pub mod close_handle;
pub mod compare_string_w;
pub mod connect_named_pipe;
pub mod copy_file_a;
pub mod copy_file_w;
pub mod create_act_ctx_a;
pub mod create_event_a;
pub mod create_file_a;
pub mod create_file_mapping_a;
pub mod create_file_mapping_w;
pub mod create_mutex_a;
pub mod create_mutex_w;
pub mod create_named_pipe_a;
pub mod create_named_pipe_w;
pub mod create_process_a;
pub mod create_process_w;
pub mod create_remote_thread;
pub mod create_thread;
pub mod create_toolhelp32_snapshot;
pub mod decode_pointer;
pub mod disconnect_named_pipe;
pub mod encode_pointer;
pub mod enter_critical_section;
pub mod exit_process;
pub mod expand_environment_strings_a;
pub mod expand_environment_strings_w;
pub mod file_time_to_system_time;
pub mod find_act_ctx_section_string_w;
pub mod find_close;
pub mod find_first_file_a;
pub mod find_first_file_ex_w;
pub mod find_first_file_w;
pub mod find_next_file_a;
pub mod find_next_file_w;
pub mod find_resource_a;
pub mod find_resource_w;
pub mod fls_alloc;
pub mod fls_get_value;
pub mod fls_set_value;
pub mod free_resource;
pub mod get_acp;
pub mod get_command_line_a;
pub mod get_command_line_w;
pub mod get_computer_name_a;
pub mod get_computer_name_w;
pub mod get_console_cp;
pub mod get_console_mode;
pub mod get_console_output_cp;
pub mod get_cp_info;
pub mod get_current_directory_a;
pub mod get_current_directory_w;
pub mod get_current_process;
pub mod get_current_process_id;
pub mod get_current_thread;
pub mod get_current_thread_id;
pub mod get_environment_strings_w;
pub mod get_environment_variable_w;
pub mod get_file_attributes_a;
pub mod get_file_attributes_w;
pub mod get_file_size;
pub mod get_full_path_name_a;
pub mod get_full_path_name_w;
pub mod get_last_error;
pub mod get_local_time;
pub mod get_locale_info_a;
pub mod get_locale_info_w;
pub mod get_logical_drives;
pub mod get_module_file_name_a;
pub mod get_module_file_name_w;
pub mod get_module_handle_a;
pub mod get_module_handle_w;
pub mod get_native_system_info;
pub mod get_proc_address;
pub mod get_process_affinity_mask;
pub mod get_process_heap;
pub mod get_startup_info_a;
pub mod get_startup_info_w;
pub mod get_std_handle;
pub mod get_system_directory_a;
pub mod get_system_directory_w;
pub mod get_system_firmware_table;
pub mod get_system_info;
pub mod get_system_time;
pub mod get_system_time_as_file_time;
pub mod get_temp_path_w;
pub mod get_thread_context;
pub mod get_thread_locale;
pub mod get_tick_count;
pub mod get_user_default_lang_id;
pub mod get_user_default_lcid;
pub mod get_version;
pub mod get_version_ex_a;
pub mod get_version_ex_w;
pub mod get_windows_directory_a;
pub mod get_windows_directory_w;
pub mod global_add_atom_a;
pub mod heap_alloc;
pub mod heap_create;
pub mod heap_free;
pub mod heap_re_alloc;
pub mod init_once_begin_initialize;
pub mod initialize_critical_section;
pub mod initialize_critical_section_and_spin_count;
pub mod initialize_critical_section_ex;
pub mod is_bad_read_ptr;
pub mod is_debugger_present;
pub mod is_processor_feature_present;
pub mod leave_critical_section;
pub mod load_library_a;
pub mod load_library_ex_a;
pub mod load_library_ex_w;
pub mod load_library_w;
pub mod load_resource;
pub mod local_alloc;
pub mod lock_resource;
pub mod lstrcat_a;
pub mod lstrcat_w;
pub mod lstrcmpi;
pub mod lstrcmpi_w;
pub mod lstrcpy;
pub mod lstrcpy_w;
pub mod lstrcpyn;
pub mod lstrlen_a;
pub mod lstrlen_w;
pub mod map_view_of_file;
pub mod move_file_a;
pub mod move_file_w;
pub mod multi_byte_to_wide_char;
pub mod open_process;
pub mod open_thread;
pub mod process32_first;
pub mod process32_next;
pub mod query_performance_counter;
pub mod read_file;
pub mod read_process_memory;
pub mod reset_event;
pub mod resume_thread;
pub mod set_current_directory_a;
pub mod set_error_mode;
pub mod set_last_error;
pub mod set_thread_locale;
pub mod set_thread_stack_guarantee;
pub mod set_unhandled_exception_filter;
pub mod sizeof_resource;
pub mod sleep;
pub mod system_time_to_file_time;
pub mod system_time_to_tz_specific_local_time;
pub mod terminate_process;
pub mod thread32_first;
pub mod thread32_next;
pub mod tls_alloc;
pub mod tls_free;
pub mod tls_get_value;
pub mod tls_set_value;
pub mod unhandled_exception_filter;
pub mod virtual_alloc;
pub mod virtual_alloc_ex;
pub mod virtual_alloc_ex_numa;
pub mod virtual_free;
pub mod virtual_lock;
pub mod virtual_protect;
pub mod virtual_protect_ex;
pub mod wait_for_single_object;
pub mod wide_char_to_multi_byte;
pub mod win_exec;
pub mod write_console_w;
pub mod write_file;
pub mod write_process_memory;
pub mod device_io_control;
mod local_free;

// Re-export all functions
pub use activate_act_ctx::ActivateActCtx;
pub use add_vectored_exception_handler::AddVectoredExceptionHandler;
pub use are_file_api_is_ansi::AreFileApiIsAnsi;
pub use begin_update_resource_a::BeginUpdateResourceA;
pub use close_handle::CloseHandle;
pub use compare_string_w::CompareStringW;
pub use connect_named_pipe::ConnectNamedPipe;
pub use copy_file_a::CopyFileA;
pub use copy_file_w::CopyFileW;
pub use create_act_ctx_a::CreateActCtxA;
pub use create_event_a::CreateEventA;
pub use create_file_a::CreateFileA;
pub use create_file_mapping_a::CreateFileMappingA;
pub use create_file_mapping_w::CreateFileMappingW;
pub use create_mutex_a::CreateMutexA;
pub use create_mutex_w::CreateMutexW;
pub use create_named_pipe_a::CreateNamedPipeA;
pub use create_named_pipe_w::CreateNamedPipeW;
pub use create_process_a::CreateProcessA;
pub use create_process_w::CreateProcessW;
pub use create_remote_thread::CreateRemoteThread;
pub use create_thread::CreateThread;
pub use create_toolhelp32_snapshot::CreateToolhelp32Snapshot;
pub use decode_pointer::DecodePointer;
pub use disconnect_named_pipe::DisconnectNamedPipe;
pub use encode_pointer::EncodePointer;
pub use enter_critical_section::EnterCriticalSection;
pub use exit_process::ExitProcess;
pub use expand_environment_strings_a::ExpandEnvironmentStringsA;
pub use expand_environment_strings_w::ExpandEnvironmentStringsW;
pub use file_time_to_system_time::FileTimeToSystemTime;
pub use find_act_ctx_section_string_w::FindActCtxSectionStringW;
pub use find_close::FindClose;
pub use find_first_file_a::FindFirstFileA;
pub use find_first_file_ex_w::FindFirstFileExW;
pub use find_first_file_w::FindFirstFileW;
pub use find_next_file_a::FindNextFileA;
pub use find_next_file_w::FindNextFileW;
pub use find_resource_a::FindResourceA;
pub use find_resource_w::FindResourceW;
pub use fls_alloc::FlsAlloc;
pub use fls_get_value::FlsGetValue;
pub use fls_set_value::FlsSetValue;
pub use free_resource::FreeResource;
pub use get_acp::GetACP;
pub use get_command_line_a::GetCommandLineA;
pub use get_command_line_w::GetCommandLineW;
pub use get_computer_name_a::GetComputerNameA;
pub use get_computer_name_w::GetComputerNameW;
pub use get_console_cp::GetConsoleCP;
pub use get_console_mode::GetConsoleMode;
pub use get_console_output_cp::GetConsoleOutputCP;
pub use get_cp_info::GetCPInfo;
pub use get_current_directory_a::GetCurrentDirectoryA;
pub use get_current_directory_w::GetCurrentDirectoryW;
pub use get_current_process::GetCurrentProcess;
pub use get_current_process_id::GetCurrentProcessId;
pub use get_current_thread::GetCurrentThread;
pub use get_current_thread_id::GetCurrentThreadId;
pub use get_environment_strings_w::GetEnvironmentStringsW;
pub use get_environment_variable_w::GetEnvironmentVariableW;
pub use get_file_attributes_a::GetFileAttributesA;
pub use get_file_attributes_w::GetFileAttributesW;
pub use get_file_size::GetFileSize;
pub use get_full_path_name_a::GetFullPathNameA;
pub use get_full_path_name_w::GetFullPathNameW;
pub use get_last_error::GetLastError;
pub use get_local_time::GetLocalTime;
pub use get_locale_info_a::GetLocaleInfoA;
pub use get_locale_info_w::GetLocaleInfoW;
pub use get_logical_drives::GetLogicalDrives;
pub use get_module_file_name_a::GetModuleFileNameA;
pub use get_module_file_name_w::GetModuleFileNameW;
pub use get_module_handle_a::GetModuleHandleA;
pub use get_module_handle_w::GetModuleHandleW;
pub use get_native_system_info::GetNativeSystemInfo;
pub use get_proc_address::GetProcAddress;
pub use get_process_affinity_mask::GetProcessAffinityMask;
pub use get_process_heap::GetProcessHeap;
pub use get_startup_info_a::GetStartupInfoA;
pub use get_startup_info_w::GetStartupInfoW;
pub use get_std_handle::GetStdHandle;
pub use get_system_directory_a::GetSystemDirectoryA;
pub use get_system_directory_w::GetSystemDirectoryW;
pub use get_system_firmware_table::GetSystemFirmwareTable;
pub use get_system_info::GetSystemInfo;
pub use get_system_time::GetSystemTime;
pub use get_system_time_as_file_time::GetSystemTimeAsFileTime;
pub use get_temp_path_w::GetTempPathW;
pub use get_thread_context::GetThreadContext;
pub use get_thread_locale::GetThreadLocale;
pub use get_tick_count::GetTickCount;
pub use get_user_default_lang_id::GetUserDefaultLangId;
pub use get_user_default_lcid::GetUserDefaultLCID;
pub use get_version::GetVersion;
pub use get_version_ex_a::GetVersionExA;
pub use get_version_ex_w::GetVersionExW;
pub use get_windows_directory_a::GetWindowsDirectoryA;
pub use get_windows_directory_w::GetWindowsDirectoryW;
pub use global_add_atom_a::GlobalAddAtomA;
pub use heap_alloc::HeapAlloc;
pub use heap_create::HeapCreate;
pub use heap_free::HeapFree;
pub use heap_re_alloc::HeapReAlloc;
pub use init_once_begin_initialize::InitOnceBeginInitialize;
pub use initialize_critical_section::InitializeCriticalSection;
pub use initialize_critical_section_and_spin_count::InitializeCriticalSectionAndSpinCount;
pub use initialize_critical_section_ex::InitializeCriticalSectionEx;
pub use is_bad_read_ptr::IsBadReadPtr;
pub use is_debugger_present::IsDebuggerPresent;
pub use is_processor_feature_present::IsProcessorFeaturePresent;
pub use leave_critical_section::LeaveCriticalSection;
pub use load_library_a::LoadLibraryA;
pub use load_library_ex_a::LoadLibraryExA;
pub use load_library_ex_w::LoadLibraryExW;
pub use load_library_w::LoadLibraryW;
pub use load_resource::LoadResource;
pub use local_alloc::LocalAlloc;
pub use lock_resource::LockResource;
pub use lstrcat_a::lstrcatA;
pub use lstrcat_w::lstrcatW;
pub use lstrcmpi::LStrCmpI;
pub use lstrcmpi_w::LStrCmpIW;
pub use lstrcpy::lstrcpy;
pub use lstrcpy_w::lstrcpyW;
pub use lstrcpyn::lstrcpyn;
pub use lstrlen_a::lstrlenA;
pub use lstrlen_w::lstrlenW;
pub use map_view_of_file::MapViewOfFile;
pub use move_file_a::MoveFileA;
pub use move_file_w::MoveFileW;
pub use multi_byte_to_wide_char::MultiByteToWideChar;
pub use open_process::OpenProcess;
pub use open_thread::OpenThread;
pub use process32_first::Process32First;
pub use process32_next::Process32Next;
pub use query_performance_counter::QueryPerformanceCounter;
pub use read_file::ReadFile;
pub use read_process_memory::ReadProcessMemory;
pub use reset_event::ResetEvent;
pub use resume_thread::ResumeThread;
pub use set_current_directory_a::SetCurrentDirectoryA;
pub use set_error_mode::SetErrorMode;
pub use set_last_error::SetLastError;
pub use set_thread_locale::SetThreadLocale;
pub use set_thread_stack_guarantee::SetThreadStackGuarantee;
pub use set_unhandled_exception_filter::SetUnhandledExceptionFilter;
pub use sizeof_resource::SizeofResource;
pub use sleep::Sleep;
pub use system_time_to_file_time::SystemTimeToFileTime;
pub use system_time_to_tz_specific_local_time::SystemTimeToTzSpecificLocalTime;
pub use terminate_process::TerminateProcess;
pub use thread32_first::Thread32First;
pub use thread32_next::Thread32Next;
pub use tls_alloc::TlsAlloc;
pub use tls_free::TlsFree;
pub use tls_get_value::TlsGetValue;
pub use tls_set_value::TlsSetValue;
pub use unhandled_exception_filter::UnhandledExceptionFilter;
pub use virtual_alloc::VirtualAlloc;
pub use virtual_alloc_ex::VirtualAllocEx;
pub use virtual_alloc_ex_numa::VirtualAllocExNuma;
pub use virtual_free::VirtualFree;
pub use virtual_lock::VirtualLock;
pub use virtual_protect::VirtualProtect;
pub use virtual_protect_ex::VirtualProtectEx;
pub use wait_for_single_object::WaitForSingleObject;
pub use wide_char_to_multi_byte::WideCharToMultiByte;
pub use win_exec::WinExec;
pub use write_console_w::WriteConsoleW;
pub use write_file::WriteFile;
pub use write_process_memory::WriteProcessMemory;
pub use local_free::LocalFree;
pub use device_io_control::api_DeviceIoControl;

// a in RCX, b in RDX, c in R8, d in R9, then e pushed on stack

pub fn clear_last_error(emu: &mut emu::Emu) {
    let mut err = LAST_ERROR.lock().unwrap();
    *err = constants::ERROR_SUCCESS;
}

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = guess_api_name(emu, addr);
    match api.as_str() {
        "ActivateActCtx" => ActivateActCtx(emu),
        "AddVectoredExceptionHandler" => AddVectoredExceptionHandler(emu),
        "AreFileApiIsAnsi" => AreFileApiIsAnsi(emu),
        "BeginUpdateResourceA" => BeginUpdateResourceA(emu),
        "CloseHandle" => CloseHandle(emu),
        "CompareStringW" => CompareStringW(emu),
        "ConnectNamedPipe" => ConnectNamedPipe(emu),
        "CopyFileA" => CopyFileA(emu),
        "CopyFileW" => CopyFileW(emu),
        "CreateActCtxA" => CreateActCtxA(emu),
        "CreateEventA" => CreateEventA(emu),
        "CreateFileA" => CreateFileA(emu),
        "CreateFileMappingA" => CreateFileMappingA(emu),
        "CreateFileMappingW" => CreateFileMappingW(emu),
        "CreateMutexA" => CreateMutexA(emu),
        "CreateMutexW" => CreateMutexW(emu),
        "CreateNamedPipeA" => CreateNamedPipeA(emu),
        "CreateNamedPipeW" => CreateNamedPipeW(emu),
        "CreateProcessA" => CreateProcessA(emu),
        "CreateProcessW" => CreateProcessW(emu),
        "CreateRemoteThread" => CreateRemoteThread(emu),
        "CreateThread" => CreateThread(emu),
        "CreateToolhelp32Snapshot" => CreateToolhelp32Snapshot(emu),
        "DecodePointer" => DecodePointer(emu),
        "DisconnectNamedPipe" => DisconnectNamedPipe(emu),
        "EncodePointer" => EncodePointer(emu),
        "EnterCriticalSection" => EnterCriticalSection(emu),
        "ExitProcess" => ExitProcess(emu),
        "ExpandEnvironmentStringsA" => ExpandEnvironmentStringsA(emu),
        "ExpandEnvironmentStringsW" => ExpandEnvironmentStringsW(emu),
        "FileTimeToSystemTime" => FileTimeToSystemTime(emu),
        "FindActCtxSectionStringW" => FindActCtxSectionStringW(emu),
        "FindClose" => FindClose(emu),
        "FindFirstFileA" => FindFirstFileA(emu),
        "FindFirstFileExW" => FindFirstFileExW(emu),
        "FindFirstFileW" => FindFirstFileW(emu),
        "FindNextFileA" => FindNextFileA(emu),
        "FindNextFileW" => FindNextFileW(emu),
        "FindResourceA" => FindResourceA(emu),
        "FindResourceW" => FindResourceW(emu),
        "FlsAlloc" => FlsAlloc(emu),
        "FlsGetValue" => FlsGetValue(emu),
        "FlsSetValue" => FlsSetValue(emu),
        "FreeResource" => FreeResource(emu),
        "GetACP" => GetACP(emu),
        "GetCommandLineA" => GetCommandLineA(emu),
        "GetCommandLineW" => GetCommandLineW(emu),
        "GetComputerNameA" => GetComputerNameA(emu),
        "GetComputerNameW" => GetComputerNameW(emu),
        "GetConsoleCP" => GetConsoleCP(emu),
        "GetConsoleMode" => GetConsoleMode(emu),
        "GetConsoleOutputCP" => GetConsoleOutputCP(emu),
        "GetCPInfo" => GetCPInfo(emu),
        "GetCurrentDirectoryA" => GetCurrentDirectoryA(emu),
        "GetCurrentDirectoryW" => GetCurrentDirectoryW(emu),
        "GetCurrentProcess" => GetCurrentProcess(emu),
        "GetCurrentProcessId" => GetCurrentProcessId(emu),
        "GetCurrentThread" => GetCurrentThread(emu),
        "GetCurrentThreadId" => GetCurrentThreadId(emu),
        "GetEnvironmentStringsW" => GetEnvironmentStringsW(emu),
        "GetEnvironmentVariableW" => GetEnvironmentVariableW(emu),
        "GetFileAttributesA" => GetFileAttributesA(emu),
        "GetFileAttributesW" => GetFileAttributesW(emu),
        "GetFileSize" => GetFileSize(emu),
        "GetFullPathNameA" => GetFullPathNameA(emu),
        "GetFullPathNameW" => GetFullPathNameW(emu),
        "GetLastError" => GetLastError(emu),
        "GetLocaleInfoA" => GetLocaleInfoA(emu),
        "GetLocaleInfoW" => GetLocaleInfoW(emu),
        "GetLocalTime" => GetLocalTime(emu),
        "GetLogicalDrives" => GetLogicalDrives(emu),
        "GetModuleFileNameA" => GetModuleFileNameA(emu),
        "GetModuleFileNameW" => GetModuleFileNameW(emu),
        "GetModuleHandleA" => GetModuleHandleA(emu),
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "GetNativeSystemInfo" => GetNativeSystemInfo(emu),
        "GetProcAddress" => GetProcAddress(emu),
        "GetProcessAffinityMask" => GetProcessAffinityMask(emu),
        "GetProcessHeap" => GetProcessHeap(emu),
        "GetStartupInfoA" => GetStartupInfoA(emu),
        "GetStartupInfoW" => GetStartupInfoW(emu),
        "GetStdHandle" => GetStdHandle(emu),
        "GetThreadId" => GetThreadId(emu),
        "GetSystemDirectoryA" => GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => GetSystemDirectoryW(emu),
        "GetSystemFirmwareTable" => GetSystemFirmwareTable(emu),
        "GetSystemInfo" => GetSystemInfo(emu),
        "GetSystemTime" => GetSystemTime(emu),
        "GetSystemTimeAsFileTime" => GetSystemTimeAsFileTime(emu),
        "GetTempPathW" => GetTempPathW(emu),
        "GetThreadContext" => GetThreadContext(emu),
        "GetThreadLocale" => GetThreadLocale(emu),
        "GetTickCount" => GetTickCount(emu),
        "GetUserDefaultLangId" => GetUserDefaultLangId(emu),
        "GetUserDefaultLCID" => GetUserDefaultLCID(emu),
        "GetVersion" => GetVersion(emu),
        "GetVersionExA" => GetVersionExA(emu),
        "GetVersionExW" => GetVersionExW(emu),
        "GetWindowsDirectoryA" => GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => GetWindowsDirectoryW(emu),
        "GlobalAddAtomA" => GlobalAddAtomA(emu),
        "HeapAlloc" => HeapAlloc(emu),
        "HeapCreate" => HeapCreate(emu),
        "HeapFree" => HeapFree(emu),
        "HeapReAlloc" => HeapReAlloc(emu),
        "InitializeCriticalSection" => InitializeCriticalSection(emu),
        "InitializeCriticalSectionAndSpinCount" => InitializeCriticalSectionAndSpinCount(emu),
        "InitializeCriticalSectionEx" => InitializeCriticalSectionEx(emu),
        "InitOnceBeginInitialize" => InitOnceBeginInitialize(emu),
        "IsBadReadPtr" => IsBadReadPtr(emu),
        "IsDebuggerPresent" => IsDebuggerPresent(emu),
        "IsProcessorFeaturePresent" => IsProcessorFeaturePresent(emu),
        "LeaveCriticalSection" => LeaveCriticalSection(emu),
        "LoadLibraryA" => LoadLibraryA(emu),
        "LoadLibraryExA" => LoadLibraryExA(emu),
        "LoadLibraryExW" => LoadLibraryExW(emu),
        "LoadLibraryW" => LoadLibraryW(emu),
        "LoadResource" => LoadResource(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "LocalFree" => LocalFree(emu),
        "LockResource" => LockResource(emu),
        "lstrcatA" => lstrcatA(emu),
        "lstrcatW" => lstrcatW(emu),
        "LStrCmpI" => LStrCmpI(emu),
        "lstrcmpiW" => LStrCmpIW(emu),
        "LStrCmpIW" => LStrCmpIW(emu),
        "lstrcpy" => lstrcpy(emu),
        "lstrcpyn" => lstrcpyn(emu),
        "lstrcpyW" => lstrcpyW(emu),
        "lstrlenA" => lstrlenA(emu),
        "lstrlenW" => lstrlenW(emu),
        "MapViewOfFile" => MapViewOfFile(emu),
        "MoveFileA" => MoveFileA(emu),
        "MoveFileW" => MoveFileW(emu),
        "MultiByteToWideChar" => MultiByteToWideChar(emu),
        "OpenProcess" => OpenProcess(emu),
        "OpenThread" => OpenThread(emu),
        "Process32First" => Process32First(emu),
        "Process32Next" => Process32Next(emu),
        "QueryPerformanceCounter" => QueryPerformanceCounter(emu),
        "ReadFile" => ReadFile(emu),
        "ReadProcessMemory" => ReadProcessMemory(emu),
        "ResetEvent" => ResetEvent(emu),
        "ResumeThread" => ResumeThread(emu),
        "SetCurrentDirectoryA" => SetCurrentDirectoryA(emu),
        "SetErrorMode" => SetErrorMode(emu),
        "SetLastError" => SetLastError(emu),
        "SetThreadLocale" => SetThreadLocale(emu),
        "SetThreadStackGuarantee" => SetThreadStackGuarantee(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "SizeofResource" => SizeofResource(emu),
        "Sleep" => Sleep(emu),
        "SystemTimeToFileTime" => SystemTimeToFileTime(emu),
        "SystemTimeToTzSpecificLocalTime" => SystemTimeToTzSpecificLocalTime(emu),
        "TerminateProcess" => TerminateProcess(emu),
        "Thread32First" => Thread32First(emu),
        "Thread32Next" => Thread32Next(emu),
        "TlsAlloc" => TlsAlloc(emu),
        "TlsFree" => TlsFree(emu),
        "TlsGetValue" => TlsGetValue(emu),
        "TlsSetValue" => TlsSetValue(emu),
        "UnhandledExceptionFilter" => UnhandledExceptionFilter(emu),
        "VirtualAlloc" => VirtualAlloc(emu),
        "VirtualAllocEx" => VirtualAllocEx(emu),
        "VirtualAllocExNuma" => VirtualAllocExNuma(emu),
        "VirtualFree" => VirtualFree(emu),
        "VirtualLock" => VirtualLock(emu),
        "VirtualProtect" => VirtualProtect(emu),
        "VirtualProtectEx" => VirtualProtectEx(emu),
        "WaitForSingleObject" => WaitForSingleObject(emu),
        "WideCharToMultiByte" => WideCharToMultiByte(emu),
        "WinExec" => WinExec(emu),
        "WriteConsoleW" => WriteConsoleW(emu),
        "WriteFile" => WriteFile(emu),
        "WriteProcessMemory" => WriteProcessMemory(emu),
        "DeviceIoControl" => api_DeviceIoControl(emu),
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


fn GetThreadId(emu: &mut Emu) {
    let hndl = emu.regs().rcx;

    for i in 0..emu.threads.len() {
        if emu.threads[i].handle == hndl {
            emu.regs_mut().rax = emu.threads[i].id;
            log_red!(emu, "kernel32!GetThreadId hndl:{} (requested handle exists and its tid {})", hndl, emu.threads[i].id);
            return;
        }
    }
    log_red!(emu, "kernel32!GetThreadId hndl:{} (requested handle doesn't exist, returning a fake handle for now but should return zero.)", hndl);
    emu.regs_mut().rax = 0x2c2878; // if handle not found should return zero.
}




lazy_static! {
    pub static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    pub static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    pub static ref LAST_ERROR: Mutex<u64> = Mutex::new(0);
}

pub fn set_last_error(err_code: u64) {
    let mut guard = LAST_ERROR.lock().unwrap();
    *guard = err_code;
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
