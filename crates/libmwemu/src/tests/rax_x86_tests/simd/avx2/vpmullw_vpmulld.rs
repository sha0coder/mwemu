use crate::*;

// VPMULLW/VPMULLD - Multiply Packed Integers and Store Low Result (AVX2)
//
// Performs SIMD multiply of packed integers and stores the low half of the result.
//
// VPMULLW: Multiply 16 packed word integers (16-bit each) and store low 16 bits
// VPMULLD: Multiply 8 packed doubleword integers (32-bit each) and store low 32 bits
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG D5 /r     VPMULLW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 40 /r   VPMULLD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMULLW Tests - 16x Word Multiplication (256-bit, low result)
// ============================================================================

#[test]
fn test_vpmullw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMULLW YMM0, YMM1, YMM2 (0 * 0 = 0)
    let code = [
        0xc5, 0xf5, 0xd5, 0xc2, // VPMULLW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPMULLW YMM3, YMM4, YMM5 (1 * 1 = 1)
    let code = [
        0xc5, 0xdd, 0xd5, 0xdd, // VPMULLW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm6_ymm7_ymm8_by_two() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xd5, 0xf0, // VPMULLW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm9_ymm10_ymm11_overflow() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xd5, 0xcb, // VPMULLW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm12_ymm13_ymm14_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xd5, 0xe6, // VPMULLW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xd5, 0xf9, // VPMULLW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMULLW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm2_ymm3_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xd5, 0x10, // VPMULLW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm4_ymm5_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xd5, 0x20, // VPMULLW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..16).flat_map(|i| (1u16 << (i % 16)).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm6_ymm7_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xd5, 0x30, // VPMULLW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (1..=16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm0_ymm1_ymm2_small_values() {
    let mut emu = emu64();
    // 2 * 3 = 6
    let code = [
        0xc5, 0xf5, 0xd5, 0xc2, // VPMULLW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm3_ymm4_ymm5_negative_representation() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xd5, 0xdd, // VPMULLW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm8_ymm9_ymm10_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0xd5, 0xc2, // VPMULLW YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_ymm11_ymm12_ymm13_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xd5, 0xdd, // VPMULLW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xd5, 0xc2, // VPMULLW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xd5, 0xc3, // VPMULLW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_mem_different_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| {
        if i % 2 == 0 { 0x000Au16 } else { 0x0005u16 }.to_le_bytes()
    }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_mem_large_multipliers() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|_| 0x8000u16.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMULLD Tests - 8x Doubleword Multiplication (256-bit, low result)
// ============================================================================

#[test]
fn test_vpmulld_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMULLD YMM0, YMM1, YMM2 (0 * 0 = 0)
    let code = [
        0xc4, 0xe2, 0x75, 0x40, 0xc2, // VPMULLD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPMULLD YMM3, YMM4, YMM5 (1 * 1 = 1)
    let code = [
        0xc4, 0xe2, 0x5d, 0x40, 0xdd, // VPMULLD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm6_ymm7_ymm8_by_two() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x40, 0xf0, // VPMULLD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm9_ymm10_ymm11_overflow() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x40, 0xcb, // VPMULLD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm12_ymm13_ymm14_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x40, 0xe6, // VPMULLD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x7d, 0x40, 0xf9, // VPMULLD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMULLD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm2_ymm3_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x40, 0x10, // VPMULLD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm4_ymm5_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x40, 0x20, // VPMULLD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm6_ymm7_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xc2, 0x45, 0x40, 0x30, // VPMULLD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (1..=8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm0_ymm1_ymm2_small_values() {
    let mut emu = emu64();
    // 2 * 3 = 6
    let code = [
        0xc4, 0xe2, 0x75, 0x40, 0xc2, // VPMULLD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm3_ymm4_ymm5_negative_representation() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x40, 0xdd, // VPMULLD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm8_ymm9_ymm10_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x40, 0xc2, // VPMULLD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_ymm11_ymm12_ymm13_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x1d, 0x40, 0xdd, // VPMULLD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x40, 0xc2, // VPMULLD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x40, 0xc3, // VPMULLD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_mem_different_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..8).flat_map(|i| {
        if i % 2 == 0 { 10u32 } else { 5u32 }.to_le_bytes()
    }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_mem_large_multipliers() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|_| 0x80000000u32.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_mem_prime_numbers() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let primes: Vec<u8> = vec![2u32, 3, 5, 7, 11, 13, 17, 19]
        .into_iter()
        .flat_map(|p| p.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &primes);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmullw_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 2).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd5, 0x00, // VPMULLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmulld_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 4).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x40, 0x00, // VPMULLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02]);
    emu.run(None).unwrap();
}
