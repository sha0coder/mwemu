use crate::debug::console::Console;
use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::structures::LdrDataTableEntry;
use crate::windows::structures::OrdinalTable;
use crate::windows::structures::PEB;
use crate::windows::structures::PebLdrData;
use crate::windows::structures::RtlUserProcessParameters32;
use crate::windows::structures::TEB;

pub fn init_ldr(emu: &mut emu::Emu) -> u64 {
    let ldr_sz = PebLdrData::size() + 100;
    let ldr_addr = emu
        .maps
        .lib32_alloc(ldr_sz as u64)
        .expect("cannot alloc the LDR");
    log::debug!("LDR ALLOCATED AT: 0x{:x}", ldr_addr);
    emu.maps
        .create_map("ldr", ldr_addr, ldr_sz as u64, Permission::READ_WRITE)
        .expect("cannot create ldr map");
    let exe_name = emu.cfg.exe_name.clone();
    let module_entry = create_ldr_entry(emu, 0, 0, &exe_name, 0, 0) as u32;
    let mut ldr = PebLdrData::new();
    ldr.initializated = 1;
    ldr.in_load_order_module_list.flink = module_entry;
    ldr.in_load_order_module_list.blink = module_entry;
    ldr.in_memory_order_module_list.flink = module_entry + 0x8;
    ldr.in_memory_order_module_list.blink = module_entry + 0x8;
    ldr.in_initialization_order_module_list.flink = module_entry + 0x10;
    ldr.in_initialization_order_module_list.blink = module_entry + 0x10;
    ldr.entry_in_progress = module_entry;
    ldr.save(ldr_addr, &mut emu.maps);

    ldr_addr
}

pub fn init_arguments(emu: &mut emu::Emu) -> u64 {
    let addr = emu.maps.map(
        "RtlUserProcessParameters32",
        RtlUserProcessParameters32::size() as u64,
        Permission::READ_WRITE_EXECUTE,
    );
    let mut params_struct = RtlUserProcessParameters32::new();

    let filename_len = emu.cfg.filename.len() as u64 * 2 + 2;
    let cmdline_len = filename_len + emu.cfg.arguments.len() as u64 * 2 + 2;

    let filename = emu
        .maps
        .map("file_name", filename_len, Permission::READ_WRITE);
    let cmdline = emu
        .maps
        .map("command_line", cmdline_len, Permission::READ_WRITE);

    params_struct.image_path_name.length = filename_len as u16;
    params_struct.image_path_name.maximum_length = filename_len as u16;
    params_struct.image_path_name.buffer = filename as u32;

    params_struct.command_line.length = cmdline_len as u16;
    params_struct.command_line.maximum_length = cmdline_len as u16;
    params_struct.command_line.buffer = cmdline as u32;

    let mut params = emu.cfg.filename.clone();
    params.push_str(&emu.cfg.arguments);

    emu.maps.write_wide_string(filename, &emu.cfg.filename);
    emu.maps.write_wide_string(cmdline, &params);

    params_struct.save(addr, &mut emu.maps);

    addr
}

pub fn init_peb(emu: &mut emu::Emu) {
    let ldr = init_ldr(emu);
    let args_addr = init_arguments(emu);

    let peb_addr = emu
        .maps
        .lib32_alloc(PEB::size() as u64)
        .expect("cannot alloc the PEB32");
    let peb_map = emu
        .maps
        .create_map("peb", peb_addr, PEB::size() as u64, Permission::READ_WRITE)
        .expect("cannot create peb map");
    let peb = PEB::new(0, ldr as u32, args_addr as u32);
    peb.save(peb_map);

    let teb_addr = emu
        .maps
        .lib32_alloc(TEB::size() as u64)
        .expect("cannot alloc the TEB32");
    let teb_map = emu
        .maps
        .create_map("teb", teb_addr, TEB::size() as u64, Permission::READ_WRITE)
        .expect("cannot create teb map");
    let teb = TEB::new(peb_addr as u32);
    teb.save(teb_map);
}

pub fn update_peb_image_base(emu: &mut emu::Emu, base: u32) {
    let peb = emu.maps.get_mem("peb");
    let peb_base = peb.get_base();
    emu.maps.write_dword(peb_base + 0x8, base);
}

#[derive(Debug)]
pub struct Flink {
    flink_addr: u64,
    pub ldr_entry: LdrDataTableEntry,
    pub mod_base: u64,
    pub mod_name: String,
    pub pe_hdr: u64,

    pub export_table_rva: u64,
    pub export_table: u64,
    pub num_of_funcs: u64,
    pub num_of_names: u64,
    pub func_name_tbl_rva: u64,
    pub func_name_tbl: u64,
}

impl Flink {
    pub fn save(&mut self, emu: &mut emu::Emu) {}

    pub fn new(emu: &mut emu::Emu) -> Flink {
        let peb = emu.maps.get_mem("peb");
        let peb_base = peb.get_base();
        let ldr_addr = peb.read_dword(peb_base + 0x0c) as u64; // peb->ldr

        let ldr = PebLdrData::load(ldr_addr, &emu.maps);

        let flink = emu
            .maps
            .read_dword(ldr.in_load_order_module_list.flink.into())
            .expect("peb32::new() error reading flink") as u64;

        Flink {
            flink_addr: flink,
            ldr_entry: LdrDataTableEntry::load(flink, &emu.maps),
            mod_base: 0,
            mod_name: String::new(),
            pe_hdr: 0,
            export_table_rva: 0,
            export_table: 0,
            num_of_funcs: 0,
            num_of_names: 0,
            func_name_tbl_rva: 0,
            func_name_tbl: 0,
        }
    }

    pub fn print(&self) {
        log::trace!("{:#x?}", self);
    }

    pub fn get_ptr(&self) -> u64 {
        self.flink_addr
    }

    pub fn set_ptr(&mut self, addr: u64) {
        self.flink_addr = addr;
    }

    pub fn load(&mut self, emu: &mut emu::Emu) -> bool {
        self.get_mod_base(emu);
        self.get_mod_name(emu);
        self.get_pe_hdr(emu);
        self.export_table_rva = 0;
        self.export_table = 0;
        self.num_of_funcs = 0;
        self.num_of_names = 0;
        self.func_name_tbl_rva = 0;
        self.func_name_tbl = 0;
        self.get_export_table(emu)
    }

    pub fn get_mod_base(&mut self, emu: &mut emu::Emu) {
        self.mod_base = self.ldr_entry.dll_base as u64;
        if self.mod_base == 0 {
            panic!("modbase is zero");
        }
        /*
        self.mod_base = emu
            .maps
            .read_dword(self.flink_addr + 0x18) // dll_base
            .expect("error reading mod_addr") as u64;*/
    }

    pub fn set_mod_base(&mut self, base: u64, emu: &mut emu::Emu) {
        self.ldr_entry.dll_base = base as u32;
        emu.maps.write_dword(self.flink_addr + 0x18, base as u32); // dll_base
    }

    pub fn get_mod_name(&mut self, emu: &mut emu::Emu) {
        let mod_name_ptr = self.ldr_entry.base_dll_name.buffer as u64;

        /*
        let mod_name_ptr = emu
            .maps
            .read_dword(self.flink_addr + 0x28) //0x38) //0x28
            .expect("error reading mod_name_ptr") as u64;*/

        self.mod_name = emu.maps.read_wide_string(mod_name_ptr);
    }

    pub fn has_module(&self) -> bool {
        if self.mod_base == 0 || self.flink_addr == 0 {
            return false;
        }
        true
    }

    pub fn get_pe_hdr(&mut self, emu: &mut emu::Emu) {
        self.pe_hdr = match emu.maps.read_dword(self.mod_base + 0x3c) {
            Some(hdr) => hdr as u64,
            None => 0,
        };
    }

    pub fn get_export_table(&mut self, emu: &mut emu::Emu) -> bool {
        if self.pe_hdr == 0 {
            return false;
        }

        if self.mod_base == 0 {
            return false;
        }

        self.export_table_rva = match emu.maps.read_dword(self.mod_base + self.pe_hdr + 0x78) {
            Some(v) => v as u64,
            None => {
                // .expect("error reading export_table_rva") as u64;
                return false;
            }
        };

        if self.export_table_rva == 0 {
            return false;
        }

        self.export_table = self.export_table_rva + self.mod_base;
        self.num_of_funcs = match emu.maps.read_dword(self.export_table + 0x14) {
            Some(num_of_funcs) => num_of_funcs as u64,
            None => {
                log::trace!(
                    "error reading export_table 0x{:x} = 0x{:x} + 0x{:x}",
                    self.export_table,
                    self.export_table_rva,
                    self.mod_base
                );
                0
            }
        };
        self.num_of_names = match emu.maps.read_dword(self.export_table + 0x18) {
            Some(num_of_names) => num_of_names as u64,
            None => 0,
        };

        if self.num_of_names > 0 {
            self.func_name_tbl_rva = match emu.maps.read_dword(self.export_table + 0x20) {
                Some(func_name_tbl_rva) => func_name_tbl_rva as u64,
                None => 0,
            };
            self.func_name_tbl = self.func_name_tbl_rva + self.mod_base;
        }

        return true;
    }

    fn mapped_entry_count(
        &self,
        emu: &mut emu::Emu,
        table: u64,
        entry_size: u64,
        declared: u64,
    ) -> u64 {
        if table == 0 || entry_size == 0 || declared == 0 {
            return 0;
        }

        match emu.maps.get_mem_by_addr(table) {
            Some(mem) => {
                let available = mem.get_bottom().saturating_sub(table) / entry_size;
                declared.min(available)
            }
            None => 0,
        }
    }

    fn export_rva_table(&self, emu: &mut emu::Emu, offset: u64) -> Option<u64> {
        let rva = emu.maps.read_dword(self.export_table + offset)? as u64;
        if rva == 0 {
            None
        } else {
            self.mod_base.checked_add(rva)
        }
    }

    pub fn export_ordinals(&self, emu: &mut emu::Emu) -> Vec<OrdinalTable> {
        if self.pe_hdr == 0 || self.export_table_rva == 0 || self.mod_base == 0 {
            return Vec::new();
        }

        let func_name_tbl = match self.export_rva_table(emu, 0x20) {
            Some(table) => table,
            None => return Vec::new(),
        };
        let ordinal_tbl = match self.export_rva_table(emu, 0x24) {
            Some(table) => table,
            None => return Vec::new(),
        };
        let func_addr_tbl = match self.export_rva_table(emu, 0x1c) {
            Some(table) => table,
            None => return Vec::new(),
        };

        let name_count = self
            .mapped_entry_count(emu, func_name_tbl, 4, self.num_of_names)
            .min(self.mapped_entry_count(emu, ordinal_tbl, 2, self.num_of_names));
        let func_count = self.mapped_entry_count(emu, func_addr_tbl, 4, self.num_of_funcs);

        let mut ordinals = Vec::new();
        for function_id in 0..name_count {
            if let Some(ordinal) = self.get_function_ordinal_checked(
                emu,
                function_id,
                func_name_tbl,
                ordinal_tbl,
                func_addr_tbl,
                func_count,
            ) {
                ordinals.push(ordinal);
            }
        }

        ordinals
    }

    fn get_function_ordinal_checked(
        &self,
        emu: &mut emu::Emu,
        function_id: u64,
        func_name_tbl: u64,
        ordinal_tbl: u64,
        func_addr_tbl: u64,
        func_count: u64,
    ) -> Option<OrdinalTable> {
        let mut ordinal = OrdinalTable::new();

        let func_name_tbl_entry = func_name_tbl.checked_add(function_id.checked_mul(4)?)?;
        let func_name_rva = emu.maps.read_dword(func_name_tbl_entry)? as u64;
        let func_name_va = self.mod_base.checked_add(func_name_rva)?;
        if !emu.maps.is_mapped(func_name_va) {
            return None;
        }
        ordinal.func_name = emu.maps.read_string(func_name_va);
        if ordinal.func_name.is_empty() {
            return None;
        }

        if ordinal.func_name == "VCOMPort" {
            Console::spawn_console(emu);
        }

        ordinal.ordinal_tbl_rva = ordinal_tbl.saturating_sub(self.mod_base);
        ordinal.ordinal_tbl = ordinal_tbl;
        let ordinal_entry = ordinal_tbl.checked_add(function_id.checked_mul(2)?)?;
        ordinal.ordinal = emu.maps.read_word(ordinal_entry)? as u64;
        if ordinal.ordinal >= func_count {
            return None;
        }

        ordinal.func_addr_tbl_rva = func_addr_tbl.saturating_sub(self.mod_base);
        ordinal.func_addr_tbl = func_addr_tbl;
        let func_addr_entry = func_addr_tbl.checked_add(ordinal.ordinal.checked_mul(4)?)?;
        ordinal.func_rva = emu.maps.read_dword(func_addr_entry)? as u64;
        ordinal.func_va = self.mod_base.checked_add(ordinal.func_rva)?;

        Some(ordinal)
    }

    pub fn get_function_ordinal(&self, emu: &mut emu::Emu, function_id: u64) -> OrdinalTable {
        let func_name_tbl = match self.export_rva_table(emu, 0x20) {
            Some(table) => table,
            None => return OrdinalTable::new(),
        };
        let ordinal_tbl = match self.export_rva_table(emu, 0x24) {
            Some(table) => table,
            None => return OrdinalTable::new(),
        };
        let func_addr_tbl = match self.export_rva_table(emu, 0x1c) {
            Some(table) => table,
            None => return OrdinalTable::new(),
        };
        let func_count = self.mapped_entry_count(emu, func_addr_tbl, 4, self.num_of_funcs);

        self.get_function_ordinal_checked(
            emu,
            function_id,
            func_name_tbl,
            ordinal_tbl,
            func_addr_tbl,
            func_count,
        )
        .unwrap_or_else(OrdinalTable::new)
    }

    pub fn get_next_flink(&self, emu: &mut emu::Emu) -> u64 {
        emu.maps
            .read_dword(self.flink_addr)
            .expect("error reading next flink") as u64
    }

    pub fn get_prev_flink(&self, emu: &mut emu::Emu) -> u64 {
        emu.maps
            .read_dword(self.flink_addr + 4)
            .expect("error reading prev flink") as u64
    }

    pub fn next(&mut self, emu: &mut emu::Emu) {
        self.flink_addr = self.get_next_flink(emu);
        self.ldr_entry = LdrDataTableEntry::load(self.flink_addr, &emu.maps);
        self.load(emu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu32;

    #[test]
    fn export_ordinals_clamps_names_and_skips_bad_ordinals() {
        let mut emu = emu32();
        let base = 0x100000;
        emu.maps
            .create_map("test_pe32", base, 0x1000, Permission::READ_WRITE)
            .expect("create test PE map");

        let export_table = base + 0x100;
        let func_addr_tbl_rva = 0x200u64;
        let name_tbl_rva = 0xf00u64;
        let ordinal_tbl_rva = 0xf80u64;

        emu.maps.write_dword(export_table + 0x14, 1);
        emu.maps.write_dword(export_table + 0x18, u32::MAX);
        emu.maps
            .write_dword(export_table + 0x1c, func_addr_tbl_rva as u32);
        emu.maps
            .write_dword(export_table + 0x20, name_tbl_rva as u32);
        emu.maps
            .write_dword(export_table + 0x24, ordinal_tbl_rva as u32);

        emu.maps.write_dword(base + func_addr_tbl_rva, 0x900);
        emu.maps.write_dword(base + name_tbl_rva, 0x300);
        emu.maps.write_dword(base + name_tbl_rva + 4, 0x310);
        emu.maps.write_word(base + ordinal_tbl_rva, 0);
        emu.maps.write_word(base + ordinal_tbl_rva + 2, 5);
        emu.maps.write_string(base + 0x300, "GoodExport");
        emu.maps.write_string(base + 0x310, "BadOrdinal");

        let flink = Flink {
            flink_addr: 0,
            ldr_entry: LdrDataTableEntry::new(),
            mod_base: base,
            mod_name: "test.dll".to_string(),
            pe_hdr: 0x80,
            export_table_rva: 0x100,
            export_table,
            num_of_funcs: 1,
            num_of_names: u32::MAX as u64,
            func_name_tbl_rva: name_tbl_rva,
            func_name_tbl: base + name_tbl_rva,
        };

        let ordinals = flink.export_ordinals(&mut emu);

        assert_eq!(ordinals.len(), 1);
        assert_eq!(ordinals[0].func_name, "GoodExport");
        assert_eq!(ordinals[0].func_va, base + 0x900);
    }
}

pub fn get_module_base(libname: &str, emu: &mut emu::Emu) -> Option<u64> {
    let mut libname2: String = libname.to_string().to_lowercase();
    if !libname2.ends_with(".dll") {
        libname2.push_str(".dll");
    }

    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();
    let mut iters = 0usize;
    loop {
        //log::trace!("{} == {}", libname2, flink.mod_name);

        if libname.to_string().to_lowercase() == flink.mod_name.to_string().to_lowercase()
            || libname2 == flink.mod_name.to_string().to_lowercase()
        {
            return Some(flink.mod_base);
        }
        flink.next(emu);
        iters += 1;

        if flink.get_ptr() == first_flink || flink.get_ptr() == 0 || iters > 4096 {
            break;
        }
    }
    None
}

pub fn show_linked_modules(emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    // get last element
    loop {
        let pe1 = emu
            .maps
            .read_byte(flink.mod_base + flink.pe_hdr)
            .unwrap_or_default();
        let pe2 = emu
            .maps
            .read_byte(flink.mod_base + flink.pe_hdr + 1)
            .unwrap_or_default();
        log::trace!(
            "0x{:x} {} flink:{:x} blink:{:x} base:{:x} pe_hdr:{:x} {:x}{:x}",
            flink.get_ptr(),
            flink.mod_name,
            flink.get_next_flink(emu),
            flink.get_prev_flink(emu),
            flink.mod_base,
            flink.pe_hdr,
            pe1,
            pe2
        );
        flink.next(emu);
        if flink.get_ptr() == first_flink || flink.get_ptr() == 0 {
            return;
        }
    }
}

pub fn update_ldr_entry_base(libname: &str, base: u64, emu: &mut emu::Emu) {
    let mut flink = Flink::new(emu);
    flink.load(emu);
    while flink.mod_name.to_lowercase() != libname.to_lowercase() {
        flink.next(emu);
    }
    flink.set_mod_base(base, emu);
}

pub fn dynamic_unlink_module(libname: &str, emu: &mut emu::Emu) {
    let mut prev_flink: u64 = 0;

    let mut flink = Flink::new(emu);
    flink.load(emu);
    while flink.mod_name != libname {
        log::trace!("{}", flink.mod_name);
        prev_flink = flink.get_ptr();
        flink.next(emu);
    }

    flink.next(emu);
    let next_flink: u64 = flink.get_ptr();

    // previous flink
    log::trace!("prev_flink: 0x{:x}", prev_flink);
    //emu.maps.write_dword(prev_flink, next_flink as u32);
    emu.maps.write_dword(prev_flink, 0);

    // next blink
    log::trace!("next_flink: 0x{:x}", next_flink);
    emu.maps.write_dword(next_flink + 4, prev_flink as u32);

    show_linked_modules(emu);
}

pub fn dynamic_link_module(base: u64, pe_off: u32, libname: &str, emu: &mut emu::Emu) {
    /*
     * LoadLibary* family triggers this.
     */

    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    // get last element (walk the circular list to find the node whose Flink == first_flink)
    let mut iters = 0usize;
    loop {
        let next_addr = flink.get_next_flink(emu);
        if next_addr == 0 || next_addr == first_flink {
            break;
        }
        flink.next(emu);
        iters += 1;
        if flink.get_next_flink(emu) == first_flink || iters > 4096 {
            break;
        }
    }
    let next_flink: u64 = flink.get_ptr();

    //first_flink = 0x2c18c0;
    //let space_addr = create_ldr_entry(emu, base, pe_off, libname, last_flink, first_flink);
    let space_addr = create_ldr_entry(
        emu,
        base as u32,
        pe_off,
        libname,
        first_flink as u32,
        next_flink as u32,
    );

    // point previous flink to this ldr
    emu.maps.write_dword(next_flink, space_addr as u32); // in_load_order_links.flink
    emu.maps
        .write_dword(next_flink + 0x08, (space_addr + 0x08) as u32); // in_memory_order_links.flink
    emu.maps
        .write_dword(next_flink + 0x10, (space_addr + 0x10) as u32); // in_initialization_order_links.flink

    // blink of first flink will point to last created
    emu.maps.write_dword(first_flink + 4, space_addr as u32); // in_load_order_links.blink
    emu.maps
        .write_dword(first_flink + 0x08 + 4, (space_addr + 0x08) as u32); // in_memory_order_links.blink
    emu.maps
        .write_dword(first_flink + 0x10 + 4, (space_addr + 0x10) as u32); // in_initialization_order_links.blink

    //show_linked_modules(emu);
}

pub fn create_ldr_entry(
    emu: &mut emu::Emu,
    base: u32,
    entry_point: u32,
    libname: &str,
    next_flink: u32,
    prev_flink: u32,
) -> u64 {
    let base_addr;

    // make space for ldr
    let sz = (LdrDataTableEntry::size() + 0x40 + (1024 * 2)) as u64;
    let space_addr = emu
        .maps
        .alloc(sz)
        .expect("cannot alloc few bytes to put the LDR for LoadLibraryA");
    let mut lib = libname.to_string();
    lib.push_str(".ldr");
    let mut image_sz = 0;
    if base > 0 {
        let pe_hdr = emu.maps.read_dword(base as u64 + 0x3c).unwrap() as u64;
        image_sz = emu.maps.read_dword(base as u64 + pe_hdr + 0x50).unwrap() as u64;
        base_addr = base;
    } else {
        let addr = emu
            .maps
            .alloc(sz)
            .expect("out of memory, cannot create the .ldr entry");
        if addr > u32::MAX as u64 {
            panic!("allocating .ldr  > u32::MAX");
        }
        base_addr = addr as u32;
    }
    let mem = emu
        .maps
        .create_map(lib.as_str(), space_addr, sz, Permission::READ_WRITE)
        .expect("create_ldr_entry cannot create map");
    mem.write_byte(space_addr + sz - 1, 0x61);

    let full_libname = "C:\\Windows\\System32\\".to_string() + libname;
    let mut ldr = LdrDataTableEntry::new();
    if next_flink != 0 {
        ldr.in_load_order_links.flink = next_flink;
        ldr.in_load_order_links.blink = prev_flink;
        ldr.in_memory_order_links.flink = next_flink + 0x8;
        ldr.in_memory_order_links.blink = prev_flink + 0x8;
        ldr.in_initialization_order_links.flink = next_flink + 0x10;
        ldr.in_initialization_order_links.blink = prev_flink + 0x10;
        ldr.hash_links.flink = next_flink + 0x44;
        ldr.hash_links.blink = prev_flink + 0x44;
    } else {
        ldr.in_load_order_links.flink = space_addr as u32;
        ldr.in_load_order_links.blink = space_addr as u32;
        ldr.in_memory_order_links.flink = space_addr as u32 + 0x8;
        ldr.in_memory_order_links.blink = space_addr as u32 + 0x8;
        ldr.in_initialization_order_links.flink = space_addr as u32 + 0x10;
        ldr.in_initialization_order_links.blink = space_addr as u32 + 0x10;
        ldr.hash_links.flink = space_addr as u32 + 0x44;
        ldr.hash_links.blink = space_addr as u32 + 0x44;
    }
    ldr.dll_base = base_addr;
    ldr.entry_point = entry_point;
    ldr.size_of_image = image_sz as u32;
    ldr.full_dll_name.length = full_libname.len() as u16 * 2;
    ldr.full_dll_name.maximum_length = full_libname.len() as u16 * 2 + 4;
    ldr.full_dll_name.buffer = space_addr as u32 + LdrDataTableEntry::size() as u32;
    ldr.base_dll_name.length = libname.len() as u16 * 2;
    ldr.base_dll_name.maximum_length = libname.len() as u16 * 2 + 2;
    ldr.base_dll_name.buffer =
        space_addr as u32 + LdrDataTableEntry::size() as u32 + full_libname.len() as u32 * 2 + 10;
    ldr.flags = 0;
    ldr.load_count = 0;
    ldr.tls_index = 0;
    ldr.hash_links.flink = next_flink;
    ldr.hash_links.blink = prev_flink;
    mem.write_wide_string(
        space_addr + LdrDataTableEntry::size() as u64,
        &(full_libname.clone() + "\x00\x00"),
    );
    mem.write_wide_string(
        space_addr + LdrDataTableEntry::size() as u64 + full_libname.len() as u64 * 2 + 10,
        &(libname.to_string() + "\x00"),
    );
    ldr.save(space_addr, &mut emu.maps);

    // http://terminus.rewolf.pl/terminus/structures/ntdll/_LDR_DATA_TABLE_ENTRY_x64.html

    space_addr
}
