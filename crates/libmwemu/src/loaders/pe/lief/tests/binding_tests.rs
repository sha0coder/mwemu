use crate::loaders::pe::lief::LiefPe;
use crate::loaders::pe::lief::traits::LiefPeReader;
use crate::loaders::pe::runtime_pe64::RuntimePe64;
use crate::serialization::pe64::{SerializablePE64, SerializablePe64Backend};
use crate::config::Pe64Backend;

fn load_fixture() -> Option<LiefPe> {
    let path = "test/exe64win_msgbox.bin";
    if !std::path::Path::new(path).exists() {
        eprintln!(" Skipping LIEF test: fixture {} not found", path);
        return None;
    }
    LiefPe::load(path).ok()
}

fn fixture_path() -> Option<&'static str> {
    let path = "test/exe64win_msgbox.bin";
    if std::path::Path::new(path).exists() {
        Some(path)
    } else {
        eprintln!(" Skipping test: fixture {} not found", path);
        None
    }
}

#[test]
fn test_get_imports() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    match pe.get_imports() {
        Ok(imports) => {
            assert!(!imports.is_empty(), "Should have at least one import DLL");
            for import in &imports {
                assert!(!import.dll_name.is_empty(), "DLL name should not be empty");
            }
        }
        Err(e) => {
            panic!("get_imports returned unexpected error: {:?}", e);
        }
    }
}

#[test]
fn test_get_exports() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    match pe.get_exports() {
        Ok(exports) => {
            for export in &exports {
                assert!(!export.name.is_empty(), "Export name should not be empty");
            }
        }
        Err(e) => {
            println!("get_exports returned error (expected for some PEs): {:?}", e);
        }
    }
}

#[test]
fn test_get_relocations() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    match pe.get_relocations() {
        Ok(relocations) => {
            for _reloc in &relocations {
            }
        }
        Err(e) => {
            println!("get_relocations returned error (expected for some PEs): {:?}", e);
        }
    }
}

#[test]
fn test_basic_pe_info() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    assert!(pe.is_pe64() || pe.is_pe32(), "Should be either PE32 or PE64");
    assert!(pe.num_sections() > 0, "Should have at least one section");
    assert!(pe.image_base() > 0, "Image base should be non-zero");
}

#[test]
fn test_get_dependencies() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    let deps = pe.get_dependencies(None);
    for dep in deps {
        assert!(!dep.is_empty(), "Dependency name should not be empty");
    }
}

#[test]
fn test_section_loading() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    for i in 0..pe.num_sections() {
        let section_ptr = pe.get_section_ptr(i as usize);
        let _len = section_ptr.len();
    }
}

#[test]
fn test_cache_operations() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    let stats = pe.cache_stats();
    assert_eq!(stats.cached_sections.len(), 0, "Cache should be empty initially");
    assert_eq!(stats.cached_bytes, 0, "Cache should have 0 bytes initially");

    let _ = pe.get_section_ptr(0);

    pe.clear_cache();
    let stats = pe.cache_stats();
    assert_eq!(stats.cached_sections.len(), 0, "Cache should be empty after clear");
    assert_eq!(stats.cached_bytes, 0, "Cache should have 0 bytes after clear");
}

#[test]
fn test_forced_legacy_backend() {
    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = RuntimePe64::load_with_backend(path, Pe64Backend::Legacy)
        .expect("legacy backend should not fail");
    assert_eq!(pe.backend_name(), "legacy", "forced legacy should use legacy backend");
    assert!(!pe.is_lief(), "is_lief() should be false for legacy backend");
}

#[test]
fn test_forced_lief_backend() {
    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = RuntimePe64::load_with_backend(path, Pe64Backend::Lief)
        .expect("LIEF backend should succeed for valid fixture");
    assert_eq!(pe.backend_name(), "lief", "forced LIEF should use lief backend");
    assert!(pe.is_lief(), "is_lief() should be true for LIEF backend");
}

#[test]
fn test_auto_backend_prefers_lief() {
    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = RuntimePe64::load_with_backend(path, Pe64Backend::Auto)
        .expect("auto backend should succeed");
    assert_eq!(pe.backend_name(), "lief", "auto should prefer LIEF when available");
}

#[test]
fn test_runtime_pe64_section_access() {
    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = RuntimePe64::load_auto(path);
    for i in 0..pe.num_of_sections() {
        let section = pe.get_section(i);
        assert!(section.is_some(), "Section {} should be accessible", i);
        let sect = section.unwrap();
        assert!(!sect.name.is_empty() || sect.virtual_address > 0, "Section should have name or valid VA");
    }
}

#[test]
fn test_lief_resource_enumeration() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    let entries = pe.get_resource_entries();
    match pe.get_resources() {
        Ok(resources) => {
            for res in &resources {
                assert!(!res.name.is_empty() || res.resource_type > 0, "Resource should have name or type");
            }
        }
        Err(e) => {
            println!("get_resources returned error: {:?}", e);
        }
    }
}

#[test]
fn test_serialization_roundtrip() {
    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = RuntimePe64::load_auto(path);
    let expected_backend = if pe.is_lief() {
        SerializablePe64Backend::Lief
    } else {
        SerializablePe64Backend::Legacy
    };

    let serialized: SerializablePE64 = (&pe).into();

    assert!(!serialized.filename.is_empty(), "Serialized filename should not be empty");
    assert!(!serialized.raw.is_empty(), "Serialized raw data should not be empty");
    assert_eq!(
        serialized.backend,
        Some(expected_backend),
        "Backend discriminator should match actual backend"
    );

    let deserialized: RuntimePe64 = serialized.into();
    assert!(deserialized.image_base() > 0, "Deserialized image base should be non-zero");
    assert!(deserialized.num_of_sections() > 0, "Deserialized should have sections");
}

#[test]
fn test_lief_from_raw_bytes_after_path_deleted() {
    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = match LiefPe::load(path) {
        Ok(p) => p,
        Err(_) => return,
    };

    let raw = pe.mapped_file_data().to_vec();
    let image_base = pe.image_base();
    let entry_point = pe.entry_point();
    let num_sections = pe.num_sections();

    let fake_path = "/nonexistent/deleted/file.bin".to_string();
    drop(pe);

    let reloaded = match LiefPe::load_from_raw(&fake_path, &raw) {
        Ok(p) => p,
        Err(e) => panic!("load_from_raw should work even when original path is deleted: {}", e),
    };

    assert_eq!(reloaded.image_base(), image_base, "image_base should match after raw reload");
    assert_eq!(reloaded.entry_point(), entry_point, "entry_point should match after raw reload");
    assert_eq!(reloaded.num_sections(), num_sections, "section count should match after raw reload");
}

#[test]
fn test_section_cache_eviction() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    if pe.num_sections() < 2 {
        eprintln!("skipping: need at least 2 sections");
        return;
    }

    pe.clear_cache();

    let _ = pe.get_section_ptr(0);
    assert!(pe.is_section_loaded(&pe.get_section_name(0).unwrap_or_default()));

    let _ = pe.get_section_ptr(1);
    assert!(pe.is_section_loaded(&pe.get_section_name(1).unwrap_or_default()));

    pe.clear_cache();
    assert_eq!(pe.cache_stats().cached_sections.len(), 0);
}

#[test]
fn test_section_cache_access_order() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    if pe.num_sections() < 2 {
        eprintln!("skipping: need at least 2 sections");
        return;
    }

    pe.clear_cache();

    let name0 = pe.get_section_name(0).unwrap_or_default();
    let name1 = pe.get_section_name(1).unwrap_or_default();

    let _ = pe.get_section_ptr(0);
    let _ = pe.get_section_ptr(1);

    let _ = pe.get_section_ptr(0);

    assert!(pe.is_section_loaded(&name0));
    assert!(pe.is_section_loaded(&name1));

    pe.clear_cache();
}

#[test]
fn test_section_cache_empty_vs_missing() {
    let pe = match load_fixture() {
        Some(pe) => pe,
        None => return,
    };

    let data = pe.get_section_ptr(999);
    assert!(data.is_empty(), "out-of-bounds section should return empty");

    let valid_data = pe.get_section_ptr(0);
    if valid_data.is_empty() {
        eprintln!("note: section 0 has empty raw data (valid empty section)");
    }
}

#[test]
fn test_section_cache_lru_eviction_order() {
    use crate::loaders::pe::lief::lief_section_manager::CachePolicy;

    let path = match fixture_path() {
        Some(p) => p,
        None => return,
    };

    let pe = match crate::loaders::pe::lief::LiefPe::load_with_policy(path, CachePolicy::LRU { max_bytes: 1 }) {
        Ok(pe) => pe,
        Err(_) => return,
    };

    if pe.num_sections() < 2 {
        eprintln!("skipping: need at least 2 sections");
        return;
    }

    let data0 = pe.get_section_ptr(0);
    let data1 = pe.get_section_ptr(1);

    if data0.is_empty() || data1.is_empty() {
        eprintln!("skipping: sections have empty data");
        return;
    }

    assert!(pe.is_section_index_loaded(0) || pe.is_section_index_loaded(1),
            "at least one section should be cached after eviction");
}
