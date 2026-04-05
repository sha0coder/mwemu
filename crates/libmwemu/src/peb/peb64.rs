use crate::emu;
use crate::maps::mem64::Permission;
use crate::structures::LdrDataTableEntry64;
use crate::structures::OrdinalTable;
use crate::structures::PebLdrData64;
use crate::structures::RtlUserProcessParameters64;
use crate::structures::PEB64;
use crate::structures::TEB64;

const NTDLL_LDRP_HASH_TABLE_RVA64: u64 = 0x1d30e0;
const NTDLL_LDRP_HASH_BUCKETS64: u64 = 32;
const LDR_HASH_LINKS_OFFSET64: u64 = 0x7f;
const NTDLL_LDRP_GLOBAL_2680_RVA64: u64 = 0x1d2680;
const NTDLL_LDRP_GLOBAL_26C0_RVA64: u64 = 0x1d26c0;
const NTDLL_LDRP_GLOBAL_26F0_RVA64: u64 = 0x1d26f0;

/// `PEB+0x90` (`system_dependent_07`): ntdll reads this as a pointer in loader / API-set helpers.
/// Real CSRSS/kernel fills it; under emulation `LdrInitializeThunk` (and related code) often leaves it
/// **NULL**, which makes `mov rax,[peb+90h]` / `test rax,rax` / later `[rax]` fault.  Our
/// `PEB64::new` also defaulted this field to zero.
///
/// If the slot is **NULL** or points at **unmapped** memory, install or reuse a dedicated
/// zero-filled page and write its base to `PEB+0x90`.  If guest code already placed a mapped
/// non-zero pointer, leave it alone.
pub fn ensure_peb_system_dependent_07(emu: &mut emu::Emu) {
    if !emu.cfg.is_x64() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };
    let cur = emu.maps.read_qword(peb_base + 0x90).unwrap_or(0);
    if cur != 0 && emu.maps.is_mapped(cur) {
        return;
    }

    const SD07_NAME: &str = "peb_system_dependent_07";
    const SD07_SZ: u64 = 0x1000;
    let sd07 = if let Some(m) = emu.maps.get_map_by_name(SD07_NAME) {
        m.get_base()
    } else {
        let base = emu.maps.map(SD07_NAME, SD07_SZ, Permission::READ_WRITE);
        emu.maps.memset(base, 0, SD07_SZ as usize);
        base
    };
    emu.maps.write_qword(peb_base + 0x90, sd07);
}

/// `TEB+0x2C8` points at the thread activation-context stack. On the real loader path reached
/// after `RtlIsProcessorFeaturePresent(0x1c)`, ntdll dereferences this pointer during early
/// activation-context setup. A NULL pointer crashes immediately at `mov rax,[teb+2c8] / cmp [rax],...`.
///
/// Bootstrap only needs a mapped "empty stack" object. Model the front of the structure as:
/// - `+0x00` current active frame pointer = NULL
/// - `+0x08`/`+0x10` cached free-list LIST_ENTRY = self-linked
/// - rest zeroed
fn ensure_teb_activation_context_stack(emu: &mut emu::Emu) {
    if !emu.cfg.is_x64() {
        return;
    }
    let teb_base = match emu.maps.get_map_by_name("teb") {
        Some(m) => m.get_base(),
        None => return,
    };
    let cur = emu.maps.read_qword(teb_base + 0x2c8).unwrap_or(0);
    if cur != 0 && emu.maps.is_mapped(cur) {
        return;
    }

    const ACTCTX_NAME: &str = "teb_activation_context_stack";
    const ACTCTX_SZ: u64 = 0x1000;
    let actctx = if let Some(m) = emu.maps.get_map_by_name(ACTCTX_NAME) {
        m.get_base()
    } else {
        let base = emu.maps.map(ACTCTX_NAME, ACTCTX_SZ, Permission::READ_WRITE);
        emu.maps.memset(base, 0, ACTCTX_SZ as usize);
        emu.maps.write_qword(base + 0x08, base + 0x08);
        emu.maps.write_qword(base + 0x10, base + 0x08);
        base
    };
    emu.maps.write_qword(teb_base + 0x2c8, actctx);
}

/// `PEB+0x68` (`system_dependent_06`) is consumed by loader-side helpers on the "real" path taken
/// once `RtlIsProcessorFeaturePresent(0x1c)` returns true. In the minimal `--ssdt --init` setup we
/// used to leave this slot NULL, which later crashed at `cmp byte ptr [rcx], 7` after loading
/// `rcx = [peb+0x68]`.
///
/// For bootstrap we only need a stable mapped blob with a plausible version byte. The reference
/// path checks the first byte against `7`, so initialize a small zeroed page and set byte 0 to 7.
fn ensure_peb_system_dependent_06(emu: &mut emu::Emu) {
    if !emu.cfg.is_x64() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };
    let cur = emu.maps.read_qword(peb_base + 0x68).unwrap_or(0);
    if cur != 0 && emu.maps.is_mapped(cur) {
        return;
    }

    const SD06_NAME: &str = "peb_system_dependent_06";
    const SD06_SZ: u64 = 0x1000;
    let sd06 = if let Some(m) = emu.maps.get_map_by_name(SD06_NAME) {
        m.get_base()
    } else {
        let base = emu.maps.map(SD06_NAME, SD06_SZ, Permission::READ_WRITE);
        emu.maps.memset(base, 0, SD06_SZ as usize);
        let _ = emu.maps.write_byte(base, 7);
        base
    };
    emu.maps.write_qword(peb_base + 0x68, sd06);
}

/// Map a minimal NLS data page for each of the three code-page table pointers stored in the PEB:
///   PEB+0xA0  AnsiCodePageData
///   PEB+0xA8  OemCodePageData
///   PEB+0xB0  UnicodeCaseTableData
///
/// `PEB64::new` hard-codes real Windows addresses (e.g. `0x7fffffb0000`) that are valid in an
/// actual process but are never mapped by the emulator. ntdll reads `[ptr+2]` (CodePage WORD)
/// from each table during early loader initialisation. We create a 0x1000-byte zeroed page and
/// store the right pointer into the slot; the on-disk NLS files are not needed because the code
/// only checks whether the pointer is non-NULL / readable, and the exact table content is not
/// required for `LdrInitializeThunk` to complete.
pub fn ensure_peb_nls_tables(emu: &mut emu::Emu) {
    if !emu.cfg.is_x64() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };

    const NLS_SZ: u64 = 0x1000;
    let slots: &[(&str, u64)] = &[
        ("peb_nls_ansi",    peb_base + 0xA0),
        ("peb_nls_oem",     peb_base + 0xA8),
        ("peb_nls_unicode", peb_base + 0xB0),
    ];
    for (name, slot_addr) in slots {
        let cur = emu.maps.read_qword(*slot_addr).unwrap_or(0);
        // Align down to page boundary to test if the page is mapped
        let page = cur & !0xFFF;
        if cur != 0 && page != 0 && emu.maps.is_mapped(page) {
            continue;
        }
        let base = if let Some(m) = emu.maps.get_map_by_name(name) {
            m.get_base()
        } else {
            let b = emu.maps.map(name, NLS_SZ, Permission::READ_WRITE);
            emu.maps.memset(b, 0, NLS_SZ as usize);
            b
        };
        emu.maps.write_qword(*slot_addr, base);
    }
}

pub fn init_ldr(emu: &mut emu::Emu) -> u64 {
    let ldr_sz = PebLdrData64::size() + 100;
    let ldr_addr = emu
        .maps
        .lib64_alloc(ldr_sz as u64)
        .expect("cannot alloc the LDR");
    emu.maps
        .create_map("ldr", ldr_addr, ldr_sz as u64, Permission::READ_WRITE)
        .expect("cannot create ldr map");
    let exe_name = emu.cfg.exe_name.clone();
    let module_entry = create_ldr_entry(emu, 0, 0, &exe_name, 0, 0);
    let mut ldr = PebLdrData64::new();
    ldr.initializated = 1;
    ldr.in_load_order_module_list.flink = module_entry;
    ldr.in_load_order_module_list.blink = module_entry;
    ldr.in_memory_order_module_list.flink = module_entry + 0x10;
    ldr.in_memory_order_module_list.blink = module_entry + 0x10;
    ldr.in_initialization_order_module_list.flink = module_entry + 0x20;
    ldr.in_initialization_order_module_list.blink = module_entry + 0x20;
    ldr.entry_in_progress.flink = module_entry;
    ldr.entry_in_progress.blink = module_entry;
    ldr.save(ldr_addr, &mut emu.maps);

    ldr_addr
}

pub fn init_arguments(emu: &mut emu::Emu) -> u64 {
    let addr = emu.maps.map(
        "RtlUserProcessParameters64",
        RtlUserProcessParameters64::size() as u64,
        Permission::READ_WRITE_EXECUTE,
    );
    let mut params_struct = RtlUserProcessParameters64::new();

    let filename_len = emu.cfg.filename.len() as u64 * 2 + 2;
    let cmdline_len = filename_len + emu.cfg.arguments.len() as u64 * 2 + 2;

    let filename = emu
        .maps
        .map("file_name", filename_len, Permission::READ_WRITE);
    let cmdline = emu
        .maps
        .map("command_line", cmdline_len, Permission::READ_WRITE);

    let dll_path_buf = emu.maps.map("dll_path", 4, Permission::READ_WRITE);
    emu.maps.write_wide_string(dll_path_buf, "");

    params_struct.image_path_name.length = filename_len as u16;
    params_struct.image_path_name.maximum_length = filename_len as u16;
    params_struct.image_path_name.buffer = filename;

    params_struct.command_line.length = cmdline_len as u16;
    params_struct.command_line.maximum_length = cmdline_len as u16;
    params_struct.command_line.buffer = cmdline;

    params_struct.dll_path.length = 0;
    params_struct.dll_path.maximum_length = 4;
    params_struct.dll_path.buffer = dll_path_buf;

    let mut params = emu.cfg.filename.clone();
    params.push_str(&emu.cfg.arguments);

    emu.maps.write_wide_string(filename, &emu.cfg.filename);
    emu.maps.write_wide_string(cmdline, &params);

    params_struct.save(addr, &mut emu.maps);

    addr
}

pub fn init_peb(emu: &mut emu::Emu) {
    let ldr = init_ldr(emu);
    let params_addr = init_arguments(emu);

    let peb_addr = emu
        .maps
        .lib64_alloc(PEB64::size() as u64)
        .expect("cannot alloc the PEB64");
    let peb_map = emu
        .maps
        .create_map(
            "peb",
            peb_addr,
            PEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create peb map");
    // Create KuserSharedData map

    let peb = PEB64::new(0, ldr, params_addr);
    peb.save(peb_map);
    emu.maps.write_byte(peb_addr + 2, 0); // not being_debugged

    let teb_addr = emu
        .maps
        .lib64_alloc(TEB64::size() as u64)
        .expect("cannot alloc the TEB64");
    let teb_map = emu
        .maps
        .create_map(
            "teb",
            teb_addr,
            TEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create teb map");
    let teb = TEB64::new(peb_addr);
    teb.save(teb_map);

    ensure_teb_activation_context_stack(emu);
    ensure_peb_system_dependent_07(emu);
}

/// Allocate and initialize a minimal Windows `_HEAP` (x64) structure so that
/// ntdll code that dereferences `PEB.ProcessHeap` internal pointers doesn't fault.
/// All LIST_ENTRY fields are self-referencing (empty lists), signatures are correct,
/// and back-pointers reference the heap itself.
fn init_process_heap(emu: &mut emu::Emu) -> u64 {
    let heap_sz: u64 = 0x1000;
    let h = emu
        .maps
        .lib64_alloc(heap_sz)
        .expect("cannot alloc fake ProcessHeap");
    let _heap_map = emu
        .maps
        .create_map("process_heap", h, heap_sz, Permission::READ_WRITE)
        .expect("cannot create process_heap map");
    emu.maps.memset(h, 0, heap_sz as usize);

    // Helper: write a self-referencing LIST_ENTRY (Flink=Blink=&self).
    let self_list = |maps: &mut crate::maps::Maps, addr: u64| {
        maps.write_qword(addr, addr);       // Flink
        maps.write_qword(addr + 8, addr);   // Blink
    };

    // _HEAP_SEGMENT embedded at +0x000
    emu.maps.write_dword(h + 0x010, 0xFEED_FEED); // SegmentSignature
    self_list(&mut emu.maps, h + 0x018);           // SegmentListEntry
    emu.maps.write_qword(h + 0x028, h);            // Heap (back-pointer)
    emu.maps.write_qword(h + 0x030, h);            // BaseAddress
    emu.maps.write_dword(h + 0x038, (heap_sz >> 12) as u32); // NumberOfPages
    emu.maps.write_qword(h + 0x040, h + 0x200);    // FirstEntry (past headers)
    emu.maps.write_qword(h + 0x048, h + heap_sz);  // LastValidEntry
    self_list(&mut emu.maps, h + 0x060);           // UCRSegmentList

    // _HEAP proper fields
    emu.maps.write_dword(h + 0x070, 0x0000_0002);  // Flags (HEAP_GROWABLE)
    emu.maps.write_dword(h + 0x098, 0xEEFF_EEFF);  // Signature
    emu.maps.write_qword(h + 0x0C8, 0x7FFF_EFFF_FFFF);  // MaximumAllocationSize

    self_list(&mut emu.maps, h + 0x0E8);           // UCRList
    self_list(&mut emu.maps, h + 0x108);           // VirtualAllocdBlocks
    self_list(&mut emu.maps, h + 0x118);           // SegmentList
    self_list(&mut emu.maps, h + 0x148);           // FreeLists

    // UCRIndex (+0x138) — must not be NULL; point at a small valid stub
    // containing an empty list (Flink=Blink=self).
    let ucr_stub = h + 0x300;
    self_list(&mut emu.maps, ucr_stub);
    emu.maps.write_qword(h + 0x138, ucr_stub);

    // FrontEndHeapMaximumIndex +0x1B0 = 0 (no frontend buckets)
    emu.maps.write_word(h + 0x1B0, 0);

    // Heap lock (RTL_CRITICAL_SECTION-like) at +0x400 inside the heap page.
    // ntdll reads [heap+0x158] or [heap+0x160] as a lock pointer and performs
    // `lock btr dword [ptr+8], 0` to acquire it.
    let lock_addr = h + 0x400;
    emu.maps.write_dword(lock_addr + 0x08, 0xFFFF_FFFF); // LockCount = -1 (unlocked)
    emu.maps.write_qword(h + 0x158, lock_addr); // LockVariable
    emu.maps.write_qword(h + 0x160, lock_addr); // (some versions use this offset)

    h
}

/// Minimal PEB/TEB for `--init` / `ssdt_use_ldr_initialize_thunk`: no full `PEB64` defaults (heap,
/// NLS pointers, …). Installs `PebLdrData64` + placeholder module list, **`PEB.ProcessParameters`**
/// at **0x20** (`RtlUserProcessParameters64`), and **`PEB.Ldr`** at **0x18**. Ntdll's loader reads
/// `[PEB+20h]` (e.g. after `NtQuerySystemInformation`); leaving it null faults on `[rax+8]`.
pub fn init_peb_teb_empty(emu: &mut emu::Emu) {
    let ldr_addr = init_ldr(emu);
    let params_addr = init_arguments(emu);

    let peb_addr = emu
        .maps
        .lib64_alloc(PEB64::size() as u64)
        .expect("cannot alloc the PEB64");
    let _peb_map = emu
        .maps
        .create_map(
            "peb",
            peb_addr,
            PEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create peb map");
    let heap_addr = init_process_heap(emu);
    // Keep the bootstrap path minimal, but do not leave the rest of the PEB as all-zero:
    // real loader code reads several auxiliary fields (`+0x68`, `+0x88`, `+0xA0`, ...).
    // Start from the regular PEB defaults and then override the parts that must point to our
    // emulated bootstrap state.
    let peb = PEB64::new(0, ldr_addr, params_addr);
    peb.save(emu.maps.get_mem_mut("peb"));
    emu.maps.write_byte(peb_addr + 2, 0); // BeingDebugged = FALSE
    emu.maps.write_qword(peb_addr + 0x30, heap_addr); // ProcessHeap

    let teb_addr = emu
        .maps
        .lib64_alloc(TEB64::size() as u64)
        .expect("cannot alloc the TEB64");
    let _teb_map = emu
        .maps
        .create_map(
            "teb",
            teb_addr,
            TEB64::size() as u64,
            Permission::READ_WRITE,
        )
        .expect("cannot create teb map");
    let teb = TEB64::new(peb_addr);
    teb.save(emu.maps.get_mem_mut("teb"));

    ensure_teb_activation_context_stack(emu);
    ensure_peb_system_dependent_06(emu);
    ensure_peb_system_dependent_07(emu);
    ensure_peb_nls_tables(emu);
}

pub fn update_peb_image_base(emu: &mut emu::Emu, base: u64) {
    let peb = emu.maps.get_mem("peb");
    let peb_base = peb.get_base();
    emu.maps.write_qword(peb_base + 0x10, base);
}

#[derive(Debug)]
pub struct Flink {
    flink_addr: u64,
    pub mod_base: u64,
    pub mod_name: String,
    pub pe_hdr: u64,

    pub export_table_rva: u64,
    /// Size of the IMAGE_DIRECTORY_ENTRY_EXPORT region (used to detect forwarded exports).
    pub export_dir_size: u64,
    pub export_table: u64,
    pub num_of_funcs: u64,
    pub func_name_tbl_rva: u64,
    pub func_name_tbl: u64,
}

impl Flink {
    pub fn new(emu: &mut emu::Emu) -> Flink {
        let peb = emu.maps.get_mem("peb");
        let peb_base = peb.get_base();
        let ldr = peb.read_qword(peb_base + 0x18); // peb->ldr
        let flink = emu
            .maps
            .read_qword(ldr + 0x10)
            .expect("peb64::new() error reading flink");

        Flink {
            flink_addr: flink,
            mod_base: 0,
            mod_name: String::new(),
            pe_hdr: 0,
            export_table_rva: 0,
            export_dir_size: 0,
            export_table: 0,
            num_of_funcs: 0,
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

    pub fn load(&mut self, emu: &mut emu::Emu) {
        self.get_mod_base(emu);
        self.get_mod_name(emu);
        self.get_pe_hdr(emu);
        self.get_export_table(emu);
    }

    pub fn get_mod_base(&mut self, emu: &mut emu::Emu) {
        self.mod_base = emu
            .maps
            .read_qword(self.flink_addr + 0x30)
            .expect("error reading mod_addr");
        if self.mod_base == 0 {
            // During early loader bootstrap (e.g. SSDT + `LdrInitializeThunk`), the list can contain
            // placeholder entries with base==0. Treat as "no module" and let callers skip it.
            if emu.cfg.verbose >= 2 {
                log::trace!("peb64: LDR entry has modbase=0 at 0x{:x}", self.flink_addr);
            }
        }
    }

    pub fn set_mod_base(&mut self, base: u64, emu: &mut emu::Emu) {
        self.mod_base = base;
        emu.maps.write_qword(self.flink_addr + 0x30, base);
    }

    pub fn get_mod_name(&mut self, emu: &mut emu::Emu) {
        let mod_name_ptr = emu
            .maps
            .read_qword(self.flink_addr + 0x60)
            .expect("error reading mod_name_ptr");
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

    pub fn get_export_table(&mut self, emu: &mut emu::Emu) {
        if self.pe_hdr == 0 {
            return;
        }

        //log::trace!("mod_base 0x{:x} pe_hdr 0x{:x}", self.mod_base, self.pe_hdr);

        self.export_table_rva = match emu.maps.read_dword(self.mod_base + self.pe_hdr + 0x88) {
            Some(rva) => rva as u64,
            None => 0,
        };

        self.export_dir_size = emu
            .maps
            .read_dword(self.mod_base + self.pe_hdr + 0x8c)
            .unwrap_or(0) as u64;

        if self.export_table_rva == 0 {
            return;
        }

        self.export_table = self.export_table_rva + self.mod_base;

        ////////
        /*
        emu.maps.print_maps();
        log::trace!("rva: 0x{:x} = 0x{:x} + 0x{:x} + 0x88 -> 0x{:x}",
            self.mod_base+self.pe_hdr+0x88,
            self.mod_base,
            self.pe_hdr,
            self.export_table_rva);
        log::trace!("export_table: 0x{:x} = 0x{:x} + 0x{:x}",
            self.export_table,
            self.mod_base,
            self.export_table_rva);
        log::trace!("num_of_funcs [0x{:x} + 0x18] = [0x{:x}]",
            self.export_table,
            self.export_table+0x18);
        */

        self.num_of_funcs = emu
            .maps
            .read_dword(self.export_table + 0x18)
            .expect("error reading the num_of_funcs") as u64;
        self.func_name_tbl_rva = emu
            .maps
            .read_dword(self.export_table + 0x20)
            .expect(" error reading func_name_tbl_rva") as u64;
        self.func_name_tbl = self.func_name_tbl_rva + self.mod_base;
    }

    pub fn get_function_ordinal(&self, emu: &mut emu::Emu, function_id: u64) -> OrdinalTable {
        self.get_function_ordinal_depth(emu, function_id, 0)
    }

    /// `forward_depth` limits chained PE export forwarders (`DLL.Symbol`).
    pub fn get_function_ordinal_depth(
        &self,
        emu: &mut emu::Emu,
        function_id: u64,
        forward_depth: u32,
    ) -> OrdinalTable {
        let mut ordinal = OrdinalTable::new();
        let func_name_rva = emu
            .maps
            .read_dword(self.func_name_tbl + function_id * 4)
            .expect("error reading func_rva") as u64;
        ordinal.func_name = emu.maps.read_string(func_name_rva + self.mod_base);
        ordinal.ordinal_tbl_rva = emu
            .maps
            .read_dword(self.export_table + 0x24)
            .expect("error reading ordinal_tbl_rva") as u64;
        ordinal.ordinal_tbl = ordinal.ordinal_tbl_rva + self.mod_base;
        ordinal.ordinal = emu
            .maps
            .read_word(ordinal.ordinal_tbl + 2 * function_id)
            .expect("error reading ordinal") as u64;
        ordinal.func_addr_tbl_rva = emu
            .maps
            .read_dword(self.export_table + 0x1c)
            .expect("error reading func_addr_tbl_rva") as u64;
        ordinal.func_addr_tbl = ordinal.func_addr_tbl_rva + self.mod_base;
        ordinal.func_rva = emu
            .maps
            .read_dword(ordinal.func_addr_tbl + 4 * ordinal.ordinal)
            .expect("error reading func_rva") as u64;

        // Forwarded export: RVA falls inside the export directory → ASCII "TARGETDLL.SymbolName".
        if self.export_dir_size > 0
            && ordinal.func_rva >= self.export_table_rva
            && ordinal.func_rva < self.export_table_rva.saturating_add(self.export_dir_size)
        {
            let forwarder = emu.maps.read_string(self.mod_base + ordinal.func_rva);
            let resolved = crate::winapi::winapi64::kernel32::resolve_forwarded_export_string_depth(
                emu,
                &forwarder,
                forward_depth.saturating_add(1),
            );
            if resolved != 0 {
                ordinal.func_va = resolved;
            } else {
                ordinal.func_va = ordinal.func_rva + self.mod_base;
            }
        } else {
            ordinal.func_va = ordinal.func_rva + self.mod_base;
        }

        /*
        println!("Function Name RVA: 0x{:x}", func_name_rva);
        println!("Function Name: {}", ordinal.func_name);
        println!("Ordinal Table RVA: 0x{:x}", ordinal.ordinal_tbl_rva);
        println!("Ordinal: {}", ordinal.ordinal);
        println!("Function RVA: 0x{:x} + base: 0x{:x}", ordinal.func_rva, self.mod_base);
        println!("Function VA: 0x{:x}", ordinal.func_va);
        println!("--------------------------------");
        */

        ordinal
    }

    pub fn get_next_flink(&self, emu: &mut emu::Emu) -> u64 {
        return emu
            .maps
            .read_qword(self.flink_addr)
            .expect("error reading next flink");
    }

    pub fn get_prev_flink(&self, emu: &mut emu::Emu) -> u64 {
        return emu
            .maps
            .read_qword(self.flink_addr + 8)
            .expect("error reading prev flink");
    }

    pub fn next(&mut self, emu: &mut emu::Emu) {
        self.flink_addr = self.get_next_flink(emu);
        self.load(emu);
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
    loop {
        //log::trace!("{} == {}", libname2, flink.mod_name);

        if libname.to_string().to_lowercase() == flink.mod_name.to_string().to_lowercase()
            || libname2 == flink.mod_name.to_string().to_lowercase()
        {
            return Some(flink.mod_base);
        }
        flink.next(emu);

        if flink.get_ptr() == first_flink {
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
        if flink.get_ptr() == first_flink {
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
    //emu.maps.write_qword(prev_flink, next_flink);
    emu.maps.write_qword(prev_flink, 0);

    // next blink
    log::trace!("next_flink: 0x{:x}", next_flink);
    emu.maps.write_qword(next_flink + 4, prev_flink);

    show_linked_modules(emu);
}

pub fn dynamic_link_module(base: u64, pe_off: u32, libname: &str, emu: &mut emu::Emu) {
    /*
     * LoadLibary* family triggers this.
     */
    //log::trace!("************ dynamic_link_module {}", libname);
    let mut last_flink: u64;
    let mut flink = Flink::new(emu);
    flink.load(emu);
    let first_flink = flink.get_ptr();

    // get last element
    loop {
        //last_flink = flink.get_ptr();
        flink.next(emu);
        if flink.get_next_flink(emu) == first_flink {
            break;
        }
    }
    let next_flink: u64 = flink.get_ptr();

    //log::trace!("last: {} {:x}", flink.mod_name, next_flink);

    //let space_addr = create_ldr_entry(emu, base, pe_off, libname, last_flink, first_flink);
    let space_addr = create_ldr_entry(
        emu,
        base,
        pe_off.into(),
        libname,
        first_flink,
        next_flink, /*first_flink*/
    );
    //TODO: pe_off is entry point

    // point previous flink to this ldr
    //let repl1 = emu.maps.read_qword(next_flink).unwrap();
    emu.maps.write_qword(next_flink, space_addr); // in_load_order_links.flink
    emu.maps.write_qword(next_flink + 0x10, space_addr + 0x10); // in_memory_order_links.flink
    emu.maps.write_qword(next_flink + 0x20, space_addr + 0x20); // in_initialization_order_links.flink

    // blink of first flink will point to last created
    emu.maps.write_qword(first_flink + 8, space_addr); // in_load_order_links.blink
    emu.maps
        .write_qword(first_flink + 0x10 + 8, space_addr + 0x10); // in_memory_order_links.blink
    emu.maps
        .write_qword(first_flink + 0x20 + 8, space_addr + 0x20); // in_initialization_order_links.blink

    //show_linked_modules(emu);
}

pub fn create_ldr_entry(
    emu: &mut emu::Emu,
    base: u64,
    entry_point: u64,
    libname: &str,
    next_flink: u64,
    prev_flink: u64,
) -> u64 {
    let base_addr;

    // make space for ldr
    let sz = LdrDataTableEntry64::size() + 0x40 + (1024 * 2);
    let space_addr = emu
        .maps
        .alloc(sz)
        .expect("cannot alloc few bytes to put the LDR for LoadLibraryA");
    let mut lib = format!("{}.ldr", libname);
    if emu.maps.exists_mapname(&lib) {
        use std::sync::atomic::{AtomicU32, Ordering};
        static LDR_SEQ: AtomicU32 = AtomicU32::new(0);
        lib = format!("{}.ldr.{}", libname, LDR_SEQ.fetch_add(1, Ordering::Relaxed));
    }
    let mut image_sz = 0;
    if base > 0 {
        let pe_hdr = emu.maps.read_dword(base + 0x3c).unwrap() as u64;
        image_sz = emu.maps.read_dword(base + pe_hdr + 0x50).unwrap();
        base_addr = base;
    } else {
        base_addr = space_addr
    }
    let mem = emu
        .maps
        .create_map(lib.as_str(), space_addr, sz, Permission::READ_WRITE)
        .expect("cannot create ldr entry map");
    mem.write_byte(space_addr + sz - 1, 0x61);

    //let full_libname = "\"C:\\Windows\\System32\\".to_string() + &libname.to_string() + "\"\x00";
    let full_libname = "C:\\Windows\\System32\\".to_string() + libname;

    let mut ldr = LdrDataTableEntry64::new();
    if next_flink != 0 {
        ldr.in_load_order_links.flink = next_flink;
        ldr.in_load_order_links.blink = prev_flink;
        ldr.in_memory_order_links.flink = next_flink + 0x10;
        ldr.in_memory_order_links.blink = prev_flink + 0x10;
        ldr.in_initialization_order_links.flink = next_flink + 0x20;
        ldr.in_initialization_order_links.blink = prev_flink + 0x20;
        ldr.hash_links.flink = next_flink + 0x7f;
        ldr.hash_links.blink = prev_flink + 0x7f;
    } else {
        ldr.in_load_order_links.flink = space_addr;
        ldr.in_load_order_links.blink = space_addr;
        ldr.in_memory_order_links.flink = space_addr + 0x10;
        ldr.in_memory_order_links.blink = space_addr + 0x10;
        ldr.in_initialization_order_links.flink = space_addr + 0x20;
        ldr.in_initialization_order_links.blink = space_addr + 0x20;
        ldr.hash_links.flink = space_addr + 0x7f;
        ldr.hash_links.blink = space_addr + 0x7f;
    }
    ldr.dll_base = base_addr;
    ldr.entry_point = entry_point;
    ldr.size_of_image = image_sz;
    ldr.full_dll_name.length = full_libname.len() as u16 * 2;
    ldr.full_dll_name.maximum_length = full_libname.len() as u16 * 2 + 4;
    ldr.full_dll_name.buffer = space_addr + LdrDataTableEntry64::size();
    ldr.base_dll_name.length = libname.len() as u16 * 2;
    ldr.base_dll_name.maximum_length = libname.len() as u16 * 2 + 2;
    ldr.base_dll_name.buffer =
        space_addr + LdrDataTableEntry64::size() + full_libname.len() as u64 * 2 + 10;
    ldr.flags = 0;
    ldr.load_count = 0;
    ldr.tls_index = 0;
    ldr.hash_links.flink = next_flink;
    ldr.hash_links.blink = prev_flink;
    mem.write_wide_string(
        space_addr + LdrDataTableEntry64::size(),
        &(full_libname.clone() + "\x00\x00"),
    );
    mem.write_wide_string(
        space_addr + LdrDataTableEntry64::size() + full_libname.len() as u64 * 2 + 10,
        &(libname.to_string() + "\x00"),
    );
    ldr.save(space_addr, &mut emu.maps);

    // http://terminus.rewolf.pl/terminus/structures/ntdll/_LDR_DATA_TABLE_ENTRY_x64.html

    space_addr
}

fn ldr_hash_bucket_index(libname: &str) -> u64 {
    let mut hash: u32 = 0;
    for ch in libname.encode_utf16() {
        let folded = if ch >= b'a' as u16 && ch <= b'z' as u16 {
            ch - 0x20
        } else {
            ch
        };
        hash = hash.wrapping_mul(0x1003f).wrapping_add(folded as u32);
    }
    (hash & 0x1f) as u64
}

fn rebuild_ldr_hash_table(emu: &mut emu::Emu, modules: &[ModInfo], entries: &[u64]) {
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let table = ntdll_map.get_base() + NTDLL_LDRP_HASH_TABLE_RVA64;

    for i in 0..NTDLL_LDRP_HASH_BUCKETS64 {
        let head = table + i * 0x10;
        emu.maps.write_qword(head, head);
        emu.maps.write_qword(head + 8, head);
    }

    for (module, entry) in modules.iter().zip(entries.iter()) {
        let bucket = ldr_hash_bucket_index(&module.name);
        let head = table + bucket * 0x10;
        let hash_links = *entry + LDR_HASH_LINKS_OFFSET64;
        let tail = emu.maps.read_qword(head + 8).unwrap_or(head);

        emu.maps.write_qword(hash_links, head);
        emu.maps.write_qword(hash_links + 8, tail);
        emu.maps.write_qword(tail, hash_links);
        emu.maps.write_qword(head + 8, hash_links);
    }
}

fn ensure_ntdll_loader_globals(emu: &mut emu::Emu) {
    let Some(ntdll_map) = emu.maps.get_map_by_name("ntdll.pe") else {
        return;
    };
    let base = ntdll_map.get_base();

    let list_2680 = base + NTDLL_LDRP_GLOBAL_2680_RVA64;
    emu.maps.write_qword(list_2680, list_2680);
    emu.maps.write_qword(list_2680 + 8, list_2680);

    let state_26c0 = base + NTDLL_LDRP_GLOBAL_26C0_RVA64;
    emu.maps.write_qword(state_26c0, u64::MAX);

    let list_26f0 = base + NTDLL_LDRP_GLOBAL_26F0_RVA64;
    emu.maps.write_qword(list_26f0, list_26f0);
    emu.maps.write_qword(list_26f0 + 8, list_26f0);
}

struct ModInfo {
    name: String,
    base: u64,
}

/// Rebuild the PEB_LDR_DATA lists after ntdll's `LdrInitializeThunk` has run.
///
/// LdrInitializeThunk reinitializes the Ldr lists from scratch.  If it fails
/// mid-way the lists are left empty (head.Flink == head).  This function
/// recreates entries for every PE image currently mapped and links them into
/// the three order lists so that user-mode code walking
/// `InMemoryOrderModuleList` finds valid entries.
pub fn rebuild_ldr_lists(emu: &mut emu::Emu) {
    ensure_peb_system_dependent_07(emu);
    let peb_addr = emu.maps.get_mem("peb").get_base();
    let ldr_addr = emu.maps.read_qword(peb_addr + 0x18).unwrap_or(0);
    if ldr_addr == 0 {
        return;
    }

    // Collect all ".pe" maps: (display_name, base_address)
    let exe_base = emu.base;
    let exe_name = emu.cfg.exe_name.clone();

    let pe_names: Vec<String> = emu
        .maps
        .name_map
        .keys()
        .filter(|n| n.ends_with(".pe"))
        .cloned()
        .collect();

    // Exe always first
    let mut modules: Vec<ModInfo> = vec![ModInfo {
        name: exe_name,
        base: exe_base,
    }];

    // Then ntdll (must be second for the standard module order)
    for map_name in &pe_names {
        let stem = map_name.trim_end_matches(".pe");
        if stem.eq_ignore_ascii_case("ntdll") {
            if let Some(m) = emu.maps.get_map_by_name(map_name) {
                modules.push(ModInfo {
                    name: "ntdll.dll".into(),
                    base: m.get_base(),
                });
            }
        }
    }

    // Then kernel32
    for map_name in &pe_names {
        let stem = map_name.trim_end_matches(".pe");
        if stem.eq_ignore_ascii_case("kernel32") {
            if let Some(m) = emu.maps.get_map_by_name(map_name) {
                modules.push(ModInfo {
                    name: "kernel32.dll".into(),
                    base: m.get_base(),
                });
            }
        }
    }

    // Remaining DLLs (skip exe, ntdll, kernel32 already added)
    for map_name in &pe_names {
        let stem = map_name.trim_end_matches(".pe");
        let sl = stem.to_lowercase();
        if sl == "ntdll" || sl == "kernel32" {
            continue;
        }
        // Skip the exe map (its stem matches the exe_name minus extension)
        let exe_stem = emu
            .cfg
            .filename
            .split('/')
            .last()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("");
        if stem.eq_ignore_ascii_case(exe_stem) {
            continue;
        }
        if let Some(m) = emu.maps.get_map_by_name(map_name) {
            modules.push(ModInfo {
                name: format!("{}.dll", stem),
                base: m.get_base(),
            });
        }
    }

    if modules.is_empty() {
        return;
    }

    // Create LDR entries for each module (all self-linked initially)
    let mut entries: Vec<u64> = Vec::new();
    for m in &modules {
        let entry_point = if m.base > 0 {
            let pe_hdr = emu.maps.read_dword(m.base + 0x3c).unwrap_or(0) as u64;
            if pe_hdr > 0 {
                let ep_rva = emu.maps.read_dword(m.base + pe_hdr + 0x28).unwrap_or(0) as u64;
                m.base + ep_rva
            } else {
                0
            }
        } else {
            0
        };
        let addr = create_ldr_entry(emu, m.base, entry_point, &m.name, 0, 0);
        entries.push(addr);
    }

    // Link entries in a circular chain (not through the Ldr head)
    let n = entries.len();
    for i in 0..n {
        let next = entries[(i + 1) % n];
        let prev = entries[(i + n - 1) % n];
        // InLoadOrderLinks
        emu.maps.write_qword(entries[i], next);
        emu.maps.write_qword(entries[i] + 8, prev);
        // InMemoryOrderLinks
        emu.maps.write_qword(entries[i] + 0x10, next + 0x10);
        emu.maps.write_qword(entries[i] + 0x18, prev + 0x10);
        // InInitializationOrderLinks
        emu.maps.write_qword(entries[i] + 0x20, next + 0x20);
        emu.maps.write_qword(entries[i] + 0x28, prev + 0x20);
    }

    // Point Ldr list heads to the first/last entries
    let first = entries[0];
    let last = entries[n - 1];
    emu.maps.write_qword(ldr_addr + 0x10, first); // InLoadOrder.Flink
    emu.maps.write_qword(ldr_addr + 0x18, last); // InLoadOrder.Blink
    emu.maps.write_qword(ldr_addr + 0x20, first + 0x10); // InMemoryOrder.Flink
    emu.maps.write_qword(ldr_addr + 0x28, last + 0x10); // InMemoryOrder.Blink
    emu.maps.write_qword(ldr_addr + 0x30, first + 0x20); // InInitializationOrder.Flink
    emu.maps.write_qword(ldr_addr + 0x38, last + 0x20); // InInitializationOrder.Blink
    emu.maps.write_dword(ldr_addr + 4, 1); // Initialized = TRUE

    rebuild_ldr_hash_table(emu, &modules, &entries);
    ensure_ntdll_loader_globals(emu);

    // Also update PEB.ImageBaseAddress
    if exe_base != 0 {
        emu.maps.write_qword(peb_addr + 0x10, exe_base);
    }

    log::trace!(
        "rebuild_ldr_lists: rebuilt with {} modules",
        modules.len()
    );
}
