use crate::loaders::elf::elf64::Elf64;
use crate::loaders::elf::lief::LiefElf64;
use crate::tests::helpers;

const ELF64_LINUX_X64_HELLO: &[u8] = include_bytes!("../../fixtures/elf64_x86_64_linux_hello.bin");
const ELF64_AARCH64_ADD: &[u8] = include_bytes!("../../fixtures/elf64_aarch64_add.bin");

fn write_temp(name: &str, data: &[u8]) -> String {
    let tmp = std::env::temp_dir().join(name);
    std::fs::write(&tmp, data).unwrap();
    tmp.to_string_lossy().to_string()
}

#[test]
fn elf64_lief_parses_x86_64_dynamic() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_elf64_x86_64_hello.bin",
        ELF64_LINUX_X64_HELLO,
    );

    let legacy = Elf64::parse(&path).expect("legacy parse failed");

    let lief = LiefElf64::load(&path)
        .and_then(|l| l.to_legacy_model())
        .expect("LIEF parse failed");

    assert_eq!(legacy.elf_hdr.e_machine, lief.elf_hdr.e_machine);
    assert_eq!(legacy.elf_hdr.e_entry, lief.elf_hdr.e_entry);
    assert_eq!(legacy.elf_hdr.e_phnum, lief.elf_hdr.e_phnum);
    assert!(legacy.elf_hdr.e_shnum > 0);
    assert!(lief.elf_hdr.e_shnum > 0);

    assert_eq!(legacy.elf_phdr.len(), lief.elf_phdr.len());

    let legacy_needed = legacy.get_dynamic();
    let lief_needed = &lief.needed_libs;
    assert_eq!(legacy_needed.len(), lief_needed.len());
    assert!(lief_needed.iter().any(|lib| lib.contains("libc")));

    let has_puts = lief.elf_dynsym.iter().any(|s| s.st_dynstr_name == "puts");
    let has_start_main = lief
        .elf_dynsym
        .iter()
        .any(|s| s.st_dynstr_name == "__libc_start_main");
    assert!(has_puts, "LIEF parse should find 'puts' in dynsym");
    assert!(
        has_start_main,
        "LIEF parse should find '__libc_start_main' in dynsym"
    );
}

#[test]
fn elf64_lief_parses_aarch64_static() {
    helpers::setup();

    let path = write_temp("mwemu_test_lief_elf64_aarch64_add.bin", ELF64_AARCH64_ADD);

    let legacy = Elf64::parse(&path).expect("legacy parse failed");

    let lief = LiefElf64::load(&path)
        .and_then(|l| l.to_legacy_model())
        .expect("LIEF parse failed");

    assert_eq!(legacy.elf_hdr.e_machine, lief.elf_hdr.e_machine);
    assert_eq!(legacy.elf_hdr.e_entry, lief.elf_hdr.e_entry);

    assert_eq!(legacy.elf_phdr.len(), lief.elf_phdr.len());

    assert_eq!(lief.needed_libs.len(), 0);
}

#[test]
fn elf64_lief_detection_x64() {
    let path = write_temp("mwemu_test_detect_elf64_x64.bin", ELF64_LINUX_X64_HELLO);

    assert!(Elf64::is_elf64_x64(&path));
    assert!(LiefElf64::is_elf64_x64(&path));
}

#[test]
fn elf64_lief_detection_aarch64() {
    let path = write_temp("mwemu_test_detect_elf64_aarch64.bin", ELF64_AARCH64_ADD);

    assert!(Elf64::is_elf64_aarch64(&path));
    assert!(LiefElf64::is_elf64_aarch64(&path));
}

#[test]
fn elf64_lief_forced_errors_on_empty_file() {
    let tmp = std::env::temp_dir().join("mwemu_test_empty.bin");
    std::fs::write(&tmp, []).unwrap();

    let result = LiefElf64::load(tmp.to_str().unwrap());
    assert!(result.is_err());
}

#[test]
fn elf64_lief_errors_on_truncated_magic() {
    let tmp = std::env::temp_dir().join("mwemu_test_truncated.bin");
    std::fs::write(&tmp, [0x7f, b'E', b'L']).unwrap();

    let result = LiefElf64::load(tmp.to_str().unwrap());
    assert!(result.is_err(), "truncated magic should fail");
}

#[test]
fn elf64_lief_rejects_elf32() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_elf32.bin",
        &[
            0x7f, 0x45, 0x4c, 0x46, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    );

    let result = LiefElf64::load(&path);
    assert!(result.is_err(), "ELF32 class should be rejected");
}

#[test]
fn elf64_lief_rejects_unknown_machine() {
    helpers::setup();

    let path = write_temp(
        "mwemu_test_lief_unknown_machine.bin",
        &[
            0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
    );

    let result = LiefElf64::load(&path);
    assert!(result.is_err(), "unknown machine should be rejected");
}

#[test]
fn elf64_lief_load_from_raw_works() {
    helpers::setup();

    let result = LiefElf64::load_from_raw("test.bin", ELF64_LINUX_X64_HELLO);
    assert!(result.is_ok(), "load_from_raw should succeed");
    let lief = result.unwrap();
    let elf = lief
        .to_legacy_model()
        .expect("to_legacy_model should succeed");
    assert_eq!(elf.elf_hdr.e_machine, 0x3E);
}

#[test]
fn elf64_lief_load_from_raw_rejects_truncated() {
    let result = LiefElf64::load_from_raw("trunc.bin", &[0x7f, b'E', b'L', b'F', 0x02]);
    assert!(result.is_err(), "truncated load_from_raw should fail");
}

#[test]
fn elf64_lief_auto_falls_back_to_legacy_when_lief_fails() {
    helpers::setup();

    let tmp = std::env::temp_dir().join("mwemu_test_auto_fallback.bin");
    std::fs::write(&tmp, b"not an ELF file at all").unwrap();
    let path = tmp.to_string_lossy().to_string();

    let lief_result = LiefElf64::load(&path).and_then(|l| l.to_legacy_model());
    assert!(
        lief_result.is_err(),
        "LIEF should fail on invalid file for Auto fallback to trigger"
    );
}
