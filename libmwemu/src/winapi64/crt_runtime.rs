use crate::emu;
use crate::winapi64;
use crate::serialization;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "_initialize_onexit_table" => _initialize_onexit_table(emu),
        "_register_onexit_function" => _register_onexit_function(emu),
        "_get_initial_narrow_environment" => _get_initial_narrow_environment(emu),
        "__p___argv" => __p___argv(emu),
        "__p___argc" => __p___argc(emu),
        "__acrt_iob_func" => __acrt_iob_func(emu),
        "__stdio_common_vfprintf" => __stdio_common_vfprintf(emu),
        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(&emu, emu.cfg.dump_filename.as_ref().unwrap());
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!("calling unimplemented API 0x{:x} {} at 0x{:x}", addr, api, emu.regs.rip);
            return api;
        }
    }

    String::new()
}

fn _initialize_onexit_table(emu: &mut emu::Emu) {
    let table = emu.regs.rcx;

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

    log::info!(
        "{}** {} crt_runtime!_initialize_onexit_table  {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    emu.regs.rax = 0;
}

fn _register_onexit_function(emu: &mut emu::Emu) {
    let table = emu.regs.rcx;
    let callback = emu.regs.rdx;

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

    log::info!(
        "{}** {} crt_runtime!_initialize_onexit_function callback: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        callback,
        emu.colors.nc
    );

    emu.regs.rax = 0;
}

/*
extern "C" char** __cdecl _get_initial_narrow_environment()
{
    return common_get_initial_environment<char>();
}
*/
fn _get_initial_narrow_environment(emu: &mut emu::Emu) {
    let env = emu.regs.rcx;

    log::info!(
        "{}** {} crt_runtime!_get_initial_narrow_environment env: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        env,
        emu.colors.nc
    );

    // TODO: Implement this
    emu.regs.rax = 0;
}

// char*** CDECL __p___argv(void) { return &MSVCRT___argv; }
fn __p___argv(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} crt_runtime!__p___argv {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );

    // First, allocate space for argv array (pointer array)
    // We'll allocate space for 2 pointers - one for program name and null terminator
    let argv_array_addr = emu
        .maps
        .alloc(16) // 2 * sizeof(pointer) on x64
        .expect("crt_runtime!__p___argv cannot allocate argv array");
    emu.maps.create_map(&format!("alloc_{:x}", argv_array_addr), argv_array_addr, 16);

    // Allocate space for program name string (using a dummy name)
    let prog_name = "program.exe\0";
    let prog_name_addr = emu
        .maps
        .alloc(prog_name.len() as u64)
        .expect("crt_runtime!__p___argv cannot allocate program name");
    emu.maps.create_map(&format!("alloc_{:x}", prog_name_addr), prog_name_addr, 16);

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
        .expect("crt_runtime!__p___argv cannot allocate p_argv");
    emu.maps.create_map(&format!("alloc_{:x}", p_argv_addr), p_argv_addr, 8);

    // Write pointer to argv array
    emu.maps.write_qword(p_argv_addr, argv_array_addr);

    // Return pointer to argv
    emu.regs.rax = p_argv_addr;
}

// int* CDECL __p___argc(void) { return &MSVCRT___argc; }
fn __p___argc(emu: &mut emu::Emu) {
    let argc = emu.regs.rcx;

    log::info!(
        "{}** {} crt_runtime!__p___argc argc: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        argc,
        emu.colors.nc
    );

    let argc_addr = emu
        .maps
        .alloc(4)
        .expect("crt_runtime!__p___argc cannot allocate");
    emu.maps.create_map(&format!("alloc_{:x}", argc_addr), argc_addr, 4);
    emu.maps.write_dword(argc_addr, 1);
    emu.regs.rax = argc_addr;
}

/*
FILE * CDECL __acrt_iob_func(int index)
{
    return &__iob_func()[index];
}
*/

fn __acrt_iob_func(emu: &mut emu::Emu) {
    let index = emu.regs.rcx;

    log::info!(
        "{}** {} crt_runtime!__acrt_iob_func index: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        index,
        emu.colors.nc
    );

    // TODO: Implement this
}

/*
_ACRTIMP int __cdecl __stdio_common_vfprintf(unsigned __int64,FILE*,const char*,_locale_t,__ms_va_list);
*/
fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let index = emu.regs.rcx;

    log::info!(
        "{}** {} crt_runtime!__stdio_common_vfprintf index: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        index,
        emu.colors.nc
    );

    // TODO: Implement this
}
