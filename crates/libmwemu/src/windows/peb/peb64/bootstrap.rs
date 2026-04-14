use crate::emu;
use crate::maps::mem64::Permission;
use crate::windows::peb::peb64::ldr::create_ldr_entry;
use crate::windows::structures::PebLdrData64;
use crate::windows::structures::PEB64;
use crate::windows::structures::RtlUserProcessParameters64;
use crate::windows::structures::TEB64;

/// `PEB+0x90` (`system_dependent_07`): ntdll reads this as a pointer in loader / API-set helpers.
/// Real CSRSS/kernel fills it; under emulation `LdrInitializeThunk` (and related code) often leaves
/// this NULL, which makes loader code fault. If the slot is NULL or points at unmapped memory,
/// install or reuse a dedicated zero-filled page and write its base to `PEB+0x90`.
pub fn ensure_peb_system_dependent_07(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
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

/// `TEB+0x2C8` points at the thread activation-context stack.
fn ensure_teb_activation_context_stack(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
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

/// `PEB+0x68` (`system_dependent_06`) is consumed by loader-side helpers on the real path.
fn ensure_peb_system_dependent_06(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
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

/// Map a minimal NLS data page for the three code-page table pointers stored in the PEB.
pub fn ensure_peb_nls_tables(emu: &mut emu::Emu) {
    if !emu.cfg.arch.is_64bits() {
        return;
    }
    let peb_base = match emu.maps.get_map_by_name("peb") {
        Some(m) => m.get_base(),
        None => return,
    };

    const NLS_SZ: u64 = 0x1000;
    let slots: &[(&str, u64)] = &[
        ("peb_nls_ansi", peb_base + 0xA0),
        ("peb_nls_oem", peb_base + 0xA8),
        ("peb_nls_unicode", peb_base + 0xB0),
    ];
    for (name, slot_addr) in slots {
        let cur = emu.maps.read_qword(*slot_addr).unwrap_or(0);
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
    let peb = PEB64::new(0, ldr, params_addr);
    peb.save(peb_map);
    emu.maps.write_byte(peb_addr + 2, 0);

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

/// Allocate and initialize a minimal Windows `_HEAP` (x64) structure.
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

    let self_list = |maps: &mut crate::maps::Maps, addr: u64| {
        maps.write_qword(addr, addr);
        maps.write_qword(addr + 8, addr);
    };

    emu.maps.write_dword(h + 0x010, 0xFEED_FEED);
    self_list(&mut emu.maps, h + 0x018);
    emu.maps.write_qword(h + 0x028, h);
    emu.maps.write_qword(h + 0x030, h);
    emu.maps.write_dword(h + 0x038, (heap_sz >> 12) as u32);
    emu.maps.write_qword(h + 0x040, h + 0x200);
    emu.maps.write_qword(h + 0x048, h + heap_sz);
    self_list(&mut emu.maps, h + 0x060);

    emu.maps.write_dword(h + 0x070, 0x0000_0002);
    emu.maps.write_dword(h + 0x098, 0xEEFF_EEFF);
    emu.maps.write_qword(h + 0x0C8, 0x7FFF_EFFF_FFFF);

    self_list(&mut emu.maps, h + 0x0E8);
    self_list(&mut emu.maps, h + 0x108);
    self_list(&mut emu.maps, h + 0x118);
    self_list(&mut emu.maps, h + 0x148);

    let ucr_stub = h + 0x300;
    self_list(&mut emu.maps, ucr_stub);
    emu.maps.write_qword(h + 0x138, ucr_stub);

    emu.maps.write_word(h + 0x1B0, 0);

    let lock_addr = h + 0x400;
    emu.maps.write_dword(lock_addr + 0x08, 0xFFFF_FFFF);
    emu.maps.write_qword(h + 0x158, lock_addr);
    emu.maps.write_qword(h + 0x160, lock_addr);

    h
}

/// Minimal PEB/TEB for `--init` / `ssdt_use_ldr_initialize_thunk`.
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
    let peb = PEB64::new(0, ldr_addr, params_addr);
    peb.save(emu.maps.get_mem_mut("peb"));
    emu.maps.write_byte(peb_addr + 2, 0);
    emu.maps.write_qword(peb_addr + 0x30, heap_addr);

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
