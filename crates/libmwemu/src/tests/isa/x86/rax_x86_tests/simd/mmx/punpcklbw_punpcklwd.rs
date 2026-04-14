//! Tests for PUNPCKLBW and PUNPCKLWD instructions (MMX).
//!
//! PUNPCKLBW/PUNPCKLWD - Unpack Low Data
//!
//! Unpacks and interleaves the low-order data elements (bytes or words) of the
//! destination and source operands into the destination operand.
//!
//! - PUNPCKLBW: Interleave low-order bytes from mm and mm/m32 into mm
//! - PUNPCKLWD: Interleave low-order words from mm and mm/m32 into mm
//!
//! Flags affected: None
//!
//! Reference: docs/punpcklbw:punpcklwd:punpckldq:punpcklqdq.txt

use crate::*;

// Helper to write 64-bit value to memory
fn write_mem_at_u64(mem: u64, addr: u64, value: u64) {
    let mut emu = emu64();
    emu.maps.write_bytes_slice(addr, &value.to_le_bytes());
}

// Helper to read 64-bit value from memory
fn read_mem_at_u64(mem: u64, addr: u64) -> u64 {
    let mut emu = emu64();
    let mut buf = [0u8; 8];
    emu.maps.read_bytes_buff(&mut buf, addr);
    u64::from_le_bytes(buf)
}

// ============================================================================
// PUNPCKLBW mm, mm/m32 (opcode 0F 60 /r) - Interleave low-order bytes
// ============================================================================

#[test]
fn test_punpcklbw_mm_mm_basic() {
    let mut emu = emu64();
    // PUNPCKLBW MM0, MM1 - basic interleaving
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x60, 0xc1,                               // PUNPCKLBW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM0
        0xf4,                                            // HLT
    ];

    emu.load_code_bytes(&code);

    // MM0 = 0x0706050403020100 (low bytes: 03 02 01 00)
    emu.maps.write_qword(0x2000, 0x0706050403020100);
    // MM1 = 0x0F0E0D0C0B0A0908 (low bytes: 0B 0A 09 08)
    emu.maps.write_qword(0x2008, 0x0F0E0D0C0B0A0908);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0B030A020901_0800, "PUNPCKLBW: basic interleaving");
}

#[test]
fn test_punpcklbw_mm_mm_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PUNPCKLBW: all zeros");
}

#[test]
fn test_punpcklbw_mm_mm_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PUNPCKLBW: all ones");
}

#[test]
fn test_punpcklbw_mm_mm_alternating() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x55AA55AA55AA55AA, "PUNPCKLBW: alternating pattern");
}

#[test]
fn test_punpcklbw_mm_m32() {
    let mut emu = emu64();
    // PUNPCKLBW MM2, [memory] - only low 32 bits of memory accessed
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2000]
        0x0f, 0x60, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PUNPCKLBW MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x8877665544332211);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFCCBBAA99);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xCC44BB33AA229911, "PUNPCKLBW: memory operand");
}

#[test]
fn test_punpcklbw_sequential_bytes() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0706050403020100);
    emu.maps.write_qword(0x2008, 0x0F0E0D0C0B0A0908);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0B030A0209010800, "PUNPCKLBW: sequential bytes");
}

#[test]
fn test_punpcklbw_mm3_mm4() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2000]
        0x0f, 0x6f, 0x24, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM4, [0x2008]
        0x0f, 0x60, 0xdc,                               // PUNPCKLBW MM3, MM4
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ [0x2010], MM3
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1111111111111111);
    emu.maps.write_qword(0x2008, 0x2222222222222222);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x2211221122112211, "PUNPCKLBW: MM3 with MM4");
}

#[test]
fn test_punpcklbw_zero_extension() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFF00FF00FF00FF00);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x00FF000000FF0000, "PUNPCKLBW: zero extension");
}

#[test]
fn test_punpcklbw_high_bits_ignored() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF12345678);
    emu.maps.write_qword(0x2008, 0x0000000012345678);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1212343456567878, "PUNPCKLBW: high bits ignored");
}

#[test]
fn test_punpcklbw_mm5_mm6() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x34, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xee,
        0x0f, 0x7f, 0x2c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00000000DEADBEEF);
    emu.maps.write_qword(0x2008, 0x00000000CAFEBABE);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xCADEFEADBABEBEEF, "PUNPCKLBW: MM5 with MM6");
}

// ============================================================================
// PUNPCKLWD mm, mm/m32 (opcode 0F 61 /r) - Interleave low-order words
// ============================================================================

#[test]
fn test_punpcklwd_mm_mm_basic() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,                               // PUNPCKLWD MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    // MM0 = 0x0003000200010000 (low words: 0002 0001)
    emu.maps.write_qword(0x2000, 0x0004000300020001);
    // MM1 = 0x0007000600050004 (low words: 0006 0005)
    emu.maps.write_qword(0x2008, 0x0008000700060005);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0006000200050001, "PUNPCKLWD: basic interleaving");
}

#[test]
fn test_punpcklwd_mm_mm_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000000000000000, "PUNPCKLWD: all zeros");
}

#[test]
fn test_punpcklwd_mm_mm_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF, "PUNPCKLWD: all ones");
}

#[test]
fn test_punpcklwd_mm_mm_alternating() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0x5555555555555555);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x5555AAAA5555AAAA, "PUNPCKLWD: alternating pattern");
}

#[test]
fn test_punpcklwd_mm_m32() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0x14, 0x25, 0x08, 0x20, 0x00, 0x00, // PUNPCKLWD MM2, [0x2008]
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x8888777766665555);
    emu.maps.write_qword(0x2008, 0xFFFFFFFFCCCCBBBB);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xCCCC6666BBBB5555, "PUNPCKLWD: memory operand");
}

#[test]
fn test_punpcklwd_sequential_words() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0004000300020001);
    emu.maps.write_qword(0x2008, 0x0008000700060005);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0006000200050001, "PUNPCKLWD: sequential words");
}

#[test]
fn test_punpcklwd_mm3_mm7() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x3c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xdf,                               // PUNPCKLWD MM3, MM7
        0x0f, 0x7f, 0x1c, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1111111122222222);
    emu.maps.write_qword(0x2008, 0x3333333344444444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x4444222244442222, "PUNPCKLWD: MM3 with MM7");
}

#[test]
fn test_punpcklwd_zero_extension() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFF0000FFFF0000);
    emu.maps.write_qword(0x2008, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000FFFF00000000, "PUNPCKLWD: zero extension");
}

#[test]
fn test_punpcklwd_high_words_ignored() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF12345678);
    emu.maps.write_qword(0x2008, 0x0000000012345678);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1234123456785678, "PUNPCKLWD: high words ignored");
}

#[test]
fn test_punpcklwd_mm4_mm5() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x24, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x2c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xe5,                               // PUNPCKLWD MM4, MM5
        0x0f, 0x7f, 0x24, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00000000DEADBEEF);
    emu.maps.write_qword(0x2008, 0x00000000CAFEBABE);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xCAFEDEADBABEBEEF, "PUNPCKLWD: MM4 with MM5");
}

// ============================================================================
// Additional edge cases and comprehensive tests
// ============================================================================

#[test]
fn test_punpcklbw_single_byte_pattern() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000001010101);
    emu.maps.write_qword(0x2008, 0x0000000002020202);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0201020102010201, "PUNPCKLBW: repeating bytes");
}

#[test]
fn test_punpcklwd_single_word_pattern() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x0000000011111111);
    emu.maps.write_qword(0x2008, 0x0000000022222222);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x2222111122221111, "PUNPCKLWD: repeating words");
}

#[test]
fn test_punpcklbw_max_min_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00000000FF00FF00);
    emu.maps.write_qword(0x2008, 0x0000000000FF00FF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x00FFFF0000FFFF00, "PUNPCKLBW: max/min values");
}

#[test]
fn test_punpcklwd_max_min_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x00000000FFFF0000);
    emu.maps.write_qword(0x2008, 0x000000000000FFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x0000FFFFFFFF0000, "PUNPCKLWD: max/min values");
}

#[test]
fn test_punpcklbw_mixed_registers_mm2_mm3() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x14, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x1c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xd3,
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAABBBBBBBB);
    emu.maps.write_qword(0x2008, 0xCCCCCCCCDDDDDDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xDDBBDDBBDDBBDDBB, "PUNPCKLBW: MM2 with MM3");
}

#[test]
fn test_punpcklwd_mixed_registers_mm6_mm7() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x34, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x3c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xf7,
        0x0f, 0x7f, 0x34, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xAAAAAAAABBBBBBBB);
    emu.maps.write_qword(0x2008, 0xCCCCCCCCDDDDDDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xDDDDBBBBDDDDBBBB, "PUNPCKLWD: MM6 with MM7");
}

#[test]
fn test_punpcklbw_incremental_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF10111213);
    emu.maps.write_qword(0x2008, 0xFFFFFFFF14151617);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1410151116121713, "PUNPCKLBW: incremental values");
}

#[test]
fn test_punpcklwd_incremental_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF10111213);
    emu.maps.write_qword(0x2008, 0xFFFFFFFF14151617);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x1415101116171213, "PUNPCKLWD: incremental values");
}

#[test]
fn test_punpcklbw_byte_boundary() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x000000007F807F80);
    emu.maps.write_qword(0x2008, 0x00000000FF00FF01);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFF7F0080FF7F0180, "PUNPCKLBW: byte boundaries");
}

#[test]
fn test_punpcklwd_word_boundary() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x000000007FFF8000);
    emu.maps.write_qword(0x2008, 0x00000000FFFF0001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0xFFFF7FFF00018000, "PUNPCKLWD: word boundaries");
}

#[test]
fn test_punpcklbw_register_independence() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2010]
        0x0f, 0x60, 0xc1,                               // PUNPCKLBW MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00, // MOVQ [0x2018], MM0
        0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00, // MOVQ [0x2020], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1111111111111111);
    emu.maps.write_qword(0x2008, 0x2222222222222222);
    emu.maps.write_qword(0x2010, 0x3333333333333333);

    emu.run(None).unwrap();

    // MM0 was modified
    let mm0_result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(mm0_result, 0x2211221122112211, "PUNPCKLBW: MM0 result");

    // MM2 should be unchanged
    let mm2_result = emu.maps.read_qword(0x2020).unwrap();
    assert_eq!(mm2_result, 0x3333333333333333, "PUNPCKLBW: MM2 unchanged");
}

#[test]
fn test_punpcklwd_register_independence() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00,
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0xc1,                               // PUNPCKLWD MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x18, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x14, 0x25, 0x20, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0x1111111111111111);
    emu.maps.write_qword(0x2008, 0x2222222222222222);
    emu.maps.write_qword(0x2010, 0x3333333333333333);

    emu.run(None).unwrap();

    let mm0_result = emu.maps.read_qword(0x2018).unwrap();
    assert_eq!(mm0_result, 0x2222111122221111, "PUNPCKLWD: MM0 result");

    let mm2_result = emu.maps.read_qword(0x2020).unwrap();
    assert_eq!(mm2_result, 0x3333333333333333, "PUNPCKLWD: MM2 unchanged");
}

#[test]
fn test_punpcklbw_memory_alignment() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x60, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // PUNPCKLBW MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF01234567);
    emu.maps.write_qword(0x2008, 0xFFFFFFFF89ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x8901AB23CD45EF67, "PUNPCKLBW: memory alignment");
}

#[test]
fn test_punpcklwd_memory_alignment() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x61, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // PUNPCKLWD MM0, [0x2008]
        0x0f, 0x7f, 0x04, 0x25, 0x10, 0x20, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);

    emu.maps.write_qword(0x2000, 0xFFFFFFFF01234567);
    emu.maps.write_qword(0x2008, 0xFFFFFFFF89ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x2010).unwrap();
    assert_eq!(result, 0x89AB0123CDEF4567, "PUNPCKLWD: memory alignment");
}
