
use crate::emu;
use crate::winapi::helper;

pub fn GetModuleHandleA(emu: &mut emu::Emu) {
    let module_name_ptr = emu.regs().rcx;
    let mut module_name: String;

    if module_name_ptr == 0 {
        module_name = "self".to_string();
        emu.regs_mut().rax = match emu.maps.get_base() {
            Some(base) => base,
            None => helper::handler_create(&module_name),
        }
    } else {
        module_name = emu.maps.read_string(module_name_ptr).to_lowercase();
        if module_name.ends_with(".dll") {
            module_name = module_name.replace(".dll",".pe");
        }

        let mod_mem = match emu.maps.get_mem2(&module_name) {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };

        emu.regs_mut().rax = mod_mem.get_base();
    }

    log::info!(
        "{}** {} kernel32!GetModuleHandleA `{}`  {}",
        emu.colors.light_red,
        emu.pos,
        module_name,
        emu.colors.nc
    );
}