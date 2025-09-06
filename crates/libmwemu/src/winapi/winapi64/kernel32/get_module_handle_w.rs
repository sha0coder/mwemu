use crate::constants;
use crate::emu;
use crate::peb;
use crate::winapi::helper;

pub fn GetModuleHandleW(emu: &mut emu::Emu) {
    let module_name_ptr = emu.regs().rcx;
    let module_name: String;
    let base;

    if module_name_ptr == 0 {
        module_name = constants::EXE_NAME.to_string();
        base = match peb::peb64::get_module_base(&module_name, emu) {
            Some(b) => b,
            None => helper::handler_create(&module_name),
        };
    } else {
        module_name = emu.maps.read_wide_string(module_name_ptr).to_lowercase();
        let mod_mem = match emu
            .maps
            .get_mem2(&module_name.to_lowercase().replace(".dll", ".pe"))
        {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };

        base = mod_mem.get_base();
    }

    log_red!(
        emu,
        "kernel32!GetModuleHandleW `{}` 0x{:x}",
        module_name,
        base
    );

    emu.regs_mut().rax = base;
}
