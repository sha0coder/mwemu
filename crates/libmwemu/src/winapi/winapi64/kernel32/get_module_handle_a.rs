
use crate::emu;
use crate::winapi::helper;

pub fn GetModuleHandleA(emu: &mut emu::Emu) {
    let module_name_ptr = emu.regs().rcx;
    let module_name: String;

    if module_name_ptr == 0 {
        module_name = "self".to_string();

        let base = match emu.maps.get_base() {
            Some(base) => base,
            None => helper::handler_create(&module_name),
        };

        log_red!(
            emu,
            "kernel32!GetModuleHandleA `{}` {:x}",
            module_name,
            base
        );
        
        emu.regs_mut().rax = base;
    } else {
        module_name = emu.maps.read_string(module_name_ptr);

        let mod_mem = match emu.maps.get_mem2(&module_name.to_lowercase().replace(".dll",".pe")) {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };
        let base = mod_mem.get_base();

        log_red!(
            emu,
            "kernel32!GetModuleHandleA `{}` {:x}",
            module_name,
            base
        );

        emu.regs_mut().rax = base;
    }

    
}
