use crate::*;

// VPSHUFD - Shuffle Packed Doublewords (AVX2)
//
// Shuffles the doublewords in the source operand according to an 8-bit immediate.
// Each 128-bit lane is shuffled independently.
//
// For each 128-bit lane:
// - Bits [1:0] of imm8 select which source dword goes to dword 0
// - Bits [3:2] of imm8 select which source dword goes to dword 1
// - Bits [5:4] of imm8 select which source dword goes to dword 2
// - Bits [7:6] of imm8 select which source dword goes to dword 3
//
// Each 2-bit selector can be 00, 01, 10, or 11 (selecting dword 0, 1, 2, or 3)
//
// Opcode: VEX.256.66.0F.WIG 70 /r ib    VPSHUFD ymm1, ymm2/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with identity shuffle (0xE4 = 11 10 01 00)
// ============================================================================

#[test]
fn test_vpshufd_ymm0_ymm1_identity() {
    let mut emu = emu64();
    // VPSHUFD YMM0, YMM1, 0xE4 (identity: 3,2,1,0)
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0xe4, // VPSHUFD YMM0, YMM1, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm3_ymm4_identity() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xdc, 0xe4, // VPSHUFD YMM3, YMM4, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm6_ymm7_identity() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xf7, 0xe4, // VPSHUFD YMM6, YMM7, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with reverse shuffle (0x1B = 00 01 10 11)
// ============================================================================

#[test]
fn test_vpshufd_ymm0_ymm1_reverse() {
    let mut emu = emu64();
    // VPSHUFD YMM0, YMM1, 0x1B (reverse: 0,1,2,3)
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x1b, // VPSHUFD YMM0, YMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm2_ymm3_reverse() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xd3, 0x1b, // VPSHUFD YMM2, YMM3, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm5_ymm6_reverse() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xee, 0x1b, // VPSHUFD YMM5, YMM6, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with broadcast dword 0 (0x00 = 00 00 00 00)
// ============================================================================

#[test]
fn test_vpshufd_ymm0_ymm1_broadcast_dword0() {
    let mut emu = emu64();
    // VPSHUFD YMM0, YMM1, 0x00 (broadcast dword 0)
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x00, // VPSHUFD YMM0, YMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm3_ymm4_broadcast_dword1() {
    let mut emu = emu64();
    // VPSHUFD YMM3, YMM4, 0x55 (broadcast dword 1: 01 01 01 01)
    let code = [
        0xc5, 0xfd, 0x70, 0xdc, 0x55, // VPSHUFD YMM3, YMM4, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm5_ymm6_broadcast_dword2() {
    let mut emu = emu64();
    // VPSHUFD YMM5, YMM6, 0xAA (broadcast dword 2: 10 10 10 10)
    let code = [
        0xc5, 0xfd, 0x70, 0xee, 0xaa, // VPSHUFD YMM5, YMM6, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm7_ymm0_broadcast_dword3() {
    let mut emu = emu64();
    // VPSHUFD YMM7, YMM0, 0xFF (broadcast dword 3: 11 11 11 11)
    let code = [
        0xc5, 0xfd, 0x70, 0xf8, 0xff, // VPSHUFD YMM7, YMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with swap pairs (0x4E = 01 00 11 10)
// ============================================================================

#[test]
fn test_vpshufd_ymm0_ymm1_swap_pairs() {
    let mut emu = emu64();
    // VPSHUFD YMM0, YMM1, 0x4E (swap low/high pairs: 1,0,3,2)
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x4e, // VPSHUFD YMM0, YMM1, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm2_ymm3_swap_pairs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xd3, 0x4e, // VPSHUFD YMM2, YMM3, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm4_ymm5_swap_pairs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xe5, 0x4e, // VPSHUFD YMM4, YMM5, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers (YMM8-YMM15)
// ============================================================================

#[test]
fn test_vpshufd_ymm8_ymm9_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xc1, 0xe4, // VPSHUFD YMM8, YMM9, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm10_ymm11_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xd3, 0x1b, // VPSHUFD YMM10, YMM11, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm12_ymm13_broadcast() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xe5, 0x00, // VPSHUFD YMM12, YMM13, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm14_ymm15_swap() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xf7, 0x4e, // VPSHUFD YMM14, YMM15, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm15_ymm8_custom() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xf8, 0x39, // VPSHUFD YMM15, YMM8, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory operands
// ============================================================================

#[test]
fn test_vpshufd_ymm0_mem_identity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfd, 0x70, 0x00, 0xe4, // VPSHUFD YMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm1_mem_reverse() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfd, 0x70, 0x08, 0x1b, // VPSHUFD YMM1, [RAX], 0x1B
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i * 2).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm2_mem_broadcast() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfd, 0x70, 0x10, 0x00, // VPSHUFD YMM2, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm3_mem_swap() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfd, 0x70, 0x18, 0x4e, // VPSHUFD YMM3, [RAX], 0x4E
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm4_mem_custom_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfd, 0x70, 0x20, 0x93, // VPSHUFD YMM4, [RAX], 0x93
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
fn test_vpshufd_ymm0_ymm1_imm_0x27() {
    let mut emu = emu64();
    // 0x27 = 00 10 01 11 -> 0,2,1,3
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x27, // VPSHUFD YMM0, YMM1, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x39() {
    let mut emu = emu64();
    // 0x39 = 00 11 10 01 -> 0,3,2,1
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x39, // VPSHUFD YMM0, YMM1, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x4B() {
    let mut emu = emu64();
    // 0x4B = 01 00 10 11 -> 1,0,2,3
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x4b, // VPSHUFD YMM0, YMM1, 0x4B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x72() {
    let mut emu = emu64();
    // 0x72 = 01 11 00 10 -> 1,3,0,2
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x72, // VPSHUFD YMM0, YMM1, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x8D() {
    let mut emu = emu64();
    // 0x8D = 10 00 11 01 -> 2,0,3,1
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x8d, // VPSHUFD YMM0, YMM1, 0x8D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x93() {
    let mut emu = emu64();
    // 0x93 = 10 01 00 11 -> 2,1,0,3
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x93, // VPSHUFD YMM0, YMM1, 0x93
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0xB1() {
    let mut emu = emu64();
    // 0xB1 = 10 11 00 01 -> 2,3,0,1
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0xb1, // VPSHUFD YMM0, YMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0xC6() {
    let mut emu = emu64();
    // 0xC6 = 11 00 01 10 -> 3,0,1,2
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0xc6, // VPSHUFD YMM0, YMM1, 0xC6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0xD8() {
    let mut emu = emu64();
    // 0xD8 = 11 01 10 00 -> 3,1,2,0
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0xd8, // VPSHUFD YMM0, YMM1, 0xD8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with duplicate patterns
// ============================================================================

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x44() {
    let mut emu = emu64();
    // 0x44 = 01 00 01 00 -> 1,0,1,0 (duplicate low pair)
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x44, // VPSHUFD YMM0, YMM1, 0x44
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0xEE() {
    let mut emu = emu64();
    // 0xEE = 11 10 11 10 -> 3,2,3,2 (duplicate high pair)
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0xee, // VPSHUFD YMM0, YMM1, 0xEE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0x50() {
    let mut emu = emu64();
    // 0x50 = 01 01 00 00 -> 1,1,0,0
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x50, // VPSHUFD YMM0, YMM1, 0x50
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_ymm0_ymm1_imm_0xFA() {
    let mut emu = emu64();
    // 0xFA = 11 11 10 10 -> 3,3,2,2
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0xfa, // VPSHUFD YMM0, YMM1, 0xFA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Chained operations and edge cases
// ============================================================================

#[test]
fn test_vpshufd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x1b, // VPSHUFD YMM0, YMM1, 0x1B
        0xc5, 0xfd, 0x70, 0xc0, 0x1b, // VPSHUFD YMM0, YMM0, 0x1B (should restore)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_same_register() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xc0, 0x4e, // VPSHUFD YMM0, YMM0, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_all_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xff, 0x27, // VPSHUFD YMM15, YMM15, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xfd, 0x70, 0x00, 0xe4, // VPSHUFD YMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_complex_pattern_1() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x6c, // VPSHUFD YMM0, YMM1, 0x6C
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_complex_pattern_2() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x9e, // VPSHUFD YMM0, YMM1, 0x9E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_complex_pattern_3() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xfd, 0x70, 0xc1, 0x2d, // VPSHUFD YMM0, YMM1, 0x2D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_with_ymm9_ymm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x7d, 0x70, 0xcc, 0xb1, // VPSHUFD YMM9, YMM12, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpshufd_mem_extended_reg() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x61, 0x7d, 0x70, 0x38, 0x93, // VPSHUFD YMM15, [RAX], 0x93
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
