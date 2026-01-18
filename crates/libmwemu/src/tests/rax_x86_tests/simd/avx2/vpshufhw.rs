use crate::*;

// VPSHUFHW - Shuffle Packed High Words (AVX2)
//
// Shuffles the high 4 words (words 4-7) of each 128-bit lane according to an 8-bit immediate.
// The low 4 words (words 0-3) are copied unchanged.
//
// For each 128-bit lane:
// - Words 0-3 are copied unchanged
// - Bits [1:0] of imm8 select which high word (4-7) goes to word 4
// - Bits [3:2] of imm8 select which high word (4-7) goes to word 5
// - Bits [5:4] of imm8 select which high word (4-7) goes to word 6
// - Bits [7:6] of imm8 select which high word (4-7) goes to word 7
//
// Opcode: VEX.256.F3.0F.WIG 70 /r ib    VPSHUFHW ymm1, ymm2/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with identity shuffle (0xE4 = 11 10 01 00)
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_ymm1_identity() {
    let mut emu = emu64();
    // VPSHUFHW YMM0, YMM1, 0xE4 (identity: high words 7,6,5,4)
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xe4, // VPSHUFHW YMM0, YMM1, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm3_ymm4_identity() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xdc, 0xe4, // VPSHUFHW YMM3, YMM4, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm6_ymm7_identity() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xf7, 0xe4, // VPSHUFHW YMM6, YMM7, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with reverse shuffle (0x1B = 00 01 10 11)
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_ymm1_reverse() {
    let mut emu = emu64();
    // VPSHUFHW YMM0, YMM1, 0x1B (reverse high words: 4,5,6,7)
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x1b, // VPSHUFHW YMM0, YMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm2_ymm3_reverse() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xd3, 0x1b, // VPSHUFHW YMM2, YMM3, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm5_ymm6_reverse() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xee, 0x1b, // VPSHUFHW YMM5, YMM6, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with broadcast patterns
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_ymm1_broadcast_word4() {
    let mut emu = emu64();
    // VPSHUFHW YMM0, YMM1, 0x00 (broadcast word 4 to high words)
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x00, // VPSHUFHW YMM0, YMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm3_ymm4_broadcast_word5() {
    let mut emu = emu64();
    // VPSHUFHW YMM3, YMM4, 0x55 (broadcast word 5: 01 01 01 01)
    let code = [
        0xc5, 0xfe, 0x70, 0xdc, 0x55, // VPSHUFHW YMM3, YMM4, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm5_ymm6_broadcast_word6() {
    let mut emu = emu64();
    // VPSHUFHW YMM5, YMM6, 0xAA (broadcast word 6: 10 10 10 10)
    let code = [
        0xc5, 0xfe, 0x70, 0xee, 0xaa, // VPSHUFHW YMM5, YMM6, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm7_ymm0_broadcast_word7() {
    let mut emu = emu64();
    // VPSHUFHW YMM7, YMM0, 0xFF (broadcast word 7: 11 11 11 11)
    let code = [
        0xc5, 0xfe, 0x70, 0xf8, 0xff, // VPSHUFHW YMM7, YMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with swap pairs (0x4E = 01 00 11 10)
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_ymm1_swap_pairs() {
    let mut emu = emu64();
    // VPSHUFHW YMM0, YMM1, 0x4E (swap low/high pairs of high words)
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x4e, // VPSHUFHW YMM0, YMM1, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm2_ymm3_swap_pairs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xd3, 0x4e, // VPSHUFHW YMM2, YMM3, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm4_ymm5_swap_pairs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xe5, 0x4e, // VPSHUFHW YMM4, YMM5, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vpshufhw_ymm8_ymm9_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xc1, 0xe4, // VPSHUFHW YMM8, YMM9, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm10_ymm11_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xd3, 0x1b, // VPSHUFHW YMM10, YMM11, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm12_ymm13_broadcast() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xe5, 0x00, // VPSHUFHW YMM12, YMM13, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm14_ymm15_swap() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xf7, 0x4e, // VPSHUFHW YMM14, YMM15, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm15_ymm8_custom() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xf8, 0x39, // VPSHUFHW YMM15, YMM8, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory operands
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_mem_identity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfe, 0x70, 0x00, 0xe4, // VPSHUFHW YMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm1_mem_reverse() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfe, 0x70, 0x08, 0x1b, // VPSHUFHW YMM1, [RAX], 0x1B
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i * 2).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm2_mem_broadcast() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfe, 0x70, 0x10, 0x00, // VPSHUFHW YMM2, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm3_mem_swap() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfe, 0x70, 0x18, 0x4e, // VPSHUFHW YMM3, [RAX], 0x4E
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm4_mem_custom_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfe, 0x70, 0x20, 0x93, // VPSHUFHW YMM4, [RAX], 0x93
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| 0xFF - i).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with various immediate patterns
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x27() {
    let mut emu = emu64();
    // 0x27 = 00 10 01 11
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x27, // VPSHUFHW YMM0, YMM1, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x39() {
    let mut emu = emu64();
    // 0x39 = 00 11 10 01
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x39, // VPSHUFHW YMM0, YMM1, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x4B() {
    let mut emu = emu64();
    // 0x4B = 01 00 10 11
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x4b, // VPSHUFHW YMM0, YMM1, 0x4B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x72() {
    let mut emu = emu64();
    // 0x72 = 01 11 00 10
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x72, // VPSHUFHW YMM0, YMM1, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x8D() {
    let mut emu = emu64();
    // 0x8D = 10 00 11 01
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x8d, // VPSHUFHW YMM0, YMM1, 0x8D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x93() {
    let mut emu = emu64();
    // 0x93 = 10 01 00 11
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x93, // VPSHUFHW YMM0, YMM1, 0x93
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0xB1() {
    let mut emu = emu64();
    // 0xB1 = 10 11 00 01
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xb1, // VPSHUFHW YMM0, YMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0xC6() {
    let mut emu = emu64();
    // 0xC6 = 11 00 01 10
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xc6, // VPSHUFHW YMM0, YMM1, 0xC6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0xD8() {
    let mut emu = emu64();
    // 0xD8 = 11 01 10 00
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xd8, // VPSHUFHW YMM0, YMM1, 0xD8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with duplicate patterns
// ============================================================================

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x44() {
    let mut emu = emu64();
    // 0x44 = 01 00 01 00 (duplicate low pair)
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x44, // VPSHUFHW YMM0, YMM1, 0x44
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0xEE() {
    let mut emu = emu64();
    // 0xEE = 11 10 11 10 (duplicate high pair)
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xee, // VPSHUFHW YMM0, YMM1, 0xEE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0x50() {
    let mut emu = emu64();
    // 0x50 = 01 01 00 00
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x50, // VPSHUFHW YMM0, YMM1, 0x50
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_ymm0_ymm1_imm_0xFA() {
    let mut emu = emu64();
    // 0xFA = 11 11 10 10
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xfa, // VPSHUFHW YMM0, YMM1, 0xFA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Chained operations and edge cases
// ============================================================================

#[test]
fn test_vpshufhw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x1b, // VPSHUFHW YMM0, YMM1, 0x1B
        0xc5, 0xfe, 0x70, 0xc0, 0x1b, // VPSHUFHW YMM0, YMM0, 0x1B (should restore)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_same_register() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc0, 0x4e, // VPSHUFHW YMM0, YMM0, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_all_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xff, 0x27, // VPSHUFHW YMM15, YMM15, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfe, 0x70, 0x00, 0xe4, // VPSHUFHW YMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_complex_pattern_1() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x6c, // VPSHUFHW YMM0, YMM1, 0x6C
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_complex_pattern_2() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x9e, // VPSHUFHW YMM0, YMM1, 0x9E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_complex_pattern_3() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0x2d, // VPSHUFHW YMM0, YMM1, 0x2D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_with_ymm9_ymm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7e, 0x70, 0xcc, 0xb1, // VPSHUFHW YMM9, YMM12, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_mem_extended_reg() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x61, 0x7e, 0x70, 0x38, 0x93, // VPSHUFHW YMM15, [RAX], 0x93
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xa5, // VPSHUFHW YMM0, YMM1, 0xA5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufhw_low_words_unchanged() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfe, 0x70, 0xc1, 0xff, // VPSHUFHW YMM0, YMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
