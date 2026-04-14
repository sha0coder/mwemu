use crate::*;

// VPAND/VPOR/VPXOR - Packed Bitwise Logical Operations (AVX2)
//
// Performs bitwise logical operations on 256-bit packed integers.
//
// VPAND: Bitwise AND of two 256-bit operands
// VPOR: Bitwise OR of two 256-bit operands
// VPXOR: Bitwise XOR of two 256-bit operands
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG DB /r     VPAND ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG EB /r     VPOR ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG EF /r     VPXOR ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPAND Tests - 256-bit Bitwise AND
// ============================================================================

#[test]
fn test_vpand_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPAND YMM0, YMM1, YMM2 (0 AND 0 = 0)
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPAND YMM3, YMM4, YMM5 (0xFF AND 0xFF = 0xFF)
    let code = [
        0xc5, 0xdd, 0xdb, 0xdd, // VPAND YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm6_ymm7_ymm8_masking() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xdb, 0xf0, // VPAND YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xdb, 0xcb, // VPAND YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    // 0xAA AND 0x55 = 0x00
    let code = [
        0xc4, 0x41, 0x15, 0xdb, 0xe6, // VPAND YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xdb, 0xf9, // VPAND YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPAND YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xdb, 0x00, // VPAND YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xdb, 0x10, // VPAND YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0xAA } else { 0x55 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xdb, 0x20, // VPAND YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let seq: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &seq);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xdb, 0xc3, // VPAND YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_identity() {
    let mut emu = emu64();
    // AND with all 1s (identity operation)
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_clear() {
    let mut emu = emu64();
    // AND with all 0s (clears result)
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_bit_mask_low_nibble() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpand_bit_mask_high_nibble() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPOR Tests - 256-bit Bitwise OR
// ============================================================================

#[test]
fn test_vpor_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPOR YMM0, YMM1, YMM2 (0 OR 0 = 0)
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPOR YMM3, YMM4, YMM5 (0xFF OR 0xFF = 0xFF)
    let code = [
        0xc5, 0xdd, 0xeb, 0xdd, // VPOR YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm6_ymm7_ymm8_combining() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xeb, 0xf0, // VPOR YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xeb, 0xcb, // VPOR YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    // 0xAA OR 0x55 = 0xFF
    let code = [
        0xc4, 0x41, 0x15, 0xeb, 0xe6, // VPOR YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xeb, 0xf9, // VPOR YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPOR YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xeb, 0x00, // VPOR YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xeb, 0x10, // VPOR YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0xAA } else { 0x55 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xeb, 0x20, // VPOR YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let seq: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &seq);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xeb, 0xc3, // VPOR YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_identity() {
    let mut emu = emu64();
    // OR with all 0s (identity operation)
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_set_all() {
    let mut emu = emu64();
    // OR with all 1s (sets all bits)
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpor_complement_bits() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xeb, 0xc2, // VPOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPXOR Tests - 256-bit Bitwise XOR
// ============================================================================

#[test]
fn test_vpxor_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPXOR YMM0, YMM1, YMM2 (0 XOR 0 = 0)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm3_ymm4_ymm5_same_values() {
    let mut emu = emu64();
    // VPXOR YMM3, YMM4, YMM5 (0xFF XOR 0xFF = 0x00)
    let code = [
        0xc5, 0xdd, 0xef, 0xdd, // VPXOR YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm6_ymm7_ymm8_toggling() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xef, 0xf0, // VPXOR YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xef, 0xcb, // VPXOR YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    // 0xAA XOR 0x55 = 0xFF
    let code = [
        0xc4, 0x41, 0x15, 0xef, 0xe6, // VPXOR YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xef, 0xf9, // VPXOR YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPXOR YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xef, 0x00, // VPXOR YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xef, 0x10, // VPXOR YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0xAA } else { 0x55 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xef, 0x20, // VPXOR YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let seq: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &seq);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xef, 0xc3, // VPXOR YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_self_clear() {
    let mut emu = emu64();
    // XOR with self (clears register)
    let code = [
        0xc5, 0xf5, 0xef, 0xc9, // VPXOR YMM1, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_identity() {
    let mut emu = emu64();
    // XOR with all 0s (identity operation)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_complement() {
    let mut emu = emu64();
    // XOR with all 1s (complements bits)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpxor_double_application() {
    let mut emu = emu64();
    // XOR twice with same value (returns to original)
    let code = [
        0xc5, 0xf5, 0xef, 0xc2, // VPXOR YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xef, 0xc2, // VPXOR YMM0, YMM0, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined tests
// ============================================================================

#[test]
fn test_vpand_vpor_vpxor_combined() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xdb, 0xc2, // VPAND YMM0, YMM1, YMM2
        0xc5, 0xed, 0xeb, 0xdb, // VPOR YMM3, YMM2, YMM3
        0xc5, 0xe5, 0xef, 0xe4, // VPXOR YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_logical_ops_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xdb, 0x00, // VPAND YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}
