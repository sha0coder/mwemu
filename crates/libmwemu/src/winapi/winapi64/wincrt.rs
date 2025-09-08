use crate::emu;
use crate::maps::mem64::Permission;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "_initialize_onexit_table" => _initialize_onexit_table(emu),
        "_register_onexit_function" => _register_onexit_function(emu),
        "_get_initial_narrow_environment" => _get_initial_narrow_environment(emu),
        "_set_invalid_parameter_handler" => set_invalid_parameter_handler(emu),
        "malloc" => malloc(emu),
        "realloc" => realloc(emu),
        "_crt_atexit" => _crt_atexit(emu),
        "__p___argv" => __p___argv(emu),
        "__p___argc" => __p___argc(emu),
        "__acrt_iob_func" => __acrt_iob_func(emu),
        "__stdio_common_vfprintf" => __stdio_common_vfprintf(emu),
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

fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;

    /*
    http://sandbox.hlt.bme.hu/~gaebor/STLdoc/VS2017/corecrt__startup_8h_source.html
    133 typedef struct _onexit_table_t
    134 {
    135     _PVFV* _first;
    136     _PVFV* _last;
    137     _PVFV* _end;
    138 } _onexit_table_t;
    139
     */

    log_red!(emu, "wincrt!_initialize_onexit_table");

    emu.regs_mut().rax = 0;
}

fn _register_onexit_function(emu: &mut emu::Emu) {
    let table = emu.regs().rcx;
    let callback = emu.regs().rdx;

    /*
    http://sandbox.hlt.bme.hu/~gaebor/STLdoc/VS2017/corecrt__startup_8h_source.html
    133 typedef struct _onexit_table_t
    134 {
    135     _PVFV* _first;
    136     _PVFV* _last;
    137     _PVFV* _end;
    138 } _onexit_table_t;
    139
     */

    log_red!(
        emu,
        "wincrt!_initialize_onexit_function callback: 0x{:x}",
        callback
    );

    emu.regs_mut().rax = 0;
}

/*
extern "C" char** __cdecl _get_initial_narrow_environment()
{
    return common_get_initial_environment<char>();
}
*/
fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let env = emu.regs().rcx;

    log_red!(
        emu,
        "wincrt!_get_initial_narrow_environment env: 0x{:x}",
        env
    );

    // TODO: Implement this
    emu.regs_mut().rax = 0;
}

// char*** CDECL __p___argv(void) { return &MSVCRT___argv; }
fn __p___argv(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!__p___argv");

    // First, allocate space for argv array (pointer array)
    // We'll allocate space for 2 pointers - one for program name and null terminator
    let argv_array_addr = emu
        .maps
        .alloc(16) // 2 * sizeof(pointer) on x64
        .expect("wincrt!__p___argv cannot allocate argv array");
    emu.maps.create_map(
        &format!("alloc_{:x}", argv_array_addr),
        argv_array_addr,
        16,
        Permission::READ_WRITE,
    );

    // Allocate space for program name string (using a dummy name)
    let prog_name = "program.exe\0";
    let prog_name_addr = emu
        .maps
        .alloc(prog_name.len() as u64)
        .expect("wincrt!__p___argv cannot allocate program name");
    emu.maps.create_map(
        &format!("alloc_{:x}", prog_name_addr),
        prog_name_addr,
        16,
        Permission::READ_WRITE,
    );

    // Write program name string
    emu.maps.write_string(prog_name_addr, prog_name);

    // Write argv array:
    // argv[0] = pointer to program name
    emu.maps.write_qword(argv_array_addr, prog_name_addr);
    // argv[1] = null terminator
    emu.maps.write_qword(argv_array_addr + 8, 0);

    // Allocate space for pointer to argv array
    let p_argv_addr = emu
        .maps
        .alloc(8) // sizeof(pointer) on x64
        .expect("wincrt!__p___argv cannot allocate p_argv");
    emu.maps.create_map(
        &format!("alloc_{:x}", p_argv_addr),
        p_argv_addr,
        8,
        Permission::READ_WRITE,
    );

    // Write pointer to argv array
    emu.maps.write_qword(p_argv_addr, argv_array_addr);

    // Return pointer to argv
    emu.regs_mut().rax = p_argv_addr;
}

// int* CDECL __p___argc(void) { return &MSVCRT___argc; }
fn __p___argc(emu: &mut emu::Emu) {
    let argc = emu.regs().rcx;

    log_red!(emu, "wincrt!__p___argc argc: 0x{:x}", argc);

    let argc_addr = emu
        .maps
        .alloc(4)
        .expect("wincrt!__p___argc cannot allocate");
    emu.maps.create_map(
        &format!("alloc_{:x}", argc_addr),
        argc_addr,
        4,
        Permission::READ_WRITE,
    );
    emu.maps.write_dword(argc_addr, 1);
    emu.regs_mut().rax = argc_addr;
}

/*
FILE * CDECL __acrt_iob_func(int index)
{
    return &__iob_func()[index];
}
*/

fn __acrt_iob_func(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;

    log_red!(emu, "wincrt!__acrt_iob_func index: 0x{:x}", index);

    // TODO: Implement this
    emu.regs_mut().rax = 0;
}

/*
_ACRTIMP int __cdecl __stdio_common_vfprintf(unsigned __int64,FILE*,const char*,_locale_t,__ms_va_list);
*/
fn parse_format_specifiers(fmt: &str) -> Vec<&str> {
    let mut specs = Vec::new();
    let mut chars = fmt.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(next) = chars.next() {
                if next != '%' {
                    // Skip %% (literal %)
                    specs.push(match next {
                        'd' | 'i' => "int",
                        'x' | 'X' => "hex",
                        'p' => "ptr",
                        's' => "str",
                        // Add other format specifiers as needed
                        _ => "unknown",
                    });
                }
            }
        }
    }
    specs
}

fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let options = emu.regs().rcx; // _In_ options
    let file = emu.regs().rdx; // _In_ FILE*
    let format = emu.regs().r8; // _In_ format string ptr
    let locale = emu.regs().r9; // _In_opt_ locale
    let va_list = emu
        .maps
        .read_qword(emu.regs().rsp + 0x20)
        .expect("wincrt!__stdio_common_vfprintf cannot read_qword va_list");

    // Just try to read the format string
    let fmt_str = emu.maps.read_string(format);
    let specs = parse_format_specifiers(&fmt_str);

    log_red!(
        emu,
        "wincrt!__stdio_common_vfprintf options: 0x{:x} file: 0x{:x} format: '{}' locale: 0x{:x} va_list: 0x{:x}",
        options,
        file,
        fmt_str,
        locale,
        va_list
    );

    let mut current_ptr = va_list;
    for spec in specs {
        match spec {
            "int" | "hex" | "ptr" => {
                let arg = emu
                    .maps
                    .read_qword(current_ptr)
                    .expect("wincrt!__stdio_common_vfprintf cannot read_qword arg");
                current_ptr += 8; // Move to next arg
                log::info!("arg: {:016x}", arg);
            }
            "str" => {
                let str_ptr = emu
                    .maps
                    .read_qword(current_ptr)
                    .expect("wincrt!__stdio_common_vfprintf cannot read_qword str_ptr");
                let string = emu.maps.read_string(str_ptr);
                current_ptr += 8;
                log::info!("string: {}", string);
            }
            _ => {
                unimplemented!(
                    "wincrt!__stdio_common_vfprintf unknown format character: {}",
                    spec
                );
            }
        }
    }

    // Return success (1) - this is super basic
    emu.regs_mut().rax = 1;
}

fn realloc(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;

    if addr == 0 {
        if size == 0 {
            emu.maps.dealloc(addr);
            emu.regs_mut().rax = 0;
            return;
        } else {
            let base = emu.maps.alloc(size).expect("msvcrt!malloc out of memory");

            // normally malloc region is permission read write
            emu.maps
                .create_map(
                    &format!("alloc_{:x}", base),
                    base,
                    size,
                    Permission::READ_WRITE,
                )
                .expect("msvcrt!malloc cannot create map");

            log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x{:x}", addr, size, base);

            emu.regs_mut().rax = base;
            return;
        }
    }

    if size == 0 {
        log_red!(emu, "msvcrt!realloc 0x{:x} {} =0x1337", addr, size);

        emu.regs_mut().rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
        return;
    }

    let new_addr = emu.maps.alloc(size).expect("msvcrt!realloc out of memory");
    let mem = emu
        .maps
        .get_mem_by_addr_mut(addr)
        .expect("msvcrt!realloc error getting mem");
    let old_permission = mem.permission();
    let prev_size = mem.size();

    emu.maps
        .create_map(
            &format!("alloc_{:x}", new_addr),
            new_addr,
            size,
            old_permission,
        )
        .expect("msvcrt!realloc cannot create map");

    emu.maps.memcpy(new_addr, addr, prev_size);
    emu.maps.dealloc(addr);

    log_red!(
        emu,
        "msvcrt!realloc 0x{:x} {} =0x{:x}",
        addr,
        size,
        new_addr
    );

    emu.regs_mut().rax = new_addr;
}

fn set_invalid_parameter_handler(emu: &mut emu::Emu) {
    log_red!(emu, "wincrt!_set_invalid_parameter_handler");
    emu.regs_mut().rax = 0;
}

fn malloc(emu: &mut emu::Emu) {
    let size = emu.regs().rcx; // In malloc, size is the only parameter

    if size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let base = emu.maps.alloc(size).expect("msvcrt!malloc out of memory");

    emu.maps
        .create_map(
            &format!("alloc_{:x}", base),
            base,
            size,
            Permission::READ_WRITE,
        )
        .expect("msvcrt!malloc cannot create map");

    log_red!(emu, "msvcrt!malloc {} =0x{:x}", size, base);

    emu.regs_mut().rax = base;
}

/*
int _crt_atexit(
    _PVFV const function
)
*/
fn _crt_atexit(emu: &mut emu::Emu) {
    let function = emu.regs().rcx;

    log_red!(emu, "wincrt!_crt_atexit function: 0x{:x}", function);
    // TODO: Implement this
    emu.regs_mut().rax = 0;
}
