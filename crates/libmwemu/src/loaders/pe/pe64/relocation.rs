use crate::emu;

use super::PE64;
use crate::loaders::pe::readers::{
    read_u16_le as read_u16_le_shared, read_u32_le as read_u32_le_shared,
    read_u64_le as read_u64_le_shared, write_u64_le as write_u64_le_shared,
};

macro_rules! read_u16_le {
    ($raw:expr, $off:expr) => {
        read_u16_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! read_u32_le {
    ($raw:expr, $off:expr) => {
        read_u32_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! read_u64_le {
    ($raw:expr, $off:expr) => {
        read_u64_le_shared(($raw).as_ref(), $off)
    };
}

macro_rules! write_u64_le {
    ($raw:expr, $off:expr, $val:expr) => {
        write_u64_le_shared(($raw).as_mut(), $off, $val)
    };
}

impl PE64 {
    pub(crate) fn pe64_apply_relocations(&mut self, emu: &mut emu::Emu, base_addr: u64) {
        if self.opt.data_directory.len() <= crate::loaders::pe::pe32::IMAGE_DIRECTORY_ENTRY_BASERELOC
        {
            return;
        }
        let reloc_dir =
            &self.opt.data_directory[crate::loaders::pe::pe32::IMAGE_DIRECTORY_ENTRY_BASERELOC];
        let reloc_va = reloc_dir.virtual_address;
        let reloc_sz = reloc_dir.size;

        if reloc_va == 0 || reloc_sz == 0 {
            return;
        }

        let delta = base_addr.wrapping_sub(self.opt.image_base);
        if delta == 0 {
            return;
        }

        let mut off = PE64::vaddr_to_off(&self.sect_hdr, reloc_va) as usize;
        if off == 0 {
            return;
        }

        let end_off = off + reloc_sz as usize;

        if emu.cfg.verbose >= 1 {
            log::info!("Applying base relocations...");
        }

        while off < end_off && off + 8 <= self.raw.len() {
            let page_va = read_u32_le!(self.raw, off);
            let block_sz = read_u32_le!(self.raw, off + 4);

            if page_va == 0 && block_sz == 0 {
                break;
            }
            if block_sz < 8 {
                break;
            }

            let entries_count = (block_sz - 8) / 2;
            let mut entry_off = off + 8;

            for _ in 0..entries_count {
                if entry_off + 2 > self.raw.len() {
                    break;
                }
                let entry = read_u16_le!(self.raw, entry_off);
                let reloc_type = entry >> 12;
                let reloc_offset = entry & 0x0FFF;

                if reloc_type == 10 {
                    let target_rva = page_va + reloc_offset as u32;
                    let target_off = PE64::vaddr_to_off(&self.sect_hdr, target_rva) as usize;

                    if target_off > 0 && target_off + 8 <= self.raw.len() {
                        let original_val = read_u64_le!(self.raw, target_off);
                        let new_val = original_val.wrapping_add(delta);

                        write_u64_le!(self.raw, target_off, new_val);

                        let patch_addr = base_addr + target_rva as u64;
                        if let Some(mem) = emu.maps.get_mem_by_addr_mut(patch_addr) {
                            mem.force_write_qword(patch_addr, new_val);
                        }
                    }
                }
                entry_off += 2;
            }

            off += block_sz as usize;
        }

        if emu.cfg.verbose >= 1 {
            log::info!("Base Relocations applied successfully.");
        }
    }
}
