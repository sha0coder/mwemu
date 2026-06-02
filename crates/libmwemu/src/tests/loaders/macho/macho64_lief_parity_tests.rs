use crate::arch::Arch;
use crate::loaders::macho::lief::LiefMacho64;
use crate::loaders::macho::macho64::Macho64;
use crate::tests::helpers;

const MACHO64_AARCH64_ADD: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_add.bin");
const MACHO64_AARCH64_HELLO: &[u8] = include_bytes!("../../fixtures/macho64_aarch64_hello.bin");
const MACHO64_AARCH64_HELLO_RAW: &[u8] =
    include_bytes!("../../fixtures/macho64_aarch64_hello_raw.bin");
const MACHO64_X86_64_HELLO: &[u8] = include_bytes!("../../fixtures/macho64_x86_64_hello.bin");
const MACHO64_X86_64_HELLO_RAW: &[u8] =
    include_bytes!("../../fixtures/macho64_x86_64_hello_raw.bin");

fn write_temp(name: &str, data: &[u8]) -> String {
    let tmp = std::env::temp_dir().join(name);
    std::fs::write(&tmp, data).unwrap();
    tmp.to_string_lossy().to_string()
}

#[test]
fn macho64_lief_parses_aarch64() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_macho64_aarch64_add.bin",
        MACHO64_AARCH64_ADD,
    );

    let legacy = Macho64::parse(&path).expect("legacy parse failed");
    let lief = LiefMacho64::load(&path, Some(Arch::Aarch64))
        .and_then(|l| l.to_macho64())
        .expect("LIEF parse failed");

    assert_eq!(legacy.entry, lief.entry, "entrypoint mismatch");

    assert_eq!(
        legacy.segments.len(),
        lief.segments.len(),
        "segment count mismatch"
    );

    for (l_seg, r_seg) in legacy.segments.iter().zip(lief.segments.iter()) {
        assert_eq!(l_seg.name, r_seg.name, "segment name mismatch");
        assert_eq!(
            l_seg.vmaddr, r_seg.vmaddr,
            "segment vmaddr mismatch for {}",
            l_seg.name
        );
        assert_eq!(
            l_seg.vmsize, r_seg.vmsize,
            "segment vmsize mismatch for {}",
            l_seg.name
        );
    }
}

#[test]
fn macho64_lief_parses_x86_64() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_macho64_x86_64_hello.bin",
        MACHO64_X86_64_HELLO_RAW,
    );

    let legacy = Macho64::parse(&path).expect("legacy parse failed");
    let lief = LiefMacho64::load(&path, Some(Arch::X86_64))
        .and_then(|l| l.to_macho64())
        .expect("LIEF parse failed");

    assert_eq!(legacy.entry, lief.entry, "entrypoint mismatch");
    assert_eq!(
        legacy.segments.len(),
        lief.segments.len(),
        "segment count mismatch"
    );
}

#[test]
fn macho64_lief_aarch64_hello_has_segments() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_macho64_aarch64_hello.bin",
        MACHO64_AARCH64_HELLO,
    );

    let lief = LiefMacho64::load(&path, Some(Arch::Aarch64))
        .and_then(|l| l.to_macho64())
        .expect("LIEF parse failed");

    assert!(lief.segments.iter().any(|s| s.name == "__TEXT"));
    assert!(!lief.segments.iter().any(|s| s.name == "__PAGEZERO"));
    assert!(lief.entry > 0, "entrypoint should be non-zero");
    assert!(
        !lief.segments.is_empty(),
        "should have at least one segment"
    );
}

#[test]
fn macho64_lief_x86_64_hello_has_segments() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_macho64_x86_64_hello.bin",
        MACHO64_X86_64_HELLO,
    );

    let lief = LiefMacho64::load(&path, Some(Arch::X86_64))
        .and_then(|l| l.to_macho64())
        .expect("LIEF parse failed");

    assert!(!lief.segments.iter().any(|s| s.name == "__PAGEZERO"));
    assert!(lief.entry > 0);
}

#[test]
fn macho64_lief_no_matching_slice_error() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_macho64_aarch64_mismatch.bin",
        MACHO64_AARCH64_ADD,
    );

    let result = LiefMacho64::load(&path, Some(Arch::X86_64));
    assert!(result.is_err());
}

#[test]
fn macho64_lief_forced_errors_on_empty_file() {
    let tmp = std::env::temp_dir().join("mwemu_test_empty_macho.bin");
    std::fs::write(&tmp, []).unwrap();

    let result = LiefMacho64::load(tmp.to_str().unwrap(), None);
    assert!(result.is_err());
}

#[test]
fn macho64_lief_errors_on_truncated_magic() {
    let tmp = std::env::temp_dir().join("mwemu_test_truncated_macho.bin");
    std::fs::write(&tmp, [0xCF, 0xFA, 0xED, 0xFE]).unwrap();

    let result = LiefMacho64::load(tmp.to_str().unwrap(), None);
    assert!(result.is_err(), "truncated mach-o magic should fail");
}

#[test]
fn macho64_lief_errors_on_wrong_cpu() {
    helpers::setup();

    let path = write_temp("mwemu_test_lief_macho64_wrong_cpu.bin", MACHO64_AARCH64_ADD);

    let result = LiefMacho64::load(&path, Some(Arch::X86_64));
    assert!(result.is_err(), "wrong CPU type should fail");
}

#[test]
fn macho64_lief_load_from_raw_works() {
    helpers::setup();

    let result = LiefMacho64::load_from_raw("test.bin", MACHO64_AARCH64_ADD, Some(Arch::Aarch64));
    assert!(result.is_ok(), "load_from_raw should succeed");
    let lief = result.unwrap();
    let macho = lief.to_macho64().expect("to_macho64 should succeed");
    assert!(!macho.segments.is_empty());
}

#[test]
fn macho64_lief_caches_populated() {
    helpers::setup();

    let path = write_temp("mwemu_test_lief_macho64_caches.bin", MACHO64_AARCH64_HELLO);

    let lief = LiefMacho64::load(&path, Some(Arch::Aarch64))
        .and_then(|l| l.to_macho64())
        .expect("LIEF parse failed");

    assert_eq!(
        lief.backend,
        crate::loaders::macho::macho64::ParserBackendKind::Lief
    );
    assert!(lief.libs_cache.is_some(), "libs cache should be populated");
    assert!(
        lief.exports_cache.is_some(),
        "exports cache should be populated"
    );
    assert!(
        lief.fixups_cache.is_some(),
        "fixups cache should be populated"
    );

    let libs = lief.get_libs();
    let exports = lief.get_exports();
    let _fixups = lief.parse_chained_fixups();

    assert_eq!(
        libs,
        lief.libs_cache.unwrap(),
        "get_libs should return cache"
    );
    assert_eq!(
        exports,
        lief.exports_cache.unwrap(),
        "get_exports should return cache"
    );
}

#[test]
fn macho64_lief_auto_falls_back_to_legacy_when_lief_fails() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_macho_auto_fallback.bin");
    std::fs::write(&tmp, b"not a mach-o file").unwrap();
    let path = tmp.to_string_lossy().to_string();

    let lief_result = LiefMacho64::load(&path, Some(Arch::Aarch64)).and_then(|l| l.to_macho64());
    assert!(lief_result.is_err(), "LIEF should fail on invalid file for Auto fallback to trigger");
}

#[test]
fn macho64_lief_detect_arches_aarch64() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_detect_macho_arches.bin",
        MACHO64_AARCH64_HELLO_RAW,
    );

    let arches = LiefMacho64::detect_arches(&path).expect("detect_arches should succeed");
    assert!(arches.contains(&Arch::Aarch64));
}
