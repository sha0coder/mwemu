use crate::*;

// VPCMPEQB/VPCMPEQW/VPCMPEQD/VPCMPEQQ - Compare Packed Integers for Equality (AVX2)
//
// Performs SIMD comparison of packed integers for equality.
// For each element, if equal, all bits in result element are set to 1, otherwise 0.
//
// VPCMPEQB: Compare 32 packed byte integers (8-bit each)
// VPCMPEQW: Compare 16 packed word integers (16-bit each)
// VPCMPEQD: Compare 8 packed doubleword integers (32-bit each)
// VPCMPEQQ: Compare 4 packed quadword integers (64-bit each)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 74 /r     VPCMPEQB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 75 /r     VPCMPEQW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 76 /r     VPCMPEQD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 29 /r   VPCMPEQQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPCMPEQB Tests - 32x Byte Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqb_ymm0_ymm1_ymm2_all_equal() {
    let mut emu = emu64();
    // VPCMPEQB YMM0, YMM1, YMM2 - all bytes equal
    let code = [
        0xc5, 0xf5, 0x74, 0xc2, // VPCMPEQB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm3_ymm4_ymm5_all_different() {
    let mut emu = emu64();
    // VPCMPEQB YMM3, YMM4, YMM5 - all bytes different
    let code = [
        0xc5, 0xdd, 0x74, 0xdd, // VPCMPEQB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x74, 0xf0, // VPCMPEQB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm9_ymm10_ymm11_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x74, 0xcb, // VPCMPEQB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x74, 0xe6, // VPCMPEQB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x74, 0xf9, // VPCMPEQB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPEQB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x74, 0x00, // VPCMPEQB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x74, 0x10, // VPCMPEQB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_ymm4_ymm5_mem_all_same() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x74, 0x20, // VPCMPEQB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x74, 0xc1, // VPCMPEQB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPCMPEQW Tests - 16x Word Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqw_ymm0_ymm1_ymm2_all_equal() {
    let mut emu = emu64();
    // VPCMPEQW YMM0, YMM1, YMM2 - all words equal
    let code = [
        0xc5, 0xf5, 0x75, 0xc2, // VPCMPEQW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm3_ymm4_ymm5_all_different() {
    let mut emu = emu64();
    // VPCMPEQW YMM3, YMM4, YMM5 - all words different
    let code = [
        0xc5, 0xdd, 0x75, 0xdd, // VPCMPEQW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x75, 0xf0, // VPCMPEQW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm9_ymm10_ymm11_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x75, 0xcb, // VPCMPEQW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x75, 0xe6, // VPCMPEQW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x75, 0xf9, // VPCMPEQW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPEQW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x75, 0x00, // VPCMPEQW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16 * 0x1111).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x75, 0x10, // VPCMPEQW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_ymm4_ymm5_mem_all_same() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x75, 0x20, // VPCMPEQW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x75, 0xc1, // VPCMPEQW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPCMPEQD Tests - 8x Doubleword Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqd_ymm0_ymm1_ymm2_all_equal() {
    let mut emu = emu64();
    // VPCMPEQD YMM0, YMM1, YMM2 - all dwords equal
    let code = [
        0xc5, 0xf5, 0x76, 0xc2, // VPCMPEQD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm3_ymm4_ymm5_all_different() {
    let mut emu = emu64();
    // VPCMPEQD YMM3, YMM4, YMM5 - all dwords different
    let code = [
        0xc5, 0xdd, 0x76, 0xdd, // VPCMPEQD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x76, 0xf0, // VPCMPEQD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm9_ymm10_ymm11_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x76, 0xcb, // VPCMPEQD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x76, 0xe6, // VPCMPEQD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x76, 0xf9, // VPCMPEQD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPEQD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x76, 0x00, // VPCMPEQD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32 * 0x11111111).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x76, 0x10, // VPCMPEQD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_ymm4_ymm5_mem_all_same() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x76, 0x20, // VPCMPEQD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x76, 0xc1, // VPCMPEQD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPCMPEQQ Tests - 4x Quadword Equality Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpeqq_ymm0_ymm1_ymm2_all_equal() {
    let mut emu = emu64();
    // VPCMPEQQ YMM0, YMM1, YMM2 - all qwords equal
    let code = [
        0xc4, 0xe2, 0x75, 0x29, 0xc2, // VPCMPEQQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm3_ymm4_ymm5_all_different() {
    let mut emu = emu64();
    // VPCMPEQQ YMM3, YMM4, YMM5 - all qwords different
    let code = [
        0xc4, 0xe2, 0x5d, 0x29, 0xdd, // VPCMPEQQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x29, 0xf0, // VPCMPEQQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm9_ymm10_ymm11_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x29, 0xcb, // VPCMPEQQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x29, 0xe6, // VPCMPEQQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x7d, 0x29, 0xf9, // VPCMPEQQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPEQQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x29, 0x00, // VPCMPEQQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..4).flat_map(|i| (i as u64 * 0x1111111111111111).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x29, 0x10, // VPCMPEQQ YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_ymm4_ymm5_mem_all_same() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x29, 0x20, // VPCMPEQQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x29, 0xc1, // VPCMPEQQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpcmpeqb_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x74, 0xc2, // VPCMPEQB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x74, 0xdb, // VPCMPEQB YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x75, 0xc2, // VPCMPEQW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x75, 0xdb, // VPCMPEQW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x76, 0xc2, // VPCMPEQD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x76, 0xdb, // VPCMPEQD YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x29, 0xc2, // VPCMPEQQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x29, 0xdb, // VPCMPEQQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeq_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x74, 0xc2, // VPCMPEQB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x75, 0xdb, // VPCMPEQW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x76, 0xe4, // VPCMPEQD YMM4, YMM3, YMM4
        0xc4, 0xe2, 0x5d, 0x29, 0xed, // VPCMPEQQ YMM5, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpeqb_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x74, 0x00, // VPCMPEQB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}
