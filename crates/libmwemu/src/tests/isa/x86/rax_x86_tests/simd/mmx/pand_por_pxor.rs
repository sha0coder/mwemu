//! Tests for PAND, POR, PXOR instructions.
//!
//! PAND/POR/PXOR - Packed Logical Operations (MMX)
//!
//! Performs bitwise logical operations on 64-bit MMX registers.
//!
//! Flags affected: None
//!
//! Reference: docs/pand.txt, docs/por.txt, docs/pxor.txt

use crate::*;

fn write_mm_via_mem(mem: u64, addr: u64, value: u64) {
    let mut emu = emu64();
    emu.maps.write_qword(addr, value);
}

// ============================================================================
// PAND mm, mm/m64 (opcode 0F DB /r)
// ============================================================================

#[test]
fn test_pand_mm_mm_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,                               // PAND MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF00000000);
    emu.maps.write_qword(0x2008, 0xFF00FF00FF00FF00);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFF00FF0000000000, "PAND: basic AND");
}

#[test]
fn test_pand_all_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "PAND: AND with all ones");
}

#[test]
fn test_pand_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PAND: AND with zeros");
}

#[test]
fn test_pand_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PAND MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PAND: complementary patterns");
}

#[test]
fn test_pand_self() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc0,                               // PAND MM0, MM0
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "PAND: self AND");
}

#[test]
fn test_pand_mask() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFEDCBA9876543210);
    emu.maps.write_qword(0x2008, 0x00FF00FF00FF00FF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x00DC009800540010, "PAND: byte mask");
}

#[test]
fn test_pand_mm3_mm4() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xdc,                               // PAND MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xF0F0F0F0F0F0F0F0);
    emu.maps.write_qword(0x2008, 0x0F0F0F0F0F0F0F0F);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PAND: MM3 & MM4");
}

#[test]
fn test_pand_sequential() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,                               // PAND MM0, MM1
        0x0f, 0xdb, 0xc2,                               // PAND MM0, MM2
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0xFFFF0000FFFF0000);
    emu.maps.write_qword(0x2010, 0xFF00FF00FF00FF00);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(result, 0xFF000000FF000000, "PAND: sequential");
}

// ============================================================================
// POR mm, mm/m64 (opcode 0F EB /r)
// ============================================================================

#[test]
fn test_por_mm_mm_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xc1,                               // POR MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFF000000FF000000);
    emu.maps.write_qword(0x2008, 0x00FF000000FF0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFF0000FFFF0000, "POR: basic OR");
}

#[test]
fn test_por_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "POR: OR with zeros");
}

#[test]
fn test_por_all_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "POR: OR with all ones");
}

#[test]
fn test_por_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // POR MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "POR: complementary patterns");
}

#[test]
fn test_por_self() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xc0,                               // POR MM0, MM0
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "POR: self OR");
}

#[test]
fn test_por_mm5_mm6() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xee,                               // POR MM5, MM6
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xF0F0F0F0F0F0F0F0);
    emu.maps.write_qword(0x2008, 0x0F0F0F0F0F0F0F0F);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "POR: MM5 | MM6");
}

#[test]
fn test_por_sequential() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xeb, 0xc1,                               // POR MM0, MM1
        0x0f, 0xeb, 0xc2,                               // POR MM0, MM2
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFF00000000000000);
    emu.maps.write_qword(0x2008, 0x00FF000000000000);
    emu.maps.write_qword(0x2010, 0x0000FF0000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(result, 0xFFFFFF0000000000, "POR: sequential");
}

// ============================================================================
// PXOR mm, mm/m64 (opcode 0F EF /r)
// ============================================================================

#[test]
fn test_pxor_mm_mm_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,                               // PXOR MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFF00FF00FF00FF00);
    emu.maps.write_qword(0x2008, 0xFFFF0000FFFF0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x00FFFF0000FFFF00, "PXOR: basic XOR");
}

#[test]
fn test_pxor_zero_register_idiom() {
    let mut emu = emu64();
    // PXOR MM0, MM0 is the standard way to zero a register
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc0,                               // PXOR MM0, MM0
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFEDCBA9876543210);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PXOR: zero register idiom");
}

#[test]
fn test_pxor_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "PXOR: XOR with zeros");
}

#[test]
fn test_pxor_all_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xEDCBA9876F543210, "PXOR: XOR with all ones (NOT)");
}

#[test]
fn test_pxor_mm_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PXOR MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PXOR: complementary patterns");
}

#[test]
fn test_pxor_same_value() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PXOR: same value");
}

#[test]
fn test_pxor_mm7_mm1() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xf9,                               // PXOR MM7, MM1
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xF0F0F0F0F0F0F0F0);
    emu.maps.write_qword(0x2008, 0x0F0F0F0F0F0F0F0F);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PXOR: MM7 ^ MM1");
}

#[test]
fn test_pxor_double() {
    let mut emu = emu64();
    // XOR twice with same value returns original
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,                               // PXOR MM0, MM1
        0x0f, 0xef, 0xc1,                               // PXOR MM0, MM1 again
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0xFEDCBA9876543210);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234567890ABCDEF, "PXOR: double XOR");
}

#[test]
fn test_pxor_sequential() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc1,                               // PXOR MM0, MM1
        0x0f, 0xef, 0xc2,                               // PXOR MM0, MM2
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);
    emu.maps.write_qword(0x2010, 0xFFFF0000FFFF0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(result, 0x0000FFFF0000FFFF, "PXOR: sequential");
}

// ============================================================================
// Combined tests
// ============================================================================

#[test]
fn test_combined_logical_ops() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xc1,                               // PAND MM0, MM1
        0x0f, 0xeb, 0xc2,                               // POR MM0, MM2
        0x0f, 0x6f, 0x1c, 0x25, 0x18, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xc3,                               // PXOR MM0, MM3
        0x0f, 0x7f, 0x04, 0x25, 0x20, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF00000000);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2010, 0x0000FFFF0000FFFF);
    emu.maps.write_qword(0x2018, 0xF0F0F0F0F0F0F0F0);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2020).unwrap();
    // AND: FFFFFFFF00000000 & FFFFFFFFFFFFFFFF = FFFFFFFF00000000
    // OR:  FFFFFFFF00000000 | 0000FFFF0000FFFF = FFFFFFFF0000FFFF
    // XOR: FFFFFFFF0000FFFF ^ F0F0F0F0F0F0F0F0 = 0F0F0F0FF0F00F0F
    assert_eq!(result, 0x0F0F0F0FF0F00F0F, "Combined logical ops");
}

#[test]
fn test_demorgan_laws() {
    let mut emu = emu64();
    // NOT(A AND B) = NOT(A) OR NOT(B)
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        // Calculate A AND B
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0xdb, 0xd1,                               // PAND MM2, MM1
        // NOT using XOR with all 1s
        0x0f, 0x6f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0xef, 0xd3,                               // PXOR MM2, MM3 (NOT)
        0x0f, 0x7f, 0x14, 0x25, 0x18, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);
    emu.maps.write_qword(0x2010, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "DeMorgan's Law");
}

#[test]
fn test_all_registers_pxor_zero() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0xef, 0xc0,                               // PXOR MM0, MM0
        0x0f, 0xef, 0xc9,                               // PXOR MM1, MM1
        0x0f, 0xef, 0xd2,                               // PXOR MM2, MM2
        0x0f, 0xef, 0xdb,                               // PXOR MM3, MM3
        0x0f, 0xef, 0xe4,                               // PXOR MM4, MM4
        0x0f, 0xef, 0xed,                               // PXOR MM5, MM5
        0x0f, 0xef, 0xf6,                               // PXOR MM6, MM6
        0x0f, 0xef, 0xff,                               // PXOR MM7, MM7
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x3c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.run(None).unwrap();

    let mm0 = emu.maps.read_qword(0x2000).unwrap();
    let mm7 = emu.maps.read_qword(0x2008).unwrap();
    assert_eq!(mm0, 0x0000000000000000, "MM0 zeroed");
    assert_eq!(mm7, 0x0000000000000000, "MM7 zeroed");
}
