use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::{constants, emu};

pub fn GetModuleFileNameW(emu: &mut emu::Emu) {
    let module_handle = emu.regs().rcx as usize;
    let lp_filename = emu.regs().rdx as usize;
    let n_size = emu.regs().r8 as usize;
    log_red!(emu, "** {} kernel32!GetModuleFileNameW module: 0x{:x} lp_filename: 0x{:x} n_size: {}",
        emu.pos,
        module_handle,
        lp_filename,
        n_size
    );

    // TODO: what to do if module is null?
    // TODO: which module name based on handle?

    // watch out for no size?
    if n_size == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    // truncate if needed?
    let output = constants::MODULE_NAME;
    let output_len = output.len();
    if output_len >= n_size {
        // Buffer too small - truncate and return n_size
        let truncated = &output[..n_size - 1];
        emu.maps.write_wide_string(lp_filename as u64, truncated);
        emu.regs_mut().rax = n_size as u64;
    } else {
        // Buffer is large enough
        emu.maps.write_wide_string(lp_filename as u64, output);
        emu.regs_mut().rax = output_len as u64;
    }
}