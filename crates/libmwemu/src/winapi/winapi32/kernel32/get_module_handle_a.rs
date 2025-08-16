use crate::emu;
use crate::winapi::helper;

pub fn GetModuleHandleA(emu: &mut emu::Emu) {
    let mod_name_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetModuleHandleA cannot read mod_name_ptr") as u64;

    let mod_name: String;

    if mod_name_ptr == 0 {
        mod_name = "self".to_string();
        emu.regs_mut().rax = match emu.maps.get_base() {
            Some(base) => base,
            None => helper::handler_create(&mod_name),
        }
    } else {
        mod_name = emu.maps.read_string(mod_name_ptr).to_lowercase();
        let mod_mem = match emu.maps.get_mem2(mod_name.as_str()) {
            Some(m) => m,
            None => {
                emu.regs_mut().rax = 0;
                return;
            }
        };

        emu.regs_mut().rax = mod_mem.get_base();
    }

    log::info!(
        "{}** {} kernel32!GetModuleHandleA '{}' {}",
        emu.colors.light_red,
        emu.pos,
        mod_name,
        emu.colors.nc
    );

    emu.stack_pop32(false);
}