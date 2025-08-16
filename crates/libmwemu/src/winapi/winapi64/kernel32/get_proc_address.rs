use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::emu;

pub fn GetProcAddress(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let func_ptr = emu.regs().rdx;

    let func = emu.maps.read_string(func_ptr).to_lowercase();
    if func == "zwcopyfilechunk" {
        emu.regs_mut().rax = 0x7ff7e0001337;
        log::info!(
            "{}** {} kernel32!GetProcAddress  `{}!{}` =0x{:x} {}",
            emu.colors.light_red,
            emu.pos,
            "kernel32",
            "zwcopyfilechunk",
            emu.regs().rax,
            emu.colors.nc
        );
        return;
    }

    let mut flink = peb64::Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    loop {
        if flink.export_table_rva > 0 {
            for i in 0..flink.num_of_funcs {
                if flink.pe_hdr == 0 {
                    continue;
                }
                let ordinal = flink.get_function_ordinal(emu, i);

                // log::info!("func name {}!{}", flink.mod_name, ordinal.func_name);

                if ordinal.func_name.to_lowercase() == func {
                    emu.regs_mut().rax = ordinal.func_va;
                    log::info!(
                        "{}** {} kernel32!GetProcAddress  `{}!{}` =0x{:x} {}",
                        emu.colors.light_red,
                        emu.pos,
                        flink.mod_name,
                        ordinal.func_name,
                        emu.regs().rax,
                        emu.colors.nc
                    );
                    return;
                }
            }
        }

        flink.next(emu);
        if flink.get_ptr() == first_flink {
            break;
        }
    }
    emu.regs_mut().rax = 0;
    log::warn!("kernel32!GetProcAddress error searching {}", func);
}