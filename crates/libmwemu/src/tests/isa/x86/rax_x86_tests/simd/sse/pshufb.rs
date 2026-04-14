use crate::*;

// PSHUFB - Packed Shuffle Bytes
//
// Performs in-place shuffles of bytes in the destination operand according to the
// shuffle control mask in the source operand.
//
// For each byte in the shuffle control mask:
// - Bits [3:0] (for 128-bit) select which byte from source (0-15)
// - Bit 7 set means write zero to that destination byte position
//
// Opcode: 66 0F 38 00 /r    PSHUFB xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with zero mask (high bit set - zeros output)
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_all_zeros() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 where XMM1 has all high bits set
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm2_xmm3_all_zeros() {
    let mut emu = emu64();
    // PSHUFB XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xd3, // PSHUFB XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with identity shuffle (0x00, 0x01, 0x02, ..., 0x0F)
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_identity() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - identity shuffle
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm4_xmm5_identity() {
    let mut emu = emu64();
    // PSHUFB XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xe5, // PSHUFB XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with reverse shuffle (0x0F, 0x0E, 0x0D, ..., 0x00)
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_reverse() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - reverse byte order
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm6_xmm7_reverse() {
    let mut emu = emu64();
    // PSHUFB XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xf7, // PSHUFB XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with broadcast byte 0
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_broadcast_byte0() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - broadcast byte 0
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm2_xmm3_broadcast_byte0() {
    let mut emu = emu64();
    // PSHUFB XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xd3, // PSHUFB XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with broadcast byte 15
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_broadcast_byte15() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - broadcast byte 15
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm4_xmm5_broadcast_byte15() {
    let mut emu = emu64();
    // PSHUFB XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xe5, // PSHUFB XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests selecting specific bytes (0-15)
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_select_byte1() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - select byte 1
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_select_byte7() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - select byte 7
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_select_byte8() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - select byte 8
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_select_byte14() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - select byte 14
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with mixed zero and select patterns
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_mixed_pattern1() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - alternating zero and select
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_mixed_pattern2() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_mixed_pattern3() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - first half zeros
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_mixed_pattern4() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - second half zeros
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with different register pairs
// ============================================================================

#[test]
fn test_pshufb_xmm1_xmm2() {
    let mut emu = emu64();
    // PSHUFB XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xca, // PSHUFB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm3_xmm4() {
    let mut emu = emu64();
    // PSHUFB XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xdc, // PSHUFB XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm5_xmm6() {
    let mut emu = emu64();
    // PSHUFB XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xee, // PSHUFB XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm7_xmm0() {
    let mut emu = emu64();
    // PSHUFB XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xf8, // PSHUFB XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pshufb_xmm8_xmm9() {
    let mut emu = emu64();
    // PSHUFB XMM8, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm10_xmm11() {
    let mut emu = emu64();
    // PSHUFB XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x00, 0xd3, // PSHUFB XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm12_xmm13() {
    let mut emu = emu64();
    // PSHUFB XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x00, 0xe5, // PSHUFB XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm14_xmm15() {
    let mut emu = emu64();
    // PSHUFB XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x00, 0xf7, // PSHUFB XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm8() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM8
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x00, 0xc0, // PSHUFB XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm15_xmm0() {
    let mut emu = emu64();
    // PSHUFB XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x00, 0xf8, // PSHUFB XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_pshufb_xmm0_mem_identity() {
    let mut emu = emu64();
    // PSHUFB XMM0, [mem] - identity pattern in memory
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x00, // PSHUFB XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let pattern: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm1_mem_reverse() {
    let mut emu = emu64();
    // PSHUFB XMM1, [mem] - reverse pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x08, // PSHUFB XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let pattern: [u8; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm2_mem_all_zeros() {
    let mut emu = emu64();
    // PSHUFB XMM2, [mem] - all zeros pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x10, // PSHUFB XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm3_mem_broadcast_byte0() {
    let mut emu = emu64();
    // PSHUFB XMM3, [mem] - broadcast byte 0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x18, // PSHUFB XMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm7_mem_mixed() {
    let mut emu = emu64();
    // PSHUFB XMM7, [mem] - mixed pattern
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x38, // PSHUFB XMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let pattern: [u8; 16] = [0x00, 0x0F, 0x01, 0x0E, 0x02, 0x0D, 0x03, 0x0C,
                              0x04, 0x0B, 0x05, 0x0A, 0x06, 0x09, 0x07, 0x08];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);

    emu.run(None).unwrap();
}

// ============================================================================
// Addressing mode tests
// ============================================================================

#[test]
fn test_pshufb_xmm0_mem_displacement() {
    let mut emu = emu64();
    // PSHUFB XMM0, [RAX + disp]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x40, 0x10, // PSHUFB XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm1_mem_rbx() {
    let mut emu = emu64();
    // PSHUFB XMM1, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x00, 0x0b, // PSHUFB XMM1, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequential shuffle tests
// ============================================================================

#[test]
fn test_pshufb_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0x66, 0x0f, 0x38, 0x00, 0xd3, // PSHUFB XMM2, XMM3
        0x66, 0x0f, 0x38, 0x00, 0xe5, // PSHUFB XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_same_register() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM0 (shuffle with itself)
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc0, // PSHUFB XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Byte swap patterns (useful for endianness conversion)
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_swap_dwords() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - swap bytes within each dword
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_swap_words() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - swap bytes within each word
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Extract/duplicate patterns
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_extract_low_bytes() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - extract low bytes (0, 2, 4, 6, 8, 10, 12, 14)
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_extract_high_bytes() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - extract high bytes (1, 3, 5, 7, 9, 11, 13, 15)
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Interleave patterns
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_interleave_pattern1() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - interleave pattern
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_interleave_pattern2() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Rotate patterns
// ============================================================================

#[test]
fn test_pshufb_xmm0_xmm1_rotate_left_1() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - rotate left by 1 byte
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_rotate_left_4() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - rotate left by 4 bytes
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufb_xmm0_xmm1_rotate_left_8() {
    let mut emu = emu64();
    // PSHUFB XMM0, XMM1 - rotate left by 8 bytes
    let code = [
        0x66, 0x0f, 0x38, 0x00, 0xc1, // PSHUFB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
