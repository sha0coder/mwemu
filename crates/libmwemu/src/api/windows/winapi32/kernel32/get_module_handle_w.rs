use crate::emu;
use crate::winapi::helper;
use crate::windows::peb;

pub fn GetModuleHandleW(emu: &mut emu::Emu) {
    let mod_name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let mod_name: String;
    let base;

    if mod_name_ptr == 0 {
        mod_name = emu.cfg.exe_name.clone();
        base = match peb::peb32::get_module_base(&mod_name, emu) {
            Some(b) => b,
            None => helper::handler_create(&mod_name),
        };
    } else {
        mod_name = emu.maps.read_wide_string(mod_name_ptr).to_lowercase();
        base = if let Some(mod_mem) = emu.maps.get_mem2(&mod_name) {
            mod_mem.get_base()
        } else if let Some(b) = peb::peb32::get_module_base(&mod_name, emu) {
            b
        } else {
            log_red!(emu, "kernel32!GetModuleHandleW '{}' not found", mod_name);
            emu.regs_mut().rax = 0;
            emu.stack_pop32(false);
            return;
        };
    }

    log_red!(emu, "kernel32!GetModuleHandleW '{}' 0x{:x}", mod_name, base);

    emu.regs_mut().rax = base;

    emu.stack_pop32(false);
}
