use crate::emu;
use crate::serialization;
use crate::winapi64::kernel32;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    
    match api.as_str() {
        "__set_app_type" => __set_app_type(emu),
        "malloc" => malloc(emu),
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

/*
void __set_app_type (
   int at
)
*/
fn __set_app_type(emu: &mut emu::Emu) {
    let app_type = emu.regs.rcx;
    log_red!(emu, "** {} msvcrt!__set_app_type  app_type: 0x{:x}", emu.pos, app_type);
}

fn malloc(emu: &mut emu::Emu) {
    let size = emu.regs.rcx;

    if size > 0 {
        let base = emu.maps.alloc(size).expect("msvcrt!malloc out of memory");

        emu.maps
            .create_map(&format!("alloc_{:x}", base), base, size)
            .expect("msvcrt!malloc cannot create map");

        log::info!(
            "{}** {} msvcrt!malloc sz: {} addr: 0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            size,
            base,
            emu.colors.nc
        );

        emu.regs.rax = base;
    } else {
        emu.regs.rax = 0x1337; // weird msvcrt has to return a random unallocated pointer, and the program has to do free() on it
    }
}