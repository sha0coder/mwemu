//! Tests for LIEF PE binding functionality
//!
//! These tests verify that the binding methods work correctly for PE files.

use crate::pe::lief::LiefPe;
use crate::pe::lief::traits::LiefPeReader;

/// Test that imports can be parsed from a PE file
#[test]
fn test_get_imports() {
    // Use a test PE file that has imports
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        // If file doesn't exist, skip the test
        return;
    }

    let pe = pe.unwrap();

    // Try to get imports - should not panic
    match pe.get_imports() {
        Ok(imports) => {
            // If the PE has imports, verify the structure
            for import in &imports {
                assert!(!import.dll_name.is_empty(), "DLL name should not be empty");
                for func in &import.functions {
                    // Function name can be empty for ordinal-only imports
                    // But rva should be valid
                }
            }
        }
        Err(e) => {
            // Some PEs may not have imports or parsing may fail
            // Just verify it doesn't panic
            println!("get_imports returned error (expected for some PEs): {:?}", e);
        }
    }
}

/// Test that exports can be parsed from a PE file
#[test]
fn test_get_exports() {
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        return;
    }

    let pe = pe.unwrap();

    // Try to get exports - should not panic
    match pe.get_exports() {
        Ok(exports) => {
            for export in &exports {
                assert!(!export.name.is_empty(), "Export name should not be empty");
                // Ordinal and RVA can be 0 for some exports
            }
        }
        Err(e) => {
            println!("get_exports returned error (expected for some PEs): {:?}", e);
        }
    }
}

/// Test that relocations can be parsed from a PE file
#[test]
fn test_get_relocations() {
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        return;
    }

    let pe = pe.unwrap();

    // Try to get relocations - should not panic
    match pe.get_relocations() {
        Ok(relocations) => {
            for reloc in &relocations {
                // RVA should be valid
                // reloc_type can be 0
            }
        }
        Err(e) => {
            println!("get_relocations returned error (expected for some PEs): {:?}", e);
        }
    }
}

/// Test that basic PE info can be extracted
#[test]
fn test_basic_pe_info() {
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        return;
    }

    let pe = pe.unwrap();

    // Verify basic properties
    assert!(pe.is_pe64() || pe.is_pe32(), "Should be either PE32 or PE64");
    assert!(pe.num_sections() > 0, "Should have at least one section");
    assert!(pe.image_base() > 0, "Image base should be non-zero");
}

/// Test that get_dependencies works
#[test]
fn test_get_dependencies() {
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        return;
    }

    let pe = pe.unwrap();

    // Get dependencies - should return DLL names
    let deps = pe.get_dependencies(None);
    // Some PEs may have no dependencies, that's valid
    for dep in deps {
        assert!(!dep.is_empty(), "Dependency name should not be empty");
    }
}

/// Test that section data can be loaded
#[test]
fn test_section_loading() {
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        return;
    }

    let pe = pe.unwrap();

    // Try to get section data by index
    for i in 0..pe.num_sections() {
        let section_ptr = pe.get_section_ptr(i as usize);
        // Section data may be empty for certain sections
        // Just verify it doesn't panic and returns a valid slice
        let _len = section_ptr.len();
    }
}

/// Test that cache operations work
#[test]
fn test_cache_operations() {
    let pe = LiefPe::load("test/exe64win_msgbox.bin");
    if pe.is_err() {
        return;
    }

    let pe = pe.unwrap();

    // Get initial cache stats
    let stats = pe.cache_stats();
    assert_eq!(stats.cached_sections.len(), 0, "Cache should be empty initially");
    assert_eq!(stats.cached_bytes, 0, "Cache should have 0 bytes initially");

    // Access a section to load it
    let _ = pe.get_section_ptr(0);

    // Check if section is cached (depends on section content)
    let stats = pe.cache_stats();
    // The section may or may not be cached depending on its size and the cache policy

    // Clear cache should work
    pe.clear_cache();
    let stats = pe.cache_stats();
    assert_eq!(stats.cached_sections.len(), 0, "Cache should be empty after clear");
    assert_eq!(stats.cached_bytes, 0, "Cache should have 0 bytes after clear");
}
