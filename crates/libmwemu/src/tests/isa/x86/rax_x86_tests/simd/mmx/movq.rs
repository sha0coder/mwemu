//! Tests for the MOVQ instruction (MMX).
//!
//! MOVQ - Move Quadword
//!
//! Copies a quadword from the source operand (second operand) to the destination
//! operand (first operand). The source and destination operands can be MMX technology
//! registers or 64-bit memory locations. This instruction can be used to move a
//! quadword between two MMX technology registers or between an MMX technology register
//! and a 64-bit memory location.
//!
//! Opcodes:
//! - NP 0F 6F /r: MOVQ mm, mm/m64  (load from memory or register to MMX register)
//! - NP 0F 7F /r: MOVQ mm/m64, mm  (store from MMX register to memory or register)
//!
//! Operation: DEST := SRC
//!
//! Flags affected: None
//!
//! Reference: /Users/int/dev/rax/docs/movq.txt

use crate::*;

// ============================================================================
// MOVQ Tests: Register to Register
// ============================================================================

#[test]
fn test_movq_mm0_mm1() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x0c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2000]
        0x0f, 0x6f, 0xc1,                               // MOVQ MM0, MM1
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0123456789ABCDEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0123456789ABCDEF,
        "MOVQ MM0, MM1 should copy value");
}

#[test]
fn test_movq_mm2_mm3() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x1c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2000]
        0x0f, 0x6f, 0xd3,                               // MOVQ MM2, MM3
        0x0f, 0x7f, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFEDCBA9876543210);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFEDCBA9876543210,
        "MOVQ MM2, MM3 should copy value");
}

#[test]
fn test_movq_mm4_mm5() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x2c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM5, [0x2000]
        0x0f, 0x6f, 0xe5,                               // MOVQ MM4, MM5
        0x0f, 0x7f, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM4
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAA_BBBB_CCCC_DDDD);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xAAAA_BBBB_CCCC_DDDD,
        "MOVQ MM4, MM5 should copy value");
}

#[test]
fn test_movq_mm6_mm7() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM7, [0x2000]
        0x0f, 0x6f, 0xf7,                               // MOVQ MM6, MM7
        0x0f, 0x7f, 0x34, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM6
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111_2222_3333_4444);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x1111_2222_3333_4444,
        "MOVQ MM6, MM7 should copy value");
}

#[test]
fn test_movq_same_register() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0xc0,                               // MOVQ MM0, MM0
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xCAFEBABEDEADBEEF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xCAFEBABEDEADBEEF,
        "MOVQ MM0, MM0 should preserve value");
}

// ============================================================================
// MOVQ Tests: Memory to Register
// ============================================================================

#[test]
fn test_movq_mm0_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0011223344556677);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0011223344556677,
        "MOVQ MM0, [mem] should load from memory");
}

#[test]
fn test_movq_mm1_m64() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x7f, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM1
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2008, 0x8877665544332211);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x8877665544332211,
        "MOVQ MM1, [mem] should load from memory");
}

#[test]
fn test_movq_mm_m64_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0000000000000000,
        "MOVQ should load all zeros");
}

#[test]
fn test_movq_mm_m64_all_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF,
        "MOVQ should load all ones");
}

#[test]
fn test_movq_mm_m64_alternating() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xAAAAAAAAAAAAAAAA,
        "MOVQ should load alternating pattern");
}

// ============================================================================
// MOVQ Tests: Register to Memory
// ============================================================================

#[test]
fn test_movq_m64_mm0() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x7f, 0x04, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVQ [0x3008], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x123456789ABCDEF0);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3008).unwrap();
    assert_eq!(result, 0x123456789ABCDEF0,
        "MOVQ [mem], MM0 should store to memory");
}

#[test]
fn test_movq_m64_mm7() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x3c, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM7, [0x2000]
        0x0f, 0x7f, 0x3c, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVQ [0x3010], MM7
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFEDCBA9876543210);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3010).unwrap();
    assert_eq!(result, 0xFEDCBA9876543210,
        "MOVQ [mem], MM7 should store to memory");
}

#[test]
fn test_movq_store_all_zeros() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000000000000000);
    emu.maps.write_qword(0x3000, 0xFFFFFFFFFFFFFFFF); // Pre-fill with ones

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0000000000000000,
        "MOVQ should store all zeros");
}

#[test]
fn test_movq_store_all_ones() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);
    emu.maps.write_qword(0x3000, 0x0000000000000000); // Pre-fill with zeros

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF,
        "MOVQ should store all ones");
}

// ============================================================================
// MOVQ Tests: Memory to Memory (via Register)
// ============================================================================

#[test]
fn test_movq_mem_to_mem() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xDEADBEEFCAFEBABE);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xDEADBEEFCAFEBABE,
        "MOVQ should transfer data from mem to mem via register");
}

#[test]
fn test_movq_multiple_transfers() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x7f, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVQ [0x3008], MM1
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111111111111111);
    emu.maps.write_qword(0x2008, 0x2222222222222222);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_qword(0x3000).unwrap();
    let result2 = emu.maps.read_qword(0x3008).unwrap();
    assert_eq!(result1, 0x1111111111111111, "First MOVQ transfer");
    assert_eq!(result2, 0x2222222222222222, "Second MOVQ transfer");
}

// ============================================================================
// MOVQ Tests: Chain of Register Transfers
// ============================================================================

#[test]
fn test_movq_register_chain() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0xc8,                               // MOVQ MM1, MM0
        0x0f, 0x6f, 0xd1,                               // MOVQ MM2, MM1
        0x0f, 0x6f, 0xda,                               // MOVQ MM3, MM2
        0x0f, 0x7f, 0x1c, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM3
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xBEEFCAFEDEADC0DE);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xBEEFCAFEDEADC0DE,
        "Value should propagate through register chain");
}

#[test]
fn test_movq_circular_move() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x6f, 0xd0,                               // MOVQ MM2, MM0 (temp)
        0x0f, 0x6f, 0xc1,                               // MOVQ MM0, MM1
        0x0f, 0x6f, 0xca,                               // MOVQ MM1, MM2
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0x0f, 0x7f, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVQ [0x3008], MM1
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0xBBBBBBBBBBBBBBBB);

    emu.run(None).unwrap();

    let result1 = emu.maps.read_qword(0x3000).unwrap();
    let result2 = emu.maps.read_qword(0x3008).unwrap();
    assert_eq!(result1, 0xBBBBBBBBBBBBBBBB, "MM0 should have MM1's value");
    assert_eq!(result2, 0xAAAAAAAAAAAAAAAA, "MM1 should have MM0's value");
}

// ============================================================================
// MOVQ Tests: Edge Cases and Special Values
// ============================================================================

#[test]
fn test_movq_high_bit_set() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x8000000000000000);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x8000000000000000,
        "MOVQ should handle high bit set");
}

#[test]
fn test_movq_max_value() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF,
        "MOVQ should handle max value");
}

#[test]
fn test_movq_one() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0000000000000001);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0000000000000001,
        "MOVQ should handle value 1");
}

#[test]
fn test_movq_power_of_two() {
    let mut emu = emu64();
    let test_values = [
        0x0000000000000001, // 2^0
        0x0000000000000002, // 2^1
        0x0000000000000100, // 2^8
        0x0000000000010000, // 2^16
        0x0000000100000000, // 2^32
        0x8000000000000000, // 2^63
    ];

    for (i, &value) in test_values.iter().enumerate() {
        let code = vec![
            0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
            0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
            0xf4,
        ];

        emu.load_code_bytes(&code);
        emu.maps.write_qword(0x2000, value);

    emu.run(None).unwrap();

        let result = emu.maps.read_qword(0x3000).unwrap();
        assert_eq!(result, value,
            "MOVQ should handle power of two #{}", i);
    }
}

// ============================================================================
// MOVQ Tests: Byte Patterns
// ============================================================================

#[test]
fn test_movq_sequential_bytes() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x0001020304050607);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x0001020304050607,
        "MOVQ should preserve sequential bytes");
}

#[test]
fn test_movq_repeating_bytes() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00,
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00,
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1212121212121212);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x1212121212121212,
        "MOVQ should preserve repeating bytes");
}

// ============================================================================
// MOVQ Tests: Multiple Registers
// ============================================================================

#[test]
fn test_movq_all_registers() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2010]
        0x0f, 0x6f, 0x1c, 0x25, 0x18, 0x20, 0x00, 0x00, // MOVQ MM3, [0x2018]
        0x0f, 0x6f, 0x24, 0x25, 0x20, 0x20, 0x00, 0x00, // MOVQ MM4, [0x2020]
        0x0f, 0x6f, 0x2c, 0x25, 0x28, 0x20, 0x00, 0x00, // MOVQ MM5, [0x2028]
        0x0f, 0x6f, 0x34, 0x25, 0x30, 0x20, 0x00, 0x00, // MOVQ MM6, [0x2030]
        0x0f, 0x6f, 0x3c, 0x25, 0x38, 0x20, 0x00, 0x00, // MOVQ MM7, [0x2038]
        // Store them back
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0x0f, 0x7f, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVQ [0x3008], MM1
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVQ [0x3010], MM2
        0x0f, 0x7f, 0x1c, 0x25, 0x18, 0x30, 0x00, 0x00, // MOVQ [0x3018], MM3
        0x0f, 0x7f, 0x24, 0x25, 0x20, 0x30, 0x00, 0x00, // MOVQ [0x3020], MM4
        0x0f, 0x7f, 0x2c, 0x25, 0x28, 0x30, 0x00, 0x00, // MOVQ [0x3028], MM5
        0x0f, 0x7f, 0x34, 0x25, 0x30, 0x30, 0x00, 0x00, // MOVQ [0x3030], MM6
        0x0f, 0x7f, 0x3c, 0x25, 0x38, 0x30, 0x00, 0x00, // MOVQ [0x3038], MM7
        0xf4,
    ];

    emu.load_code_bytes(&code);
    for i in 0..8 {
        emu.maps.write_qword(0x2000 + i * 8, 0x1111 * (i + 1) as u64);
    }

    emu.run(None).unwrap();

    for i in 0..8 {
        let result = emu.maps.read_qword(0x3000 + i * 8).unwrap();
        assert_eq!(result, 0x1111 * (i + 1) as u64,
            "MM{} value should be preserved", i);
    }
}

// ============================================================================
// MOVQ Tests: Interaction with Other Instructions
// ============================================================================

#[test]
fn test_movq_with_emms() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0x0f, 0x77,                                      // EMMS
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x123456789ABCDEF0);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x123456789ABCDEF0,
        "MOVQ should work correctly before EMMS");
}

#[test]
fn test_movq_overwrite() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x04, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2008] (overwrite)
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0x1111111111111111);
    emu.maps.write_qword(0x2008, 0x2222222222222222);

    emu.run(None).unwrap();

    let result = emu.maps.read_qword(0x3000).unwrap();
    assert_eq!(result, 0x2222222222222222,
        "Second MOVQ should overwrite first value");
}

#[test]
fn test_movq_load_store_independence() {
    let mut emu = emu64();
    let code = vec![
        0x0f, 0x6f, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // MOVQ MM0, [0x2000]
        0x0f, 0x6f, 0x0c, 0x25, 0x08, 0x20, 0x00, 0x00, // MOVQ MM1, [0x2008]
        0x0f, 0x6f, 0x14, 0x25, 0x10, 0x20, 0x00, 0x00, // MOVQ MM2, [0x2010]
        0x0f, 0x7f, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // MOVQ [0x3000], MM0
        0x0f, 0x7f, 0x0c, 0x25, 0x08, 0x30, 0x00, 0x00, // MOVQ [0x3008], MM1
        0x0f, 0x7f, 0x14, 0x25, 0x10, 0x30, 0x00, 0x00, // MOVQ [0x3010], MM2
        0xf4,
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_qword(0x2000, 0xAAAAAAAAAAAAAAAA);
    emu.maps.write_qword(0x2008, 0xBBBBBBBBBBBBBBBB);
    emu.maps.write_qword(0x2010, 0xCCCCCCCCCCCCCCCC);

    emu.run(None).unwrap();

    let result0 = emu.maps.read_qword(0x3000).unwrap();
    let result1 = emu.maps.read_qword(0x3008).unwrap();
    let result2 = emu.maps.read_qword(0x3010).unwrap();

    assert_eq!(result0, 0xAAAAAAAAAAAAAAAA, "MM0 value");
    assert_eq!(result1, 0xBBBBBBBBBBBBBBBB, "MM1 value");
    assert_eq!(result2, 0xCCCCCCCCCCCCCCCC, "MM2 value");
}
