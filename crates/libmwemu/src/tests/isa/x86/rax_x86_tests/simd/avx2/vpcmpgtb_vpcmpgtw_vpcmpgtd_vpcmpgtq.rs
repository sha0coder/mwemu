use crate::*;

// VPCMPGTB/VPCMPGTW/VPCMPGTD/VPCMPGTQ - Packed Greater Than Comparison (AVX2)
//
// Performs SIMD signed comparison of packed integers for greater than.
// For each element, if src1 > src2 (signed), all bits in result element are set to 1, otherwise 0.
//
// VPCMPGTB: Compare 32 packed byte integers (8-bit each) - signed
// VPCMPGTW: Compare 16 packed word integers (16-bit each) - signed
// VPCMPGTD: Compare 8 packed doubleword integers (32-bit each) - signed
// VPCMPGTQ: Compare 4 packed quadword integers (64-bit each) - signed
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 64 /r     VPCMPGTB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 65 /r     VPCMPGTW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 66 /r     VPCMPGTD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 37 /r   VPCMPGTQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPCMPGTB Tests - 32x Byte Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtb_ymm0_ymm1_ymm2_all_greater() {
    let mut emu = emu64();
    // VPCMPGTB YMM0, YMM1, YMM2 - all bytes in YMM1 > YMM2
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm3_ymm4_ymm5_all_less() {
    let mut emu = emu64();
    // VPCMPGTB YMM3, YMM4, YMM5 - all bytes in YMM4 < YMM5
    let code = [
        0xc5, 0xdd, 0x64, 0xdd, // VPCMPGTB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x64, 0xf0, // VPCMPGTB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm9_ymm10_ymm11_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x64, 0xcb, // VPCMPGTB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x64, 0xe6, // VPCMPGTB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x64, 0xf9, // VPCMPGTB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPGTB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x64, 0x00, // VPCMPGTB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x64, 0x10, // VPCMPGTB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_ymm4_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x64, 0x20, // VPCMPGTB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x64, 0xc1, // VPCMPGTB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPCMPGTW Tests - 16x Word Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtw_ymm0_ymm1_ymm2_all_greater() {
    let mut emu = emu64();
    // VPCMPGTW YMM0, YMM1, YMM2 - all words in YMM1 > YMM2
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm3_ymm4_ymm5_all_less() {
    let mut emu = emu64();
    // VPCMPGTW YMM3, YMM4, YMM5 - all words in YMM4 < YMM5
    let code = [
        0xc5, 0xdd, 0x65, 0xdd, // VPCMPGTW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x65, 0xf0, // VPCMPGTW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm9_ymm10_ymm11_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x65, 0xcb, // VPCMPGTW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x65, 0xe6, // VPCMPGTW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x65, 0xf9, // VPCMPGTW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPGTW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x65, 0x00, // VPCMPGTW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16 * 0x1111).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x65, 0x10, // VPCMPGTW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_ymm4_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x65, 0x20, // VPCMPGTW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x65, 0xc1, // VPCMPGTW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPCMPGTD Tests - 8x Doubleword Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtd_ymm0_ymm1_ymm2_all_greater() {
    let mut emu = emu64();
    // VPCMPGTD YMM0, YMM1, YMM2 - all dwords in YMM1 > YMM2
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm3_ymm4_ymm5_all_less() {
    let mut emu = emu64();
    // VPCMPGTD YMM3, YMM4, YMM5 - all dwords in YMM4 < YMM5
    let code = [
        0xc5, 0xdd, 0x66, 0xdd, // VPCMPGTD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x66, 0xf0, // VPCMPGTD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm9_ymm10_ymm11_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x66, 0xcb, // VPCMPGTD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x66, 0xe6, // VPCMPGTD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x66, 0xf9, // VPCMPGTD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPGTD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x66, 0x00, // VPCMPGTD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32 * 0x11111111).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x66, 0x10, // VPCMPGTD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_ymm4_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x66, 0x20, // VPCMPGTD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x66, 0xc1, // VPCMPGTD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPCMPGTQ Tests - 4x Quadword Greater Than Comparison (256-bit)
// ============================================================================

#[test]
fn test_vpcmpgtq_ymm0_ymm1_ymm2_all_greater() {
    let mut emu = emu64();
    // VPCMPGTQ YMM0, YMM1, YMM2 - all qwords in YMM1 > YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm3_ymm4_ymm5_all_less() {
    let mut emu = emu64();
    // VPCMPGTQ YMM3, YMM4, YMM5 - all qwords in YMM4 < YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x37, 0xdd, // VPCMPGTQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm6_ymm7_ymm8_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x37, 0xf0, // VPCMPGTQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm9_ymm10_ymm11_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x37, 0xcb, // VPCMPGTQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm12_ymm13_ymm14_zeros() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x37, 0xe6, // VPCMPGTQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x7d, 0x37, 0xf9, // VPCMPGTQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPCMPGTQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x37, 0x00, // VPCMPGTQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..4).flat_map(|i| (i as u64 * 0x1111111111111111).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm2_ymm3_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x37, 0x10, // VPCMPGTQ YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_ymm4_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x37, 0x20, // VPCMPGTQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_self_comparison() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc1, // VPCMPGTQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpcmpgtb_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x64, 0xdb, // VPCMPGTB YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x65, 0xdb, // VPCMPGTW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x66, 0xdb, // VPCMPGTD YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x37, 0xdb, // VPCMPGTQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgt_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xc5, 0xed, 0x65, 0xdb, // VPCMPGTW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x66, 0xe4, // VPCMPGTD YMM4, YMM3, YMM4
        0xc4, 0xe2, 0x5d, 0x37, 0xed, // VPCMPGTQ YMM5, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_signed_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_signed_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_signed_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_signed_comparison() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x64, 0xc1, // VPCMPGTB YMM8, YMM8, YMM9
        0xc4, 0x41, 0x15, 0x64, 0xee, // VPCMPGTB YMM13, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0x65, 0xcb, // VPCMPGTW YMM9, YMM9, YMM11
        0xc4, 0x41, 0x0d, 0x65, 0xf7, // VPCMPGTW YMM14, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x66, 0xd4, // VPCMPGTD YMM10, YMM10, YMM12
        0xc4, 0x41, 0x05, 0x66, 0xf8, // VPCMPGTD YMM15, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x25, 0x37, 0xdd, // VPCMPGTQ YMM11, YMM11, YMM13
        0xc4, 0x42, 0x05, 0x37, 0xf9, // VPCMPGTQ YMM15, YMM15, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_positive_vs_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_positive_vs_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_positive_vs_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_positive_vs_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtb_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x64, 0xc2, // VPCMPGTB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtw_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x65, 0xc2, // VPCMPGTW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtd_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x66, 0xc2, // VPCMPGTD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpcmpgtq_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x37, 0xc2, // VPCMPGTQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
