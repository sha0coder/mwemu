use crate::emu;
use crate::winapi::winapi32;

use super::{HintNameItem, PE32};

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        u32::from_le_bytes([
            $raw[$off],
            $raw[$off + 1],
            $raw[$off + 2],
            $raw[$off + 3],
        ])
    };
}

macro_rules! write_u32_le {
    ($raw:expr, $off:expr, $val:expr) => {
        $raw[$off + 0] = ($val & 0x000000ff) as u8;
        $raw[$off + 1] = (($val & 0x0000ff00) >> 8) as u8;
        $raw[$off + 2] = (($val & 0x00ff0000) >> 16) as u8;
        $raw[$off + 3] = (($val & 0xff000000) >> 24) as u8;
    };
}

impl PE32 {
    pub fn pe32_delay_load_binding(&mut self, emu: &mut emu::Emu, base_addr: u32) {
        log::trace!("Delay load binding started for {} ...", self.filename);
        for i in 0..self.delay_load_dir.len() {
            let dld = &self.delay_load_dir[i];
            if dld.name.is_empty() {
                continue;
            }

            if winapi32::kernel32::load_library(emu, &dld.name) == 0 {
                panic!(
                    "cannot found the library `{}` on {}",
                    &dld.name, emu.cfg.maps_folder
                );
            }

            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, dld.name_table) as usize;
            let mut off_addr =
                PE32::vaddr_to_off(&self.sect_hdr, dld.bound_delay_import_table) as usize;
            let mut rva = dld.bound_delay_import_table;

            loop {
                if self.raw.len() <= off_name + 4 || self.raw.len() <= off_addr + 4 {
                    break;
                }

                let hint = HintNameItem::load(&self.raw, off_name);
                let _addr = read_u32_le!(self.raw, off_addr);
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, hint.func_name_addr) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    off_addr += 4;
                    rva += 4;
                    continue;
                }
                let func_name = PE32::read_string(&self.raw, off2 + 2);
                let real_addr = winapi32::kernel32::resolve_api_name(emu, &func_name);
                if real_addr == 0 {
                    break;
                }

                write_u32_le!(self.raw, off_addr, real_addr as u32);
                let patch_addr = base_addr as u64 + rva as u64;
                if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                    mem.force_write_dword(patch_addr, real_addr as u32);
                }

                off_name += HintNameItem::size();
                off_addr += 4;
                rva += 4;
            }
        }
        log::trace!("delay load bound!");
    }

    pub fn pe32_iat_binding(&mut self, emu: &mut emu::Emu, base_addr: u32) {
        let dbg = false;

        log::trace!(
            "IAT binding started image_import_descriptor.len() = {} ...",
            self.image_import_descriptor.len()
        );

        for i in 0..self.image_import_descriptor.len() {
            let iim_name = self.image_import_descriptor[i].name.clone();
            let original_first_thunk = self.image_import_descriptor[i].original_first_thunk;
            let first_thunk = self.image_import_descriptor[i].first_thunk;

            if iim_name.is_empty() {
                continue;
            }

            if winapi32::kernel32::load_library(emu, &iim_name) == 0 {
                log::trace!("cannot find the library `{}` in maps/windows/x86/", &iim_name);
                continue;
            } else if dbg {
                log::trace!("library `{}` loaded", &iim_name);
            }

            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, original_first_thunk) as usize;
            let mut off_addr = PE32::vaddr_to_off(&self.sect_hdr, first_thunk) as usize;
            let mut rva = first_thunk;

            loop {
                if self.raw.len() <= off_name + 4 || self.raw.len() <= off_addr + 4 {
                    break;
                }
                let hint = HintNameItem::load(&self.raw, off_name);
                let addr = read_u32_le!(self.raw, off_addr);
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, hint.func_name_addr) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    off_addr += 4;
                    rva += 4;
                    continue;
                }
                let func_name = PE32::read_string(&self.raw, off2 + 2);
                let real_addr =
                    winapi32::kernel32::resolve_api_name_in_module(emu, &iim_name, &func_name);
                if dbg {
                    let real_addr1 = winapi32::kernel32::resolve_api_name(emu, &func_name);
                    if real_addr1 != real_addr {
                        log::trace!("--------------------------");
                        let (va, modm, func) = winapi32::kernel32::search_api_name(emu, &func_name);
                        log::trace!(
                            "inport: {}!{}  ldr: {}!{}",
                            &iim_name,
                            &func_name,
                            modm,
                            func
                        );
                        log::trace!(
                            "0x{:x} {}!{}  0x{:x}-> 0x{:x}",
                            addr,
                            iim_name,
                            func_name,
                            off_addr,
                            real_addr
                        );
                        log::trace!(
                            "*********** prev:0x{:x} == new:0x{:x}",
                            real_addr1,
                            real_addr
                        );
                        println!("0x{:x} {} {}", va, modm, func);
                        log::trace!("--------------------------");
                    }
                }
                if real_addr == 0 {
                    break;
                }
                write_u32_le!(self.raw, off_addr, real_addr as u32);
                let patch_addr = base_addr as u64 + rva as u64;
                if emu.maps.is_mapped(patch_addr) {
                    emu.maps.write_dword(patch_addr, real_addr as u32);
                }

                off_name += HintNameItem::size();
                off_addr += 4;
                rva += 4;
            }
        }
        log::trace!("{} IAT Bound.", &self.filename);
    }

    pub fn pe32_import_addr_to_name(&self, paddr: u32) -> String {
        let dbg = false;
        if paddr == 0 {
            return String::new();
        }

        for i in 0..self.image_import_descriptor.len() {
            let iim = &self.image_import_descriptor[i];
            if dbg {
                log::trace!("import: {}", iim.name);
            }

            if iim.name.is_empty() {
                continue;
            }

            let mut off_name = PE32::vaddr_to_off(&self.sect_hdr, iim.original_first_thunk) as usize;
            let mut off_addr = PE32::vaddr_to_off(&self.sect_hdr, iim.first_thunk) as usize;

            loop {
                if self.raw.len() <= off_name + 4 || self.raw.len() <= off_addr + 4 {
                    break;
                }
                let hint = HintNameItem::load(&self.raw, off_name);
                let addr = read_u32_le!(self.raw, off_addr);
                let off2 = PE32::vaddr_to_off(&self.sect_hdr, hint.func_name_addr) as usize;
                if off2 == 0 {
                    off_name += HintNameItem::size();
                    off_addr += 4;
                    continue;
                }

                if addr == paddr {
                    let func_name = PE32::read_string(&self.raw, off2 + 2);
                    return func_name;
                }

                off_name += HintNameItem::size();
                off_addr += 4;
            }
        }
        String::new()
    }
}
