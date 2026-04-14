use crate::*;

// VPMULHW/VPMULHUW - Multiply Packed Integers and Store High Result (AVX2)
//
// Performs SIMD multiply of packed word integers and stores the high half of the result.
//
// VPMULHW: Multiply 16 packed signed word integers (16-bit) and store high 16 bits
// VPMULHUW: Multiply 16 packed unsigned word integers (16-bit) and store high 16 bits
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG E5 /r     VPMULHW ymm1, ymm2, ymm3/m256  (signed)
// VEX.256.66.0F.WIG E4 /r     VPMULHUW ymm1, ymm2, ymm3/m256 (unsigned)

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMULHW Tests - 16x Signed Word Multiplication (256-bit, high result)
// ============================================================================

#[test]
fn test_vpmulhw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMULHW YMM0, YMM1, YMM2 (0 * 0 = 0, high = 0)
    let code = [
        0xc5, 0xf5, 0xe5, 0xc2, // VPMULHW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPMULHW YMM3, YMM4, YMM5 (1 * 1 = 1, high = 0)
    let code = [
        0xc5, 0xdd, 0xe5, 0xdd, // VPMULHW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm6_ymm7_ymm8_large_values() {
    let mut emu = emu64();
    // 0x8000 * 0x0002 = 0x00010000, high = 0x0001
    let code = [
        0xc5, 0x45, 0xe5, 0xf0, // VPMULHW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm9_ymm10_ymm11_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xe5, 0xcb, // VPMULHW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm12_ymm13_ymm14_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xe5, 0xe6, // VPMULHW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xe5, 0xf9, // VPMULHW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMULHW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe5, 0x00, // VPMULHW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16 * 0x1000).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm2_ymm3_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xe5, 0x10, // VPMULHW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm4_ymm5_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xe5, 0x20, // VPMULHW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..16).flat_map(|i| (1u16 << (i % 16)).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm6_ymm7_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xe5, 0x30, // VPMULHW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).wrapping_mul(0x4000).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm0_ymm1_ymm2_small_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe5, 0xc2, // VPMULHW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm3_ymm4_ymm5_negative_values() {
    let mut emu = emu64();
    // -1 * -1 = 1 (high = 0)
    let code = [
        0xc5, 0xdd, 0xe5, 0xdd, // VPMULHW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm8_ymm9_ymm10_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0xe5, 0xc2, // VPMULHW YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_ymm11_ymm12_ymm13_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xe5, 0xdd, // VPMULHW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe5, 0xc2, // VPMULHW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xe5, 0xc3, // VPMULHW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_mem_signed_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe5, 0x00, // VPMULHW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // -1 as signed 16-bit = 0xFFFF
    let data: Vec<u8> = (0..16).flat_map(|_| 0xFFFFu16.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMULHUW Tests - 16x Unsigned Word Multiplication (256-bit, high result)
// ============================================================================

#[test]
fn test_vpmulhuw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMULHUW YMM0, YMM1, YMM2 (0 * 0 = 0, high = 0)
    let code = [
        0xc5, 0xf5, 0xe4, 0xc2, // VPMULHUW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPMULHUW YMM3, YMM4, YMM5 (1 * 1 = 1, high = 0)
    let code = [
        0xc5, 0xdd, 0xe4, 0xdd, // VPMULHUW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm6_ymm7_ymm8_large_values() {
    let mut emu = emu64();
    // 0x8000 * 0x0002 = 0x00010000, high = 0x0001
    let code = [
        0xc5, 0x45, 0xe4, 0xf0, // VPMULHUW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm9_ymm10_ymm11_max_unsigned() {
    let mut emu = emu64();
    // 0xFFFF * 0xFFFF = 0xFFFE0001, high = 0xFFFE
    let code = [
        0xc4, 0x41, 0x2d, 0xe4, 0xcb, // VPMULHUW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm12_ymm13_ymm14_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xe4, 0xe6, // VPMULHUW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xe4, 0xf9, // VPMULHUW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMULHUW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe4, 0x00, // VPMULHUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16 * 0x1000).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm2_ymm3_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xe4, 0x10, // VPMULHUW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm4_ymm5_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xe4, 0x20, // VPMULHUW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..16).flat_map(|i| (1u16 << (i % 16)).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm6_ymm7_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xe4, 0x30, // VPMULHUW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).wrapping_mul(0x4000).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm0_ymm1_ymm2_small_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe4, 0xc2, // VPMULHUW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm3_ymm4_ymm5_half_max() {
    let mut emu = emu64();
    // 0x8000 * 0x8000 = 0x40000000, high = 0x4000
    let code = [
        0xc5, 0xdd, 0xe4, 0xdd, // VPMULHUW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm8_ymm9_ymm10_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0xe4, 0xc2, // VPMULHUW YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_ymm11_ymm12_ymm13_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xe4, 0xdd, // VPMULHUW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe4, 0xc2, // VPMULHUW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xe4, 0xc3, // VPMULHUW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_mem_high_bit_set() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe4, 0x00, // VPMULHUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|_| 0x8000u16.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_mem_different_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe4, 0x00, // VPMULHUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| {
        if i % 2 == 0 { 0xAAAAu16 } else { 0x5555u16 }.to_le_bytes()
    }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_vs_vpmulhuw_same_small_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe5, 0xc2, // VPMULHW YMM0, YMM1, YMM2
        0xc5, 0xed, 0xe4, 0xda, // VPMULHUW YMM3, YMM2, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhw_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe5, 0x00, // VPMULHW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulhuw_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe4, 0x00, // VPMULHUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
    emu.run(None).unwrap();
}
