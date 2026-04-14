use crate::*;

// VPSHUFB - Packed Shuffle Bytes (AVX2)
//
// Performs in-place shuffles of bytes in the destination operand according to the
// shuffle control mask in the source operand. Each 128-bit lane is shuffled independently.
//
// For each byte in the shuffle control mask:
// - Bits [3:0] select which byte from the corresponding 128-bit lane (0-15)
// - Bit 7 set means write zero to that destination byte position
//
// Opcode: VEX.256.66.0F38.WIG 00 /r    VPSHUFB ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with zero mask (high bit set - zeros output)
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPSHUFB YMM0, YMM1, YMM2 where YMM2 has all high bits set
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm6_ymm7_ymm8_partial_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x00, 0xf0, // VPSHUFB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with identity shuffle (0x00, 0x01, 0x02, ..., 0x0F for each lane)
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_identity() {
    let mut emu = emu64();
    // VPSHUFB YMM0, YMM1, YMM2 - identity shuffle
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm9_ymm10_ymm11_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x00, 0xcb, // VPSHUFB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with reverse shuffle (0x0F, 0x0E, 0x0D, ..., 0x00 for each lane)
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_reverse() {
    let mut emu = emu64();
    // VPSHUFB YMM0, YMM1, YMM2 - reverse byte order in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm12_ymm13_ymm14_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x00, 0xe6, // VPSHUFB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with broadcast byte 0 from each lane
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_broadcast_byte0() {
    let mut emu = emu64();
    // VPSHUFB YMM0, YMM1, YMM2 - broadcast byte 0 in each lane
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_broadcast_byte7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm6_ymm7_ymm8_broadcast_byte15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x00, 0xf0, // VPSHUFB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with alternating patterns
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_ymm2_even_bytes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm3_ymm4_ymm5_odd_bytes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x00, 0xdd, // VPSHUFB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm9_ymm10_ymm11_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x00, 0xcb, // VPSHUFB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vpshufb_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x00, 0xc2, // VPSHUFB YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x1d, 0x00, 0xdd, // VPSHUFB YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x05, 0x00, 0xf0, // VPSHUFB YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x7d, 0x00, 0xf9, // VPSHUFB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory operands
// ============================================================================

#[test]
fn test_vpshufb_ymm0_ymm1_mem_identity() {
    let mut emu = emu64();
    // VPSHUFB YMM0, YMM1, [memory] - identity shuffle from memory
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x00, 0x00, // VPSHUFB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut pattern = Vec::new();
    pattern.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    pattern.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm2_ymm3_mem_reverse() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x00, 0x10, // VPSHUFB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut pattern = Vec::new();
    pattern.extend_from_slice(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    pattern.extend_from_slice(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm4_ymm5_mem_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x00, 0x20, // VPSHUFB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm6_ymm7_mem_broadcast() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x00, 0x30, // VPSHUFB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_ymm8_ymm9_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x35, 0x00, 0x00, // VPSHUFB YMM8, YMM9, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![
        0, 0, 0, 0, 15, 14, 13, 12, 0x80, 0x80, 0x80, 0x80, 3, 2, 1, 0,
        0, 0, 0, 0, 15, 14, 13, 12, 0x80, 0x80, 0x80, 0x80, 3, 2, 1, 0,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with chained operations
// ============================================================================

#[test]
fn test_vpshufb_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x00, 0xc3, // VPSHUFB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_double_shuffle() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x00, 0xc3, // VPSHUFB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with specific shuffle patterns
// ============================================================================

#[test]
fn test_vpshufb_swap_pairs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_swap_nibbles() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_duplicate_first_half() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_duplicate_second_half() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_rotate_left_1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_rotate_right_1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_extract_low_nibbles() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_interleave_bytes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_deinterleave_bytes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x00, 0xc2, // VPSHUFB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with lane-specific patterns
// ============================================================================

#[test]
fn test_vpshufb_different_per_lane() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x00, 0x00, // VPSHUFB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut pattern = Vec::new();
    pattern.extend_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    pattern.extend_from_slice(&[15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufb_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x00, 0x00, // VPSHUFB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}
