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
}

fn __p___argv(emu: &mut emu::Emu) {
    let argv = emu.regs.rcx;

    log::info!(
        "{}** {} crt_runtime!__p___argv argv: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        argv,
        emu.colors.nc
    );

    // TODO: Implement this
}

fn __p___argc(emu: &mut emu::Emu) {
    let argc = emu.regs.rcx;

    log::info!(
        "{}** {} crt_runtime!__p___argc argc: 0x{:x}  {}",
        emu.colors.light_red,
        emu.pos,
        argc,
        emu.colors.nc
    );

    // TODO: Implement this
}
