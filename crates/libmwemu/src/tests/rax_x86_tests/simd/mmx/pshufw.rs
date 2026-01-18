//! Tests for the PSHUFW instruction (MMX).
//!
//! PSHUFW - Shuffle Packed Words
//!
//! Copies words from the source operand (second operand) and inserts them in the
//! destination operand (first operand) at word locations selected with the order
//! operand (third operand). This operation is similar to the operation used by the
//! PSHUFD instruction. For the PSHUFW instruction, each 2-bit field in the order
//! operand selects the contents of one word location in the destination operand.
//!
//! The source operand can be an MMX technology register or a 64-bit memory location.
//! The destination operand is an MMX technology register. The order operand is an
//! 8-bit immediate. Note that this instruction permits a word in the source operand
//! to be copied to more than one word location in the destination operand.
//!
//! Opcode: NP 0F 70 /r ib
//!
//! Operation:
//! DEST[15:0] := (SRC >> (ORDER[1:0] * 16))[15:0];
//! DEST[31:16] := (SRC >> (ORDER[3:2] * 16))[15:0];
//! DEST[47:32] := (SRC >> (ORDER[5:4] * 16))[15:0];
//! DEST[63:48] := (SRC >> (ORDER[7:6] * 16))[15:0];
//!
//! Flags affected: None
//!
//! Reference: /Users/int/dev/rax/docs/pshufw.txt

use crate::*;

// ============================================================================
// PSHUFW Tests: Identity and Reverse Operations
// ============================================================================

#[test]
fn test_pshufw_identity() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x70, 0xc0, 0b11_10_01_00,                 // PSHUFW MM0, MM0, 0xE4
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,                                            // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0001_0002_0003_0004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0001_0002_0003_0004,
        "Identity shuffle should preserve order");
}

#[test]
fn test_pshufw_reverse() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x70, 0xc0, 0b00_01_10_11,                 // PSHUFW MM0, MM0, 0x1B
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0001_0002_0003_0004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0004_0003_0002_0001,
        "Reverse shuffle should reverse word order");
}

#[test]
fn test_pshufw_swap_pairs() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b10_11_00_01,                 // PSHUFW MM0, MM0, 0xB1
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xBBBB_AAAA_DDDD_CCCC,
        "Should swap adjacent word pairs");
}

// ============================================================================
// PSHUFW Tests: Broadcast Operations
// ============================================================================

#[test]
fn test_pshufw_broadcast_word0() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_00_00_00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x4444_4444_4444_4444,
        "Should broadcast word 0 to all positions");
}

#[test]
fn test_pshufw_broadcast_word1() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b01_01_01_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x3333_3333_3333_3333,
        "Should broadcast word 1 to all positions");
}

#[test]
fn test_pshufw_broadcast_word2() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b10_10_10_10,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x2222_2222_2222_2222,
        "Should broadcast word 2 to all positions");
}

#[test]
fn test_pshufw_broadcast_word3() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b11_11_11_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x1111_1111_1111_1111,
        "Should broadcast word 3 to all positions");
}

// ============================================================================
// PSHUFW Tests: Custom Shuffle Patterns
// ============================================================================

#[test]
fn test_pshufw_pattern_0123() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_01_10_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xDDDD_CCCC_BBBB_AAAA,
        "Should shuffle to pattern 0,1,2,3");
}

#[test]
fn test_pshufw_pattern_3210() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b11_10_01_00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xAAAA_BBBB_CCCC_DDDD,
        "Should preserve with pattern 3,2,1,0");
}

#[test]
fn test_pshufw_pattern_0321() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_11_10_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x4444_1111_2222_3333,
        "Should shuffle to pattern 0,3,2,1");
}

#[test]
fn test_pshufw_pattern_1032() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b01_00_11_10,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x3333_4444_1111_2222,
        "Should shuffle to pattern 1,0,3,2");
}

// ============================================================================
// PSHUFW Tests: Memory Source Operand
// ============================================================================

#[test]
fn test_pshufw_mm_m64_identity() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x70, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0b11_10_01_00, // PSHUFW MM0, [0x2008], imm
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFF_FFFF_FFFF_FFFF);
    emu.maps.write_qword(0x2008, 0xABCD_1234_5678_9ABC);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xABCD_1234_5678_9ABC,
        "Memory source with identity pattern");
}

#[test]
fn test_pshufw_mm_m64_reverse() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0b00_01_10_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000_0000_0000_0000);
    emu.maps.write_qword(0x2008, 0x0001_0002_0003_0004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0004_0003_0002_0001,
        "Memory source with reverse pattern");
}

#[test]
fn test_pshufw_mm_m64_broadcast() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, 0b10_10_10_10,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFF_FFFF_FFFF_FFFF);
    emu.maps.write_qword(0x2008, 0x1111_CAFE_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xCAFE_CAFE_CAFE_CAFE,
        "Memory source broadcast word 2");
}

// ============================================================================
// PSHUFW Tests: Different Destination Registers
// ============================================================================

#[test]
fn test_pshufw_different_registers() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x70, 0xc8, 0b00_01_10_11,                 // PSHUFW MM1, MM0, imm
        0x0f, 0x7f, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM1
        0x0f, 0x7f, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVQ [0x3008], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234_5678_9ABC_DEF0);

    emu.run(None).unwrap();

    let result_mm1 = emu.maps.read_qword(0x3000).unwrap();
    let result_mm0 = emu.maps.read_qword(0x3008).unwrap();

    assert_eq!(result_mm1, 0xDEF0_9ABC_5678_1234,
        "MM1 should contain shuffled result");
    assert_eq!(result_mm0, 0x1234_5678_9ABC_DEF0,
        "MM0 should remain unchanged");
}

#[test]
fn test_pshufw_mm2_mm3() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2000]
        0x0f, 0x70, 0xd3, 0b11_00_01_10,                 // PSHUFW MM2, MM3, imm
        0x0f, 0x7f, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xAAAA_DDDD_CCCC_BBBB,
        "Should shuffle to pattern 3,0,1,2");
}

// ============================================================================
// PSHUFW Tests: All Zero and All Ones
// ============================================================================

#[test]
fn test_pshufw_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b01_10_11_00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000_0000_0000_0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0000_0000_0000_0000,
        "All zeros should remain all zeros");
}

#[test]
fn test_pshufw_all_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b10_01_00_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFF_FFFF_FFFF_FFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFFFF_FFFF_FFFF_FFFF,
        "All ones should remain all ones");
}

// ============================================================================
// PSHUFW Tests: Alternating Patterns
// ============================================================================

#[test]
fn test_pshufw_alternating_5555() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_11_00_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x5555_5555_5555_5555);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x5555_5555_5555_5555,
        "Alternating pattern should be preserved");
}

#[test]
fn test_pshufw_alternating_aaaa() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b01_10_01_10,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_AAAA_AAAA_AAAA);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xAAAA_AAAA_AAAA_AAAA,
        "Alternating pattern should be preserved");
}

// ============================================================================
// PSHUFW Tests: Edge Cases and Complex Patterns
// ============================================================================

#[test]
fn test_pshufw_sequential_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b10_00_11_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0001_0002_0003_0004);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0002_0004_0001_0003,
        "Should shuffle to pattern 2,0,3,1");
}

#[test]
fn test_pshufw_duplicate_selections() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_00_01_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xDDDD_DDDD_CCCC_CCCC,
        "Should duplicate words 0 and 1");
}

#[test]
fn test_pshufw_triple_duplicate() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b10_10_10_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_FEED_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFEED_FEED_FEED_3333,
        "Should triplicate word 2 and include word 1");
}

#[test]
fn test_pshufw_high_low_swap() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b01_00_11_10,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234_5678_ABCD_EF01);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xABCD_EF01_1234_5678,
        "Should swap high and low dwords");
}

#[test]
fn test_pshufw_rotate_left() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b10_01_00_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xBBBB_CCCC_DDDD_AAAA,
        "Should rotate left by one word");
}

#[test]
fn test_pshufw_rotate_right() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_11_10_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xDDDD_AAAA_BBBB_CCCC,
        "Should rotate right by one word");
}

// ============================================================================
// PSHUFW Tests: Byte Pattern Verification
// ============================================================================

#[test]
fn test_pshufw_byte_boundaries() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b00_01_10_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0102_0304_0506_0708);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0708_0506_0304_0102,
        "Word boundaries should be preserved");
}

#[test]
fn test_pshufw_signed_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b11_10_01_00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x8000_8001_8002_8003);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x8000_8001_8002_8003,
        "Signed-looking values should be treated as unsigned");
}

#[test]
fn test_pshufw_max_values() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b01_01_01_01,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFF_0000_FFFF_0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFFFF_FFFF_FFFF_FFFF,
        "Should broadcast 0xFFFF to all positions");
}

// ============================================================================
// PSHUFW Tests: Chain Operations
// ============================================================================

#[test]
fn test_pshufw_double_shuffle() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x70, 0xc0, 0b00_01_10_11,                 // PSHUFW MM0, MM0, reverse
        0x0f, 0x70, 0xc0, 0b00_01_10_11,                 // PSHUFW MM0, MM0, reverse again
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1234_5678_9ABC_DEF0);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x1234_5678_9ABC_DEF0,
        "Double reverse should restore original value");
}

#[test]
fn test_pshufw_with_emms() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x70, 0xc0, 0b11_11_11_11,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000_0000_BEEF_0000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0000_0000_0000_0000,
        "Should broadcast word 3 (0x0000)");
}
