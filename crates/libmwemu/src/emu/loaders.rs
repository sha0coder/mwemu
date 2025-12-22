use iced_x86::Register;

use crate::constants;
use crate::elf::elf32::Elf32;
use crate::elf::elf64::Elf64;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::pe::pe32::PE32;
use crate::pe::pe64::PE64;
use crate::peb::{peb32, peb64};

macro_rules! align_up {
    ($size:expr, $align:expr) => {{
        // Ensure alignment is a power of two at compile time if possible
        ($size + $align - 1) & !($align - 1)
    }};
}

impl Emu {
    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE32, but also from load_library etc.
    /// Powered by pe32.rs implementation.
    pub fn load_pe32(&mut self, filename: &str, set_entry: bool, force_base: u32) -> (u32, u32) {
        let is_maps = filename.contains("maps32/");
        let map_name = self.filename_to_mapname(filename);
        let filename2 = map_name;
        let mut pe32 = PE32::load(filename);
        let base: u32;

        log::info!("loading pe32 {}", filename);

        /* .rsrc extraction tests
        if set_entry {
            log::info!("get_resource_by_id");
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
        } else if !is_maps && self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE {
            base = self.cfg.code_base_addr as u32;
            if self.maps.overlaps(base as u64, pe32.size() as u64) {
                panic!("the setted base address overlaps");
            }

        // base is setted by image base (if overlapps, alloc)
        } else {
            // user's program
            if set_entry {
                if pe32.opt.image_base >= constants::LIBS32_MIN as u32 {
                    base = self
                        .maps
                        .alloc(pe32.mem_size() as u64 + 0xff)
                        .expect("out of memory") as u32;
                } else if self
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

        if set_entry {
            // 2. pe binding
            if !is_maps {
                pe32.iat_binding(self);
                pe32.delay_load_binding(self);
                self.base = base as u64;
            }

            // 3. entry point logic
            if self.cfg.entry_point == constants::CFG_DEFAULT_BASE {
                self.regs_mut().rip = base as u64 + pe32.opt.address_of_entry_point as u64;
                log::info!("entry point at 0x{:x}", self.regs().rip);
            } else {
                self.regs_mut().rip = self.cfg.entry_point;
                log::info!(
                    "entry point at 0x{:x} but forcing it at 0x{:x}",
                    base as u64 + pe32.opt.address_of_entry_point as u64,
                    self.regs().rip
                );
            }

            log::info!("base: 0x{:x}", base);
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
                log::info!("size of section {} is 0", sect.get_name());
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
                Err(e) => {
                    log::info!(
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
            let space_addr = peb32::create_ldr_entry(
                self,
                base,
                self.regs().rip as u32,
                &filename2.clone(),
                0,
                0x2c1950,
            );
            peb32::update_ldr_entry_base(constants::EXE_NAME, base as u64, self);
        }

        // 6. return values
        let pe_hdr_off = pe32.dos.e_lfanew;
        self.pe32 = Some(pe32);
        (base, pe_hdr_off)
    }

    /// Complex funtion called from many places and with multiple purposes.
    /// This is called from load_code() if sample is PE64, but also from load_library etc.
    /// Powered by pe64.rs implementation.
    pub fn load_pe64(&mut self, filename: &str, set_entry: bool, force_base: u64) -> (u64, u32) {
        let is_maps = filename.contains("maps64/");
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
                base = self.maps.lib64_alloc(pe64.size()).expect("out of memory");
            }
        }

        if set_entry {
            // 2. pe binding
            if !is_maps {
                pe64.iat_binding(self);
                pe64.delay_load_binding(self);
                self.base = base;
            }

            // 3. entry point logic
            if self.cfg.entry_point == constants::CFG_DEFAULT_BASE {
                self.regs_mut().rip = base + pe64.opt.address_of_entry_point as u64;
                log::info!("entry point at 0x{:x}", self.regs().rip);
            } else {
                self.regs_mut().rip = self.cfg.entry_point;
                log::info!(
                    "entry point at 0x{:x} but forcing it at 0x{:x} by -a flag",
                    base + pe64.opt.address_of_entry_point as u64,
                    self.regs().rip
                );
            }
            log::info!("base: 0x{:x}", base);
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
                panic!("annot create pe64 map: {}", e);
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

            let sz: u64 = if sect.virtual_size > sect.size_of_raw_data {
                sect.virtual_size as u64
            } else {
                sect.size_of_raw_data as u64
            };

            if sz == 0 {
                log::info!("size of section {} is 0", sect.get_name());
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
                align_up!(sz, sec_allign as u64),
                permission,
            ) {
                Ok(m) => m,
                Err(e) => {
                    log::info!(
                        "weird pe, skipping section because overlaps {} {}",
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
            let space_addr = peb64::create_ldr_entry(
                self,
                base,
                self.regs().rip,
                &filename2.clone(),
                0,
                0x2c1950,
            );
            peb64::update_ldr_entry_base(constants::EXE_NAME, base, self);
        }

        // 6. return values
        let pe_hdr_off = pe64.dos.e_lfanew;
        self.pe64 = Some(pe64);
        (base, pe_hdr_off)
    }

    /// Loads an ELF64 parsing sections etc, powered by elf64.rs
    /// This is called from load_code() if the sample is ELF64
    pub fn load_elf64(&mut self, filename: &str) {
        let mut elf64 = Elf64::parse(filename).unwrap();
        let dyn_link = !elf64.get_dynamic().is_empty();

        if dyn_link {
            log::info!("dynamic elf64 detected.");
        } else {
            log::info!("static elf64 detected.");
        }

        elf64.load(
            &mut self.maps,
            "elf64",
            false,
            dyn_link,
            self.cfg.code_base_addr,
        );
        self.init_linux64(dyn_link);

        // Get .text addr and size
        let mut text_addr: u64 = 0;
        let mut text_sz = 0;
        for i in 0..elf64.elf_shdr.len() {
            let sname = elf64.get_section_name(elf64.elf_shdr[i].sh_name as usize);
            if sname == ".text" {
                text_addr = elf64.elf_shdr[i].sh_addr;
                text_sz = elf64.elf_shdr[i].sh_size;
                break;
            }
        }

        if text_addr == 0 {
            panic!(".text not found on this elf64");
        }

        // entry point logic:

        // 1. Configured entry point
        if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
            log::info!("forcing entry point to 0x{:x}", self.cfg.entry_point);
            self.regs_mut().rip = self.cfg.entry_point;

        // 2. Entry point pointing inside .text
        } else if elf64.elf_hdr.e_entry >= text_addr && elf64.elf_hdr.e_entry < text_addr + text_sz
        {
            log::info!(
                "Entry point pointing to .text 0x{:x}",
                elf64.elf_hdr.e_entry
            );
            self.regs_mut().rip = elf64.elf_hdr.e_entry;

        // 3. Entry point points above .text, relative entry point
        } else if elf64.elf_hdr.e_entry < text_addr {
            self.regs_mut().rip = elf64.elf_hdr.e_entry + elf64.base; //text_addr;
            log::info!(
                "relative entry point: 0x{:x}  fixed: 0x{:x}",
                elf64.elf_hdr.e_entry,
                self.regs().rip
            );

        // 4. Entry point points below .text, weird case.
        } else {
            panic!(
                "Entry points is pointing below .text 0x{:x}",
                elf64.elf_hdr.e_entry
            );
        }

        /*
        if dyn_link {
            //let mut ld = Elf64::parse("/lib64/ld-linux-x86-64.so.2").unwrap();
            //ld.load(&mut self.maps, "ld-linux", true, dyn_link, constants::CFG_DEFAULT_BASE);
            //log::info!("--- emulating ld-linux _start ---");

            self.regs_mut().rip = elf64.elf_hdr.e_entry;

            //TODO: emulate the linker
            //self.regs_mut().rip = ld.elf_hdr.e_entry + elf64::LD_BASE;
            //self.run(None);
        } else {
            self.regs_mut().rip = elf64.elf_hdr.e_entry;
        }*/

        /*
        for lib in elf64.get_dynamic() {
            log::info!("dynamic library {}", lib);
            let libspath = "/usr/lib/x86_64-linux-gnu/";
            let libpath = format!("{}{}", libspath, lib);
            let mut elflib = Elf64::parse(&libpath).unwrap();
            elflib.load(&mut self.maps, &lib, true);

            if lib.contains("libc") {
                elflib.craft_libc_got(&mut self.maps, "elf64");
            }

            /*
            match elflib.init {
                Some(addr) => {
                    self.call64(addr, &[]);
                }
                None => {}
            }*/
        }*/
    }

    /// Load a sample. It can be PE32, PE64, ELF32, ELF64 or shellcode.
    /// If its a shellcode cannot be known if is for windows or linux, it triggers also init() to
    /// setup windows simulator.
    /// For now mwemu also don't know if shellcode is for 32bits or 64bits, in commandline -6 has
    /// to be selected for indicating 64bits, and from python or rust the emu32() or emu64()
    /// construtor dtermines the engine.
    pub fn load_code(&mut self, filename: &str) {
        self.filename = filename.to_string();
        self.cfg.filename = self.filename.clone();

        //let map_name = self.filename_to_mapname(filename);
        //self.cfg.filename = map_name;

        // ELF32
        if Elf32::is_elf32(filename) && !self.cfg.shellcode {
            self.linux = true;
            self.cfg.is_64bits = false;

            log::info!("elf32 detected.");
            let mut elf32 = Elf32::parse(filename).unwrap();
            elf32.load(&mut self.maps);
            self.regs_mut().rip = elf32.elf_hdr.e_entry.into();
            let stack_sz = 0x30000;
            let stack = self.alloc("stack", stack_sz, Permission::READ_WRITE);
            self.regs_mut().rsp = stack + (stack_sz / 2);
            //unimplemented!("elf32 is not supported for now");


        // ELF64
        } else if Elf64::is_elf64(filename) && !self.cfg.shellcode {
            self.linux = true;
            self.cfg.is_64bits = true;
            self.maps.clear();

            let base = self.load_elf64(filename);

        // PE32
        } else if !self.cfg.is_64bits && PE32::is_pe32(filename) && !self.cfg.shellcode {
            log::info!("PE32 header detected.");
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.init(clear_registers, clear_flags);
            let (base, pe_off) = self.load_pe32(filename, true, 0);
            let ep = self.regs().rip;
            // emulating tls callbacks

            /*
            for i in 0..self.tls_callbacks.len() {
                self.regs_mut().rip = self.tls_callbacks[i];
                log::info!("emulating tls_callback {} at 0x{:x}", i + 1, self.regs().rip);
                self.stack_push32(base);
                self.run(Some(base as u64));
            }*/

            self.regs_mut().rip = ep;

        // PE64
        } else if self.cfg.is_64bits && PE64::is_pe64(filename) && !self.cfg.shellcode {
            log::info!("PE64 header detected.");
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.init(clear_registers, clear_flags);
            let (base, pe_off) = self.load_pe64(filename, true, 0);
            let ep = self.regs().rip;

            match self.pe64 {
                Some(ref pe64) => {
                    // start loading dll
                    if pe64.is_dll() {
                        self.regs_mut().set_reg(Register::RCX, base);
                        self.regs_mut().set_reg(Register::RDX, 1);
                        self.regs_mut().set_reg(Register::R8L, 0);
                    }
                }
                _ => {
                    log::error!("No Pe64 found inside self");
                }
            }
            // emulating tls callbacks
            /*
            for i in 0..self.tls_callbacks.len() {
                self.regs_mut().rip = self.tls_callbacks[i];
                log::info!("emulating tls_callback {} at 0x{:x}", i + 1, self.regs().rip);
                self.stack_push64(base);
                self.run(Some(base));
            }*/

            self.regs_mut().rip = ep;

        // Shellcode
        } else {
            log::info!("shellcode detected.");
            let clear_registers = false; // TODO: this needs to be more dynamic, like if we have a register set via args or not
            let clear_flags = false; // TODO: this needs to be more dynamic, like if we have a flag set via args or not
            self.init(clear_registers, clear_flags);
            if self.cfg.is_64bits {
                let (base, pe_off) = self.load_pe64(
                    &format!("{}/{}", self.cfg.maps_folder, constants::EXE_NAME),
                    false,
                    0,
                );
                peb64::update_ldr_entry_base(constants::EXE_NAME, base, self);
            } else {
                let (base, pe_off) = self.load_pe32(
                    &format!("{}/{}", self.cfg.maps_folder, constants::EXE_NAME),
                    false,
                    0,
                );
                peb32::update_ldr_entry_base(constants::EXE_NAME, base as u64, self);
            }

            if !self
                .maps
                .create_map(
                    "code",
                    self.cfg.code_base_addr,
                    0,
                    Permission::READ_WRITE_EXECUTE,
                )
                .expect("cannot create code map")
                .load(filename)
            {
                log::info!("shellcode not found, select the file with -f");
                std::process::exit(1);
            }
            let code = self.maps.get_mem_mut("code");
            code.extend(0xffff); // this could overlap an existing map
        }

        if self.cfg.entry_point != constants::CFG_DEFAULT_BASE {
            self.regs_mut().rip = self.cfg.entry_point;
        }

        /*if self.cfg.code_base_addr != constants::CFG_DEFAULT_BASE {
            let code = self.maps.get_mem("code");
            code.update_base(self.cfg.code_base_addr);
            code.update_bottom(self.cfg.code_base_addr + code.size() as u64);
        }*/
    }

    /// Load a shellcode from a variable.
    /// This assumes that there is no headers like PE/ELF and it's direclty code.
    /// Any OS simulation is triggered, but init() could be called by the user
    pub fn load_code_bytes(&mut self, bytes: &[u8]) {
        if self.cfg.verbose >= 1 {
            log::info!("Loading shellcode from bytes");
        }

        self.init_cpu();

        let code = self
            .maps
            .create_map(
                "code",
                self.cfg.code_base_addr,
                bytes.len() as u64,
                Permission::READ_WRITE_EXECUTE,
            )
            .expect("cannot create code map");
        let base = code.get_base();
        code.write_bytes(base, bytes);
        self.regs_mut().rip = code.get_base();
    }
}
