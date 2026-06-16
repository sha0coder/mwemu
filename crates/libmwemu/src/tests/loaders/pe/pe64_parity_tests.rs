use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::traits::LiefPeReader;
use crate::loaders::pe::pe64::PE64;
use crate::loaders::pe::runtime_pe64::RuntimePe64;
use crate::serialization::pe64::SerializablePE64;
use crate::tests::helpers;
use lief::generic::Section as _;

fn load_fixture_pair(name: &str) -> (PE64, LiefPe) {
    let path = helpers::test_data_path(name);
    let legacy = PE64::load(&path);
    let lief = LiefPe::load(&path).expect("LIEF must load fixture");
    (legacy, lief)
}

fn run_parity_test(fixture: &str, test_name: &str, check: impl Fn(&PE64, &LiefPe) -> Vec<String>) {
    let (legacy, lief) = load_fixture_pair(fixture);
    let mismatches = check(&legacy, &lief);
    if !mismatches.is_empty() {
        for m in &mismatches {
            eprintln!("[{}] {}", test_name, m);
        }
        panic!(
            "[{}] {} mismatches for {}",
            test_name,
            mismatches.len(),
            fixture
        );
    }
}

#[test]
fn parity_headers_msgbox() {
    run_parity_test("exe64win_msgbox.bin", "HEADERS", compare_headers);
}

#[test]
fn parity_sections_msgbox() {
    run_parity_test("exe64win_msgbox.bin", "SECTIONS", compare_sections);
}

#[test]
fn parity_imports_msgbox() {
    run_parity_test("exe64win_msgbox.bin", "IMPORTS", compare_imports);
}

#[test]
fn parity_relocations_msgbox() {
    run_parity_test("exe64win_msgbox.bin", "RELOCATIONS", compare_relocations);
}

#[test]
fn parity_resources_msgbox() {
    run_parity_test("exe64win_msgbox.bin", "RESOURCES", compare_resources);
}

#[test]
fn parity_delay_loads_msgbox() {
    run_parity_test("exe64win_msgbox.bin", "DELAY_LOADS", compare_delay_loads);
}

#[test]
fn parity_headers_enigma() {
    run_parity_test("exe64win_enigma.bin", "HEADERS", compare_headers);
}

#[test]
fn parity_sections_enigma() {
    run_parity_test("exe64win_enigma.bin", "SECTIONS", compare_sections);
}

#[test]
fn parity_imports_enigma() {
    run_parity_test("exe64win_enigma.bin", "IMPORTS", compare_imports);
}

#[test]
fn parity_relocations_enigma() {
    run_parity_test("exe64win_enigma.bin", "RELOCATIONS", compare_relocations);
}

#[test]
fn parity_delay_loads_enigma() {
    run_parity_test("exe64win_enigma.bin", "DELAY_LOADS", compare_delay_loads);
}

#[test]
fn parity_serialization_roundtrip() {
    let path = match helpers::optional_test_data_path("exe64win_msgbox.bin") {
        Some(p) => p,
        None => return,
    };

    let pe = match RuntimePe64::load(&path) {
        Ok(pe) => pe,
        Err(_) => return, // LIEF unavailable for this fixture; skip.
    };
    let serialized: SerializablePE64 = (&pe).into();
    let deserialized: RuntimePe64 = serialized
        .try_into()
        .expect("LIEF must re-parse serialized bytes");

    assert_eq!(
        pe.image_base(),
        deserialized.image_base(),
        "image_base mismatch after roundtrip"
    );
    assert_eq!(
        pe.entry_point(),
        deserialized.entry_point(),
        "entry_point mismatch after roundtrip"
    );
    assert_eq!(
        pe.num_of_sections(),
        deserialized.num_of_sections(),
        "section count mismatch after roundtrip"
    );
}

#[test]
fn parity_serialization_lief_raw_bytes() {
    let path = match helpers::optional_test_data_path("exe64win_msgbox.bin") {
        Some(p) => p,
        None => return,
    };

    let pe = match RuntimePe64::load(&path) {
        Ok(pe) => pe,
        Err(_) => {
            eprintln!("skipping: LIEF backend not available for fixture");
            return;
        }
    };
    let pe = pe.into_inner();

    let raw = pe.mapped_file_data().to_vec();
    let filename = pe.file_path().to_string();
    let image_base = pe.image_base();
    let entry_point = pe.entry_point();
    let num_sections = pe.num_sections();

    drop(pe);

    let reloaded = match crate::loaders::pe::lief::LiefPe::load_from_raw(&filename, &raw) {
        Ok(pe) => pe,
        Err(e) => panic!("LIEF load_from_raw failed after dropping original: {}", e),
    };

    assert_eq!(
        reloaded.image_base(),
        image_base,
        "image_base mismatch after raw reload"
    );
    assert_eq!(
        reloaded.entry_point(),
        entry_point,
        "entry_point mismatch after raw reload"
    );
    assert_eq!(
        reloaded.num_sections(),
        num_sections,
        "section count mismatch after raw reload"
    );
}

#[test]
fn parity_serialization_backward_compat_lowercase() {
    use crate::serialization::pe64::SerializablePe64Backend;

    let yaml_legacy = "\"legacy\"";
    let parsed: SerializablePe64Backend =
        serde_yaml::from_str(yaml_legacy).expect("should deserialize lowercase 'legacy'");
    assert_eq!(parsed, SerializablePe64Backend::Legacy);

    let yaml_lief = "\"lief\"";
    let parsed: SerializablePe64Backend =
        serde_yaml::from_str(yaml_lief).expect("should deserialize lowercase 'lief'");
    assert_eq!(parsed, SerializablePe64Backend::Lief);
}

#[test]
fn parity_serialization_missing_backend_defaults_lief() {
    use crate::serialization::pe64::SerializablePe64Backend;

    // New default: LIEF is the only runtime backend; missing-backend
    // serialization records default to Lief.
    let default = SerializablePe64Backend::default();
    assert_eq!(default, SerializablePe64Backend::Lief);
}

fn compare_headers(legacy: &PE64, lief: &LiefPe) -> Vec<String> {
    let mut m = Vec::new();

    if legacy.opt.image_base != lief.image_base() {
        m.push(format!(
            "image_base: legacy=0x{:x} lief=0x{:x}",
            legacy.opt.image_base,
            lief.image_base()
        ));
    }
    if legacy.opt.address_of_entry_point as u64 != lief.entry_point() {
        m.push(format!(
            "entry_point: legacy=0x{:x} lief=0x{:x}",
            legacy.opt.address_of_entry_point,
            lief.entry_point()
        ));
    }
    if legacy.opt.size_of_image != lief.virtual_size() as u32 {
        m.push(format!(
            "size_of_image: legacy=0x{:x} lief=0x{:x}",
            legacy.opt.size_of_image,
            lief.virtual_size()
        ));
    }
    if legacy.opt.size_of_headers != lief.size_of_headers() {
        m.push(format!(
            "size_of_headers: legacy=0x{:x} lief=0x{:x}",
            legacy.opt.size_of_headers,
            lief.size_of_headers()
        ));
    }
    if legacy.opt.section_alignment != lief.section_alignment() {
        m.push(format!(
            "section_alignment: legacy=0x{:x} lief=0x{:x}",
            legacy.opt.section_alignment,
            lief.section_alignment()
        ));
    }
    if legacy.dos.e_lfanew != lief.e_lfanew() {
        m.push(format!(
            "e_lfanew: legacy=0x{:x} lief=0x{:x}",
            legacy.dos.e_lfanew,
            lief.e_lfanew()
        ));
    }

    m
}

fn compare_sections(legacy: &PE64, lief: &LiefPe) -> Vec<String> {
    let mut m = Vec::new();

    let legacy_count = legacy.num_of_sections();
    let lief_count = lief.num_sections() as usize;
    if legacy_count != lief_count {
        m.push(format!(
            "section count: legacy={} lief={}",
            legacy_count, lief_count
        ));
        return m;
    }

    for i in 0..legacy_count {
        let ls = &legacy.sect_hdr[i];
        let Some(lief_sect) = lief.get_section(i) else {
            m.push(format!("section {}: LIEF missing", i));
            continue;
        };

        let legacy_name = ls.get_name().trim().to_string();
        let lief_name = lief_sect.name();

        if legacy_name != lief_name {
            m.push(format!(
                "section {} name: legacy='{}' lief='{}'",
                i, legacy_name, lief_name
            ));
        }
        if ls.virtual_address != lief_sect.virtual_address() as u32 {
            m.push(format!(
                "section {} VA: legacy=0x{:x} lief=0x{:x}",
                i,
                ls.virtual_address,
                lief_sect.virtual_address()
            ));
        }
        if ls.virtual_size != lief_sect.virtual_size() {
            m.push(format!(
                "section {} virtual_size: legacy=0x{:x} lief=0x{:x}",
                i,
                ls.virtual_size,
                lief_sect.virtual_size()
            ));
        }
        if ls.size_of_raw_data != lief_sect.sizeof_raw_data() {
            m.push(format!(
                "section {} raw_size: legacy=0x{:x} lief=0x{:x}",
                i,
                ls.size_of_raw_data,
                lief_sect.sizeof_raw_data()
            ));
        }
        if ls.characteristics != lief_sect.characteristics().bits() as u32 {
            m.push(format!(
                "section {} characteristics: legacy=0x{:x} lief=0x{:x}",
                i,
                ls.characteristics,
                lief_sect.characteristics().bits()
            ));
        }

        let legacy_ptr = legacy.get_section_ptr(i);
        let lief_ptr = lief.get_section_ptr(i);
        let legacy_raw_len = ls.size_of_raw_data as usize;
        let lief_raw_len = lief_sect.sizeof_raw_data().min(lief_ptr.len() as u32) as usize;
        let legacy_hash = compute_hash(&legacy_ptr[..legacy_raw_len.min(legacy_ptr.len())]);
        let lief_hash = compute_hash(&lief_ptr[..lief_raw_len.min(lief_ptr.len())]);
        if legacy_hash != lief_hash {
            m.push(format!(
                "section {} raw_byte_hash: legacy={:016x} lief={:016x}",
                i, legacy_hash, lief_hash
            ));
        }
    }

    m
}

fn compare_imports(legacy: &PE64, lief: &LiefPe) -> Vec<String> {
    let mut m = Vec::new();

    let legacy_dlls: Vec<_> = legacy
        .image_import_descriptor
        .iter()
        .filter(|d| !d.name.is_empty())
        .collect();

    let lief_imports = match lief.get_imports() {
        Ok(imports) => imports,
        Err(e) => {
            m.push(format!("LIEF get_imports failed: {}", e));
            return m;
        }
    };

    if legacy_dlls.len() != lief_imports.len() {
        m.push(format!(
            "import DLL count: legacy={} lief={}",
            legacy_dlls.len(),
            lief_imports.len()
        ));
    }

    for ld in &legacy_dlls {
        if !lief_imports
            .iter()
            .any(|li| li.dll_name.eq_ignore_ascii_case(&ld.name))
        {
            m.push(format!("DLL '{}' in legacy but not in LIEF", ld.name));
        }
    }

    for li in &lief_imports {
        if !legacy_dlls
            .iter()
            .any(|ld| ld.name.eq_ignore_ascii_case(&li.dll_name))
        {
            m.push(format!("DLL '{}' in LIEF but not in legacy", li.dll_name));
            continue;
        }

        let matching_legacy = legacy_dlls
            .iter()
            .find(|ld| ld.name.eq_ignore_ascii_case(&li.dll_name))
            .unwrap();

        let thunk_names_rva = if matching_legacy.original_first_thunk != 0 {
            matching_legacy.original_first_thunk
        } else {
            matching_legacy.first_thunk
        };
        let mut off_name = PE64::vaddr_to_off(&legacy.sect_hdr, thunk_names_rva) as usize;
        let mut off_addr =
            PE64::vaddr_to_off(&legacy.sect_hdr, matching_legacy.first_thunk) as usize;

        let mut legacy_func_names: Vec<String> = Vec::new();
        loop {
            if legacy.raw.len() <= off_name + 8 || legacy.raw.len() <= off_addr + 8 {
                break;
            }
            let thunk_data = u64::from_le_bytes(
                legacy.raw[off_name..off_name + 8]
                    .try_into()
                    .unwrap_or([0; 8]),
            );
            if thunk_data == 0 {
                break;
            }
            let is_ordinal = (thunk_data & 0x80000000_00000000) != 0;
            if is_ordinal {
                let ordinal = (thunk_data & 0xFFFF) as u16;
                legacy_func_names.push(format!("#{}", ordinal));
            } else {
                let func_name_addr = (thunk_data & 0x7fff_ffff_ffff_ffff) as u32;
                let off2 = PE64::vaddr_to_off(&legacy.sect_hdr, func_name_addr) as usize;
                if off2 != 0 {
                    let func_name = PE64::read_string(&legacy.raw, off2 + 2);
                    if !func_name.is_empty() {
                        legacy_func_names.push(func_name);
                    }
                }
            }
            off_name += 8;
            off_addr += 8;
        }

        let lief_func_names: Vec<String> = li
            .functions
            .iter()
            .filter(|lf| lf.rva != 0)
            .map(|lf| {
                if lf.name.is_empty() {
                    format!("#{}", lf.ordinal.unwrap_or(0))
                } else {
                    lf.name.clone()
                }
            })
            .collect();

        if legacy_func_names.len() != lief_func_names.len() {
            m.push(format!(
                "DLL '{}' function count mismatch: legacy={} lief={}",
                li.dll_name,
                legacy_func_names.len(),
                lief_func_names.len()
            ));
        }

        for lfn in &legacy_func_names {
            if !lief_func_names
                .iter()
                .any(|lif| lif.eq_ignore_ascii_case(lfn))
            {
                m.push(format!(
                    "DLL '{}' function '{}' in legacy import but not in LIEF",
                    li.dll_name, lfn
                ));
            }
        }
        for lif in &lief_func_names {
            if !legacy_func_names
                .iter()
                .any(|lfn| lfn.eq_ignore_ascii_case(lif))
            {
                m.push(format!(
                    "DLL '{}' function '{}' in LIEF import but not in legacy",
                    li.dll_name, lif
                ));
            }
        }
    }

    m
}

fn compare_relocations(legacy: &PE64, lief: &LiefPe) -> Vec<String> {
    let mut m = Vec::new();

    let legacy_reloc_dir = &legacy.opt.data_directory[5];
    let lief_reloc_dir = lief.get_data_directory(5);

    match lief_reloc_dir {
        Some(ld) => {
            if legacy_reloc_dir.virtual_address != ld.rva() as u32 {
                m.push(format!(
                    "reloc dir RVA: legacy=0x{:x} lief=0x{:x}",
                    legacy_reloc_dir.virtual_address,
                    ld.rva()
                ));
            }
            if legacy_reloc_dir.size != ld.size() as u32 {
                m.push(format!(
                    "reloc dir size: legacy=0x{:x} lief=0x{:x}",
                    legacy_reloc_dir.size,
                    ld.size()
                ));
            }
        }
        None => {
            if legacy_reloc_dir.virtual_address != 0 {
                m.push("LIEF has no reloc dir but legacy does".to_string());
            }
        }
    }

    if legacy_reloc_dir.virtual_address == 0 || legacy_reloc_dir.size == 0 {
        return m;
    }

    let lief_relocs = match lief.get_relocations() {
        Ok(r) => r,
        Err(e) => {
            m.push(format!("LIEF get_relocations failed: {}", e));
            return m;
        }
    };

    let mut legacy_reloc_pairs: Vec<(u64, u16)> = Vec::new();
    let mut off = PE64::vaddr_to_off(&legacy.sect_hdr, legacy_reloc_dir.virtual_address) as usize;
    let end_off = off + legacy_reloc_dir.size as usize;

    while off < end_off && off + 8 <= legacy.raw.len() {
        let page_va = u32::from_le_bytes(legacy.raw[off..off + 4].try_into().unwrap_or([0; 4]));
        let block_sz =
            u32::from_le_bytes(legacy.raw[off + 4..off + 8].try_into().unwrap_or([0; 4]));
        if page_va == 0 && block_sz == 0 {
            break;
        }
        if block_sz < 8 {
            break;
        }
        let entries = (block_sz - 8) / 2;
        let mut entry_off = off + 8;
        for _ in 0..entries {
            if entry_off + 2 > legacy.raw.len() {
                break;
            }
            let entry = u16::from_le_bytes(
                legacy.raw[entry_off..entry_off + 2]
                    .try_into()
                    .unwrap_or([0; 2]),
            );
            let reloc_type = entry >> 12;
            let reloc_offset = entry & 0x0FFF;
            if reloc_type != 0 {
                legacy_reloc_pairs.push(((page_va + reloc_offset as u32) as u64, reloc_type));
            }
            entry_off += 2;
        }
        off += block_sz as usize;
    }

    let lief_reloc_pairs: Vec<(u64, u16)> =
        lief_relocs.iter().map(|r| (r.rva, r.reloc_type)).collect();

    if legacy_reloc_pairs.len() != lief_reloc_pairs.len() {
        m.push(format!(
            "reloc entry count: legacy={} lief={}",
            legacy_reloc_pairs.len(),
            lief_reloc_pairs.len()
        ));
    }

    let mut legacy_sorted = legacy_reloc_pairs.clone();
    let mut lief_sorted = lief_reloc_pairs.clone();
    legacy_sorted.sort_by_key(|(rva, t)| (*rva, *t));
    lief_sorted.sort_by_key(|(rva, t)| (*rva, *t));

    for &(lrva, ltype) in &legacy_sorted {
        if !lief_sorted.contains(&(lrva, ltype)) {
            m.push(format!(
                "reloc (RVA=0x{:x} type={}) in legacy but not in LIEF",
                lrva, ltype
            ));
            if m.len() > 10 {
                break;
            }
        }
    }
    for &(lrva, ltype) in &lief_sorted {
        if !legacy_sorted.contains(&(lrva, ltype)) {
            m.push(format!(
                "reloc (RVA=0x{:x} type={}) in LIEF but not in legacy",
                lrva, ltype
            ));
            if m.len() > 10 {
                break;
            }
        }
    }

    m
}

fn compare_resources(legacy: &PE64, lief: &LiefPe) -> Vec<String> {
    let mut m = Vec::new();

    let lief_entries = lief.get_resource_entries();
    let legacy_rsrc_dir = &legacy.opt.data_directory[2];

    if legacy_rsrc_dir.virtual_address == 0 && lief_entries.is_empty() {
        return m;
    }

    if legacy_rsrc_dir.virtual_address != 0 && lief_entries.is_empty() {
        m.push(format!(
            "legacy has resource dir (RVA=0x{:x}) but LIEF returned zero entries",
            legacy_rsrc_dir.virtual_address
        ));
    }

    if legacy_rsrc_dir.virtual_address == 0 && !lief_entries.is_empty() {
        m.push(format!(
            "legacy has no resource dir but LIEF returned {} entries",
            lief_entries.len()
        ));
    }

    for entry in &lief_entries {
        if entry.size > 0 && entry.data_rva == 0 {
            m.push(format!(
                "resource entry type={:?} name={:?} has size {} but data_rva is 0",
                entry.type_id, entry.name, entry.size
            ));
        }
    }

    if !lief_entries.is_empty() && legacy_rsrc_dir.virtual_address != 0 {
        let lief_resources = match lief.get_resources() {
            Ok(r) => r,
            Err(e) => {
                m.push(format!("LIEF get_resources failed: {}", e));
                return m;
            }
        };
        let mut total_lief_size: u64 = 0;
        for res in &lief_resources {
            total_lief_size += res.size;
        }
        if total_lief_size == 0 && !lief_entries.is_empty() {
            m.push(format!(
                "LIEF has {} resource entries but zero total content bytes",
                lief_entries.len()
            ));
        }
    }

    if !lief_entries.is_empty() && legacy_rsrc_dir.virtual_address != 0 {
        for le in &lief_entries {
            if le.size > 0 && le.data_rva == 0 {
                m.push(format!(
                    "resource entry type={:?} name={:?} has size {} but data_rva is 0",
                    le.type_id, le.name, le.size
                ));
            }
        }
    }

    m
}

fn compare_delay_loads(legacy: &PE64, lief: &LiefPe) -> Vec<String> {
    let mut m = Vec::new();

    let delay_dir = &legacy.opt.data_directory[13];
    let has_legacy_delay = delay_dir.virtual_address != 0 && delay_dir.size != 0;

    let lief_descriptors = lief.delay_load_descriptors();

    if !has_legacy_delay && !lief_descriptors.is_empty() {
        m.push(format!(
            "legacy has no delay-load dir but LIEF returned {} descriptors",
            lief_descriptors.len()
        ));
    }

    if has_legacy_delay && lief_descriptors.is_empty() {
        m.push("legacy has delay-load dir but LIEF returned zero descriptors".to_string());
        return m;
    }

    if !has_legacy_delay && lief_descriptors.is_empty() {
        return m;
    }

    let legacy_desc = legacy.delay_load_descriptors();

    if legacy_desc.len() != lief_descriptors.len() {
        m.push(format!(
            "delay-load descriptor count: legacy={} lief={}",
            legacy_desc.len(),
            lief_descriptors.len()
        ));
    }

    let count = legacy_desc.len().min(lief_descriptors.len());
    for i in 0..count {
        let ld = &lief_descriptors[i];
        if ld.dll_name_rva == 0 {
            m.push(format!("LIEF descriptor {} has dll_name_rva=0", i));
        }
        if ld.delay_iat == 0 {
            m.push(format!("LIEF descriptor {} has delay_iat=0", i));
        }
        if !ld.dll_name.is_empty() {
            let leg_dll_name = &legacy_desc[i].dll_name;
            if !leg_dll_name.is_empty() && ld.dll_name.to_lowercase() != leg_dll_name.to_lowercase()
            {
                m.push(format!(
                    "Descriptor {} dll_name mismatch: legacy='{}' lief='{}'",
                    i, leg_dll_name, ld.dll_name
                ));
            }
        }
        if ld.attributes != legacy_desc[i].attributes {
            m.push(format!(
                "Descriptor {} attributes mismatch: legacy=0x{:x} lief=0x{:x}",
                i, legacy_desc[i].attributes, ld.attributes
            ));
        }
        if ld.delay_iat != legacy_desc[i].delay_iat {
            m.push(format!(
                "Descriptor {} delay_iat mismatch: legacy=0x{:x} lief=0x{:x}",
                i, legacy_desc[i].delay_iat, ld.delay_iat
            ));
        }
        if ld.delay_int != legacy_desc[i].delay_int {
            m.push(format!(
                "Descriptor {} delay_int mismatch: legacy=0x{:x} lief=0x{:x}",
                i, legacy_desc[i].delay_int, ld.delay_int
            ));
        }
    }

    m
}

fn compute_hash(data: &[u8]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}
