use crate::*;

// VPSLLW/VPSLLD/VPSLLQ - Packed Shift Left Logical (AVX2)
//
// Performs SIMD logical left shift of packed integers.
// Shifts are performed independently on each element.
// Shift count can come from XMM register/memory or immediate.
//
// VPSLLW: Shift 16 packed word integers (16-bit each) left
// VPSLLD: Shift 8 packed doubleword integers (32-bit each) left
// VPSLLQ: Shift 4 packed quadword integers (64-bit each) left
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG F1 /r         VPSLLW ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 71 /6 ib      VPSLLW ymm1, ymm2, imm8
// VEX.256.66.0F.WIG F2 /r         VPSLLD ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 72 /6 ib      VPSLLD ymm1, ymm2, imm8
// VEX.256.66.0F.WIG F3 /r         VPSLLQ ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 73 /6 ib      VPSLLQ ymm1, ymm2, imm8

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPSLLW Tests - 16x Word Shift Left (256-bit)
// ============================================================================

#[test]
fn test_vpsllw_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSLLW YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x71, 0xf1, 0x00, // VPSLLW YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSLLW YMM3, YMM4, 1 - shift by 1 (multiply by 2)
    let code = [
        0xc5, 0xe5, 0x71, 0xf4, 0x01, // VPSLLW YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm6_ymm7_imm_4() {
    let mut emu = emu64();
    // VPSLLW YMM6, YMM7, 4 - shift by 4
    let code = [
        0xc5, 0xcd, 0x71, 0xf7, 0x04, // VPSLLW YMM6, YMM7, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm9_ymm10_imm_8() {
    let mut emu = emu64();
    // VPSLLW YMM9, YMM10, 8 - shift by one byte
    let code = [
        0xc4, 0x41, 0x35, 0x71, 0xf2, 0x08, // VPSLLW YMM9, YMM10, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm12_ymm13_imm_15() {
    let mut emu = emu64();
    // VPSLLW YMM12, YMM13, 15 - maximum useful shift for 16-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x71, 0xf5, 0x0f, // VPSLLW YMM12, YMM13, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm15_ymm0_imm_16() {
    let mut emu = emu64();
    // VPSLLW YMM15, YMM0, 16 - shift out all bits
    let code = [
        0xc4, 0xc1, 0x05, 0x71, 0xf0, 0x10, // VPSLLW YMM15, YMM0, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSLLW YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xf1, 0xc2, // VPSLLW YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xf1, 0xdd, // VPSLLW YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSLLW YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xf1, 0x30, // VPSLLW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 3u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xf1, 0x01, // VPSLLW YMM0, YMM1, 1
        0xc5, 0xfd, 0x71, 0xf0, 0x01, // VPSLLW YMM0, YMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSLLD Tests - 8x Doubleword Shift Left (256-bit)
// ============================================================================

#[test]
fn test_vpslld_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSLLD YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x72, 0xf1, 0x00, // VPSLLD YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSLLD YMM3, YMM4, 1 - shift by 1 (multiply by 2)
    let code = [
        0xc5, 0xe5, 0x72, 0xf4, 0x01, // VPSLLD YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm6_ymm7_imm_8() {
    let mut emu = emu64();
    // VPSLLD YMM6, YMM7, 8 - shift by one byte
    let code = [
        0xc5, 0xcd, 0x72, 0xf7, 0x08, // VPSLLD YMM6, YMM7, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm9_ymm10_imm_16() {
    let mut emu = emu64();
    // VPSLLD YMM9, YMM10, 16 - shift by two bytes
    let code = [
        0xc4, 0x41, 0x35, 0x72, 0xf2, 0x10, // VPSLLD YMM9, YMM10, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm12_ymm13_imm_31() {
    let mut emu = emu64();
    // VPSLLD YMM12, YMM13, 31 - maximum useful shift for 32-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x72, 0xf5, 0x1f, // VPSLLD YMM12, YMM13, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm15_ymm0_imm_32() {
    let mut emu = emu64();
    // VPSLLD YMM15, YMM0, 32 - shift out all bits
    let code = [
        0xc4, 0xc1, 0x05, 0x72, 0xf0, 0x20, // VPSLLD YMM15, YMM0, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSLLD YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xf2, 0xc2, // VPSLLD YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xf2, 0xdd, // VPSLLD YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSLLD YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xf2, 0x30, // VPSLLD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 5u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xf1, 0x02, // VPSLLD YMM0, YMM1, 2
        0xc5, 0xfd, 0x72, 0xf0, 0x02, // VPSLLD YMM0, YMM0, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSLLQ Tests - 4x Quadword Shift Left (256-bit)
// ============================================================================

#[test]
fn test_vpsllq_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSLLQ YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x73, 0xf1, 0x00, // VPSLLQ YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSLLQ YMM3, YMM4, 1 - shift by 1 (multiply by 2)
    let code = [
        0xc5, 0xe5, 0x73, 0xf4, 0x01, // VPSLLQ YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm6_ymm7_imm_8() {
    let mut emu = emu64();
    // VPSLLQ YMM6, YMM7, 8 - shift by one byte
    let code = [
        0xc5, 0xcd, 0x73, 0xf7, 0x08, // VPSLLQ YMM6, YMM7, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm9_ymm10_imm_32() {
    let mut emu = emu64();
    // VPSLLQ YMM9, YMM10, 32 - shift by four bytes
    let code = [
        0xc4, 0x41, 0x35, 0x73, 0xf2, 0x20, // VPSLLQ YMM9, YMM10, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm12_ymm13_imm_63() {
    let mut emu = emu64();
    // VPSLLQ YMM12, YMM13, 63 - maximum useful shift for 64-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x73, 0xf5, 0x3f, // VPSLLQ YMM12, YMM13, 63
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm15_ymm0_imm_64() {
    let mut emu = emu64();
    // VPSLLQ YMM15, YMM0, 64 - shift out all bits
    let code = [
        0xc4, 0xc1, 0x05, 0x73, 0xf0, 0x40, // VPSLLQ YMM15, YMM0, 64
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSLLQ YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xf3, 0xc2, // VPSLLQ YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xf3, 0xdd, // VPSLLQ YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSLLQ YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xf3, 0x30, // VPSLLQ YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 7u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xf1, 0x04, // VPSLLQ YMM0, YMM1, 4
        0xc5, 0xfd, 0x73, 0xf0, 0x04, // VPSLLQ YMM0, YMM0, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpsllw_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xf1, 0x00, // VPSLLW YMM0, YMM1, 0
        0xc5, 0xed, 0x71, 0xf2, 0x01, // VPSLLW YMM2, YMM2, 1
        0xc5, 0xe5, 0x71, 0xf3, 0x07, // VPSLLW YMM3, YMM3, 7
        0xc5, 0xdd, 0x71, 0xf4, 0x0f, // VPSLLW YMM4, YMM4, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xf1, 0x00, // VPSLLD YMM0, YMM1, 0
        0xc5, 0xed, 0x72, 0xf2, 0x01, // VPSLLD YMM2, YMM2, 1
        0xc5, 0xe5, 0x72, 0xf3, 0x10, // VPSLLD YMM3, YMM3, 16
        0xc5, 0xdd, 0x72, 0xf4, 0x1f, // VPSLLD YMM4, YMM4, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xf1, 0x00, // VPSLLQ YMM0, YMM1, 0
        0xc5, 0xed, 0x73, 0xf2, 0x01, // VPSLLQ YMM2, YMM2, 1
        0xc5, 0xe5, 0x73, 0xf3, 0x20, // VPSLLQ YMM3, YMM3, 32
        0xc5, 0xdd, 0x73, 0xf4, 0x3f, // VPSLLQ YMM4, YMM4, 63
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsll_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xf1, 0x01, // VPSLLW YMM0, YMM1, 1
        0xc5, 0xed, 0x72, 0xf2, 0x02, // VPSLLD YMM2, YMM2, 2
        0xc5, 0xe5, 0x73, 0xf3, 0x03, // VPSLLQ YMM3, YMM3, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x71, 0xf0, 0x02, // VPSLLW YMM8, YMM8, 2
        0xc4, 0x41, 0x15, 0x71, 0xf5, 0x03, // VPSLLW YMM13, YMM13, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0x72, 0xf2, 0x04, // VPSLLD YMM9, YMM10, 4
        0xc4, 0x41, 0x0d, 0x72, 0xf6, 0x05, // VPSLLD YMM14, YMM14, 5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x73, 0xf3, 0x06, // VPSLLQ YMM11, YMM11, 6
        0xc4, 0x41, 0x05, 0x73, 0xf7, 0x07, // VPSLLQ YMM15, YMM15, 7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_power_of_two_multiplication() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xf1, 0x01, // VPSLLW YMM0, YMM1, 1  (x2)
        0xc5, 0xed, 0x71, 0xf2, 0x02, // VPSLLW YMM2, YMM2, 2  (x4)
        0xc5, 0xe5, 0x71, 0xf3, 0x03, // VPSLLW YMM3, YMM3, 3  (x8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_power_of_two_multiplication() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xf1, 0x01, // VPSLLD YMM0, YMM1, 1  (x2)
        0xc5, 0xed, 0x72, 0xf2, 0x02, // VPSLLD YMM2, YMM2, 2  (x4)
        0xc5, 0xe5, 0x72, 0xf3, 0x03, // VPSLLD YMM3, YMM3, 3  (x8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_power_of_two_multiplication() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xf1, 0x01, // VPSLLQ YMM0, YMM1, 1  (x2)
        0xc5, 0xed, 0x73, 0xf2, 0x02, // VPSLLQ YMM2, YMM2, 2  (x4)
        0xc5, 0xe5, 0x73, 0xf3, 0x03, // VPSLLQ YMM3, YMM3, 3  (x8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllw_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf1, 0x00, // VPSLLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpslld_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf2, 0x00, // VPSLLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllq_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf3, 0x00, // VPSLLQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}
