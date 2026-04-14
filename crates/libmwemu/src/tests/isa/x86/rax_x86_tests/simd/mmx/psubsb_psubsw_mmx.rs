//! Tests for PSUBSB and PSUBSW instructions (MMX).
//!
//! PSUBSB - Subtract Packed Signed Bytes with Saturation (MMX)
//! PSUBSW - Subtract Packed Signed Words with Saturation (MMX)
//!
//! Subtracts packed signed integers with saturation.
//! If underflow/overflow occurs, result is clamped to min/max value.
//!
//! Opcodes:
//! - PSUBSB: 0F E8 /r
//! - PSUBSW: 0F E9 /r
//!
//! Flags affected: None
//!
//! Reference: /Users/int/dev/rax/docs/psubsb:psubsw.txt

use crate::*;

fn write_mm_via_mem(mem: u64, addr: u64, value: u64) {
    let mut emu = emu64();
    emu.maps.write_qword(addr, value);
}

// ============================================================================
// PSUBSB mm, mm/m64 - Subtract Packed Signed Bytes with Saturation
// ============================================================================

#[test]
fn test_psubsb_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xe8, 0xc1,                               // PSUBSB MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 10-1=9, 20-2=18, 30-3=27, 40-4=36, 50-5=45, 60-6=54, 70-7=63, 80-8=72
    emu.maps.write_qword(0x2000, 0x50463C32281E140A);
    emu.maps.write_qword(0x2008, 0x0807060504030201);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x483F362D241B1209, "PSUBSB: basic subtraction");
}

#[test]
fn test_psubsb_positive_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 127 - (-1) = would be 128, saturates to 127
    emu.maps.write_qword(0x2000, 0x7F7F7F7F7F7F7F7F);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7F7F7F7F7F7F7F7F, "PSUBSB: positive saturation");
}

#[test]
fn test_psubsb_negative_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // -128 - 1 = would be -129, saturates to -128
    emu.maps.write_qword(0x2000, 0x8080808080808080);
    emu.maps.write_qword(0x2008, 0x0101010101010101);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x8080808080808080, "PSUBSB: negative saturation");
}

#[test]
fn test_psubsb_mixed_signs() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0A14FFF0E6DC8060);
    emu.maps.write_qword(0x2008, 0x0564F0F01428A050);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x05B00F00D2B4E010, "PSUBSB: mixed signs");
}

#[test]
fn test_psubsb_zero_difference() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1234567890ABCDEF);
    emu.maps.write_qword(0x2008, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PSUBSB: zero difference");
}

#[test]
fn test_psubsb_max_positive_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 127 - 0 = 127
    emu.maps.write_qword(0x2000, 0x7F7F7F7F7F7F7F7F);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7F7F7F7F7F7F7F7F, "PSUBSB: max positive values");
}

#[test]
fn test_psubsb_min_negative_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // -128 - 0 = -128
    emu.maps.write_qword(0x2000, 0x8080808080808080);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x8080808080808080, "PSUBSB: min negative values");
}

#[test]
fn test_psubsb_alternating_pattern() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7F807F807F807F80);
    emu.maps.write_qword(0x2008, 0x017F017F017F017F);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7E807E807E807E80, "PSUBSB: alternating pattern");
}

#[test]
fn test_psubsb_memory_operand() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0xe8, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // PSUBSB MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x5040302010000000);
    emu.maps.write_qword(0x2008, 0x1010101010000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x4030201000000000, "PSUBSB: memory operand");
}

#[test]
fn test_psubsb_boundary_cases() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe8, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7F80FF007F800000);
    emu.maps.write_qword(0x2008, 0x807F01007F000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7F80FE0000800000, "PSUBSB: boundary cases");
}

// ============================================================================
// PSUBSW mm, mm/m64 - Subtract Packed Signed Words with Saturation
// ============================================================================

#[test]
fn test_psubsw_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xe9, 0xc1,                               // PSUBSW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 1000-100=900, 2000-200=1800, 3000-300=2700, 4000-400=3600
    emu.maps.write_qword(0x2000, 0x0FA00BB807D003E8);
    emu.maps.write_qword(0x2008, 0x0190012C00C80064);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0E100A8C07080384, "PSUBSW: basic subtraction");
}

#[test]
fn test_psubsw_positive_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 32767 - (-1) = would be 32768, saturates to 32767
    emu.maps.write_qword(0x2000, 0x7FFF7FFF7FFF7FFF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7FFF7FFF7FFF7FFF, "PSUBSW: positive saturation");
}

#[test]
fn test_psubsw_negative_saturation() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // -32768 - 1 = would be -32769, saturates to -32768
    emu.maps.write_qword(0x2000, 0x8000800080008000);
    emu.maps.write_qword(0x2008, 0x0001000100010001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x8000800080008000, "PSUBSW: negative saturation");
}

#[test]
fn test_psubsw_mixed_signs() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0064FFF0E700A000);
    emu.maps.write_qword(0x2008, 0x00C8FF388000D000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFF9C00B86700D000, "PSUBSW: mixed signs");
}

#[test]
fn test_psubsw_zero_difference() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x123456789ABCDEF0);
    emu.maps.write_qword(0x2008, 0x123456789ABCDEF0);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PSUBSW: zero difference");
}

#[test]
fn test_psubsw_max_positive_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // 32767 - 0 = 32767
    emu.maps.write_qword(0x2000, 0x7FFF7FFF7FFF7FFF);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7FFF7FFF7FFF7FFF, "PSUBSW: max positive values");
}

#[test]
fn test_psubsw_min_negative_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // -32768 - 0 = -32768
    emu.maps.write_qword(0x2000, 0x8000800080008000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x8000800080008000, "PSUBSW: min negative values");
}

#[test]
fn test_psubsw_alternating_pattern() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7FFF80007FFF8000);
    emu.maps.write_qword(0x2008, 0x00017FFF00017FFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7FFE80007FFE8000, "PSUBSW: alternating pattern");
}

#[test]
fn test_psubsw_memory_operand() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0xe9, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // PSUBSW MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x03E807D00BB80FA0);
    emu.maps.write_qword(0x2008, 0x006400C801900320);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x038407080A280C80, "PSUBSW: memory operand");
}

#[test]
fn test_psubsw_boundary_cases() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x7FFF8000FF000000);
    emu.maps.write_qword(0x2008, 0x80007FFF01000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x7FFF8000FE000000, "PSUBSW: boundary cases");
}

#[test]
fn test_psubsw_large_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x5000500050005000);
    emu.maps.write_qword(0x2008, 0x1000100010001000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x4000400040004000, "PSUBSW: large values");
}

#[test]
fn test_psubsw_small_negative_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0xe9, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFF0FFF0FFF0FFF0); // -16 in all words
    emu.maps.write_qword(0x2008, 0xFFFCFFFCFFFCFFFC); // -4 in all words

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFF4FFF4FFF4FFF4, "PSUBSW: small negative values"); // -12 in all words
}

#[test]
fn test_psubsw_sequential() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0xe9, 0xc1,                               // PSUBSW MM0, MM1
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2010]
        0x0f, 0xe9, 0xc2,                               // PSUBSW MM0, MM2
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // MOVQ [0x2018], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1000100010001000);
    emu.maps.write_qword(0x2008, 0x0100010001000100);
    emu.maps.write_qword(0x2010, 0x0100010001000100);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(result, 0x0E000E000E000E00, "PSUBSW: sequential subtraction");
}

#[test]
fn test_psubsw_register_reuse() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0xe9, 0xc0,                               // PSUBSW MM0, MM0
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x123456789ABCDEF0);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PSUBSW: register reuse (self-subtract)");
}
