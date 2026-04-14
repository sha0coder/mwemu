use crate::emu::Emu;
use crate::loaders::pe::pe32::PE32;
use crate::loaders::pe::pe64::PE64;
use crate::maps::mem64::Permission;
use crate::windows::constants;
use crate::windows::peb::{peb32, peb64};

macro_rules! align_up {
    ($size:expr, $align:expr) => {{
        // Ensure alignment is a power of two at compile time if possible
        ($size + $align - 1) & !($align - 1)
    }};
}

impl Emu {
    /// Prefer PE `ImageBase` when it is in canonical user space and does not overlap existing maps;
    /// otherwise fall back to `lib64_alloc` in `LIBS64_*`.
    fn pick_pe64_dll_base(&mut self, pe64: &PE64) -> u64 {
        const USER_MAX: u64 = 0x7FFF_FFFF_FFFF;
        let ib = pe64.opt.image_base;
        let span = (pe64.opt.size_of_image as u64).max(pe64.size());
        if ib < 0x10000 {
            return self.maps.lib64_alloc(pe64.size()).expect("out of memory");
        }
        let Some(end) = ib.checked_add(span) else {
            return self.maps.lib64_alloc(pe64.size()).expect("out of memory");
        };
        if end > USER_MAX || self.maps.overlaps(ib, span) {
            return self.maps.lib64_alloc(pe64.size()).expect("out of memory");
        }
        ib
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE32, but also from load_library etc.
    /// cyclic stuff: [load_pe] -> [iat-binding]  ->  [load_library] -> [load_pe]
    /// Powered by pe32.rs implementation.
    pub fn load_pe32(&mut self, filename: &str, set_entry: bool, force_base: u32) -> (u32, u32) {
        let is_maps = filename.contains("windows/x86/") ;
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let mut pe32 = PE32::load(filename);
        let base: u32;

        log::trace!("loading pe32 {}", filename);

        /* .rsrc extraction tests
        if set_entry {
            log::trace!("get_resource_by_id");
            pe32.get_resource(Some(3), Some(0), None, None);
        }*/

        // 1. base logic

        // base is forced by libmwemu
        if force_base > 0 {
            if self.maps.overlaps(force_base as u64, pe32.size() as u64) {
                panic!("the forced base address overlaps");
            } else {
                base = force_base;
            }

        // base is setted by user
        } else if !is_maps
            && self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE
            && !self.cfg.emulate_winapi
        {
            base = self.cfg.code_base_addr as u32;
            if self.maps.overlaps(base as u64, pe32.size() as u64) {
                panic!("the setted base address overlaps");
            }

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                if pe32.opt.image_base >= constants::LIBS32_MIN as u32
                    || self
                        .maps
                        .overlaps(pe32.opt.image_base as u64, pe32.mem_size() as u64)
                {
                    base = self
                        .maps
                        .alloc(pe32.mem_size() as u64 + 0xff)
                        .expect("out of memory") as u32;
                } else {
                    base = pe32.opt.image_base;
                }

            // system library
            } else {
                base = self
                    .maps
                    .lib32_alloc(pe32.mem_size() as u64)
                    .expect("out of memory") as u32;
            }
        }

        if set_entry || self.cfg.emulate_winapi {
            // 2. pe binding
            if !is_maps || self.cfg.emulate_winapi {
                pe32.iat_binding(self, base);
                pe32.delay_load_binding(self, base);
                self.base = base as u64;
            }

            // 3. entry point logic
            if self.cfg.entry_point == constants::CFG_DEFAULT_BASE {
                self.regs_mut().rip = base as u64 + pe32.opt.address_of_entry_point as u64;
                log::trace!("entry point at 0x{:x}", self.regs().rip);
            } else {
                self.regs_mut().rip = self.cfg.entry_point;
                log::trace!(
                    "entry point at 0x{:x} but forcing it at 0x{:x}",
                    base as u64 + pe32.opt.address_of_entry_point as u64,
                    self.regs().rip
                );
            }

            log::trace!("base: 0x{:x}", base);
        }

        let sec_allign = pe32.opt.section_alignment;
        // 4. map pe and then sections
        let pemap = self
            .maps
            .create_map(
                &format!("{}.pe", filename2),
                base.into(),
                align_up!(pe32.opt.size_of_headers, sec_allign) as u64,
                Permission::READ_WRITE,
            )
            .expect("cannot create pe map");
        pemap.memcpy(pe32.get_headers(), pe32.opt.size_of_headers as usize);

        for i in 0..pe32.num_of_sections() {
            let ptr = pe32.get_section_ptr(i);
            let sect = pe32.get_section(i);
            let charactis = sect.characteristics;
            let is_exec = charactis & 0x20000000 != 0x0;
            let is_read = charactis & 0x40000000 != 0x0;
            let is_write = charactis & 0x80000000 != 0x0;
            let permission = Permission::from_flags(is_read, is_write, is_exec);

            let sz: u64 = if sect.virtual_size > sect.size_of_raw_data {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if sz == 0 {
                log::trace!("size of section {} is 0", sect.get_name());
                continue;
            }

            let mut sect_name = sect
                .get_name()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\x0a", "")
                .replace("\x0d", "");

            if sect_name.is_empty() {
                sect_name = format!("{:x}", sect.virtual_address);
            }

            let map = match self.maps.create_map(
                &format!("{}{}", filename2, sect_name),
                base as u64 + sect.virtual_address as u64,
                align_up!(sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(_e) => {
                    log::trace!(
                        "weird pe, skipping section {} {} because overlaps",
                        filename2,
                        sect.get_name()
                    );
                    continue;
                }
            };

            if ptr.len() > sz as usize {
                panic!(
                    "overflow {} {} {} {}",
                    filename2,
                    sect.get_name(),
                    ptr.len(),
                    sz
                );
            }
            if !ptr.is_empty() {
                map.memcpy(ptr, ptr.len());
            }
        }

        // 5. ldr table entry creation and link
        if set_entry {
            let _space_addr =
                peb32::create_ldr_entry(self, base, self.regs().rip as u32, &filename2, 0, 0x2c1950);
            let exe_name = self.cfg.exe_name.clone();
            peb32::update_ldr_entry_base(&exe_name, base as u64, self);
        }

        // 6. return values
        let pe_hdr_off = pe32.dos.e_lfanew;
        self.pe32 = Some(pe32);
        (base, pe_hdr_off)
    }

    pub fn map_dll_pe64(&mut self, filename: &str) -> (u64, PE64) {
        let map_name = self.filename_to_mapname(filename);
        let mut pe64 = PE64::load(&filename.to_lowercase());

        let base = self.pick_pe64_dll_base(&pe64);

        let sec_allign = pe64.opt.section_alignment;

        let pemap = match self.maps.create_map(
            &format!("{}.pe", map_name),
            base,
            align_up!(pe64.opt.size_of_headers, sec_allign) as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                panic!("cannot create pe64 map: {}", e);
            }
        };
        pemap.memcpy(pe64.get_headers(), pe64.opt.size_of_headers as usize);
        for i in 0..pe64.num_of_sections() {
            let ptr = pe64.get_section_ptr(i);
            let sect = pe64.get_section(i);
            let charistic = sect.characteristics;
            let is_exec = charistic & 0x20000000 != 0x0;
            let is_read = charistic & 0x40000000 != 0x0;
            let is_write = charistic & 0x80000000 != 0x0;
            let permission = Permission::from_flags(is_read, is_write, is_exec);

            let map_sz: u64 = if sect.virtual_size > 0 {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if map_sz == 0 {
                log::trace!("size of section {} is 0", sect.get_name());
                continue;
            }

            let mut sect_name = sect
                .get_name()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\x0a", "")
                .replace("\x0d", "");

            if sect_name.is_empty() {
                sect_name = format!("{:x}", sect.virtual_address);
            }

            let map = match self.maps.create_map(
                &format!("{}{}", map_name, sect_name),
                base + sect.virtual_address as u64,
                align_up!(map_sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(_e) => {
                    log::trace!(
                        "weird pe, skipping section because overlaps {} {}",
                        map_name,
                        sect.get_name()
                    );
                    continue;
                }
            };

            let copy_len = (sect.size_of_raw_data as usize).min(map_sz as usize).min(ptr.len());
            if copy_len > 0 {
                map.memcpy(&ptr[..copy_len], copy_len);
            }
        }

        pe64.apply_relocations(self, base);

        (base, pe64)
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE64, but also from load_library etc.
    /// cyclic stuff: [load_pe] -> [iat-binding]  ->  [load_library] -> [load_pe]
    /// Powered by pe64.rs implementation.
    pub fn load_pe64(&mut self, filename: &str, set_entry: bool, force_base: u64) -> (u64, u32) {
        let is_maps = filename.contains("windows/x86_64/") || filename.contains("windows/aarch64/") ;
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let mut pe64 = PE64::load(filename);
        let base: u64;

        // 1. base logic

        // base is setted by libmwemu
        if force_base > 0 {
            if self.maps.overlaps(force_base, pe64.size()) {
                panic!("the forced base address overlaps");
            } else {
                base = force_base;
            }

        // base is setted by user
        } else if !is_maps && self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE {
            base = self.cfg.code_base_addr;
            if self.maps.overlaps(base, pe64.size()) {
                panic!("the setted base address overlaps");
            }

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                if pe64.opt.image_base >= constants::LIBS64_MIN {
                    base = self.maps.alloc(pe64.size() + 0xff).expect("out of memory");
                } else if self.maps.overlaps(pe64.opt.image_base, pe64.size()) {
                    base = self.maps.alloc(pe64.size() + 0xff).expect("out of memory");
                } else {
                    base = pe64.opt.image_base;
                }

            // system library
            } else {
                base = self.pick_pe64_dll_base(&pe64);
            }
        }

        if set_entry || self.cfg.emulate_winapi {
            if !is_maps || self.cfg.emulate_winapi {
                self.base = base;
            }

            // 2. entry point logic (relocs + IAT run after PE maps exist; see step 4b below)
            if self.cfg.entry_point == constants::CFG_DEFAULT_BASE {
                self.set_pc(base + pe64.opt.address_of_entry_point as u64);
                log::trace!("entry point at 0x{:x}", self.pc());
            } else {
                self.set_pc(self.cfg.entry_point);
                log::trace!(
                    "entry point at 0x{:x} but forcing it at 0x{:x} by -a flag",
                    base + pe64.opt.address_of_entry_point as u64,
                    self.pc()
                );
            }
            log::trace!("base: 0x{:x}", base);
        }

        let sec_allign = pe64.opt.section_alignment;
        // 4. map pe and then sections
        let pemap = match self.maps.create_map(
            &format!("{}.pe", filename2),
            base,
            align_up!(pe64.opt.size_of_headers, sec_allign) as u64,
            Permission::READ_WRITE,
        ) {
            Ok(m) => m,
            Err(e) => {
                panic!("cannot create pe64 map: {}", e);
            }
        };
        pemap.memcpy(pe64.get_headers(), pe64.opt.size_of_headers as usize);

        for i in 0..pe64.num_of_sections() {
            let ptr = pe64.get_section_ptr(i);
            let sect = pe64.get_section(i);

            let charistic = sect.characteristics;
            let is_exec = charistic & 0x20000000 != 0x0;
            let is_read = charistic & 0x40000000 != 0x0;
            let is_write = charistic & 0x80000000 != 0x0;
            let permission = Permission::from_flags(is_read, is_write, is_exec);

            // Virtual size determines how much address space the section occupies.
            // Raw size is the on-disk data size and may exceed virtual size for
            // packed/overlay sections — using raw size would create an oversized map
            // that overlaps subsequent sections.
            let map_sz: u64 = if sect.virtual_size > 0 {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if map_sz == 0 {
                log::trace!("size of section {} is 0", sect.get_name());
                continue;
            }

            let mut sect_name = sect
                .get_name()
                .replace(" ", "")
                .replace("\t", "")
                .replace("\x0a", "")
                .replace("\x0d", "");

            if sect_name.is_empty() {
                sect_name = format!("{:x}", sect.virtual_address);
            }

            let map = match self.maps.create_map(
                &format!("{}{}", filename2, sect_name),
                base + sect.virtual_address as u64,
                align_up!(map_sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(_e) => {
                    log::trace!(
                        "weird pe, skipping section because overlaps {} {}",
                        filename2,
                        sect.get_name()
                    );
                    continue;
                }
            };

            // Copy only as many bytes as fit in the virtual mapping.
            let copy_len = (sect.size_of_raw_data as usize).min(map_sz as usize).min(ptr.len());

            if copy_len > 0 {
                map.memcpy(&ptr[..copy_len], copy_len);
            }
        }

        // 4b. Base relocs on the mapped image (all load paths, including DLL without emulate_winapi).
        pe64.apply_relocations(self, base);

        if set_entry || self.cfg.emulate_winapi {
            if !is_maps || self.cfg.emulate_winapi {
                // In SSDT + LdrInitializeThunk bootstrap mode, skip eager IAT binding for the main image.
                if !(set_entry && self.cfg.emulate_winapi && self.cfg.emulate_winapi) {
                    pe64.iat_binding(self, base);
                    pe64.delay_load_binding(self, base);
                }
            }
        }

        // 5. ldr table entry creation and link
        if set_entry {
            if !(self.cfg.emulate_winapi && self.cfg.emulate_winapi) {
                let _space_addr =
                    peb64::create_ldr_entry(self, base, self.pc(), &filename2, 0, 0x2c1950);
                let exe_name = self.cfg.exe_name.clone();
                peb64::update_ldr_entry_base(&exe_name, base, self);
            }
            if self.cfg.emulate_winapi && self.cfg.emulate_winapi {
                peb64::update_peb_image_base(self, base);
            }
        }

        // 6. return values
        let pe_hdr_off = pe64.dos.e_lfanew;
        self.pe64 = Some(pe64);
        (base, pe_hdr_off)
    }
}
