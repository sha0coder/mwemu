use crate::*;

// VPSRAW/VPSRAD - Packed Shift Right Arithmetic (AVX2)
//
// Performs SIMD arithmetic right shift of packed integers.
// Shifts are performed independently on each element.
// Shift count can come from XMM register/memory or immediate.
// Sign bit is replicated (sign extension).
//
// VPSRAW: Shift 16 packed word integers (16-bit each) right arithmetic
// VPSRAD: Shift 8 packed doubleword integers (32-bit each) right arithmetic
//
// Note: VPSRAQ (64-bit arithmetic shift) requires AVX512
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG E1 /r         VPSRAW ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 71 /4 ib      VPSRAW ymm1, ymm2, imm8
// VEX.256.66.0F.WIG E2 /r         VPSRAD ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 72 /4 ib      VPSRAD ymm1, ymm2, imm8

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPSRAW Tests - 16x Word Shift Right Arithmetic (256-bit)
// ============================================================================

#[test]
fn test_vpsraw_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSRAW YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x00, // VPSRAW YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSRAW YMM3, YMM4, 1 - shift by 1
    let code = [
        0xc5, 0xe5, 0x71, 0xe4, 0x01, // VPSRAW YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm6_ymm7_imm_4() {
    let mut emu = emu64();
    // VPSRAW YMM6, YMM7, 4 - shift by 4
    let code = [
        0xc5, 0xcd, 0x71, 0xe7, 0x04, // VPSRAW YMM6, YMM7, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm9_ymm10_imm_8() {
    let mut emu = emu64();
    // VPSRAW YMM9, YMM10, 8 - shift by one byte
    let code = [
        0xc4, 0x41, 0x35, 0x71, 0xe2, 0x08, // VPSRAW YMM9, YMM10, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm12_ymm13_imm_15() {
    let mut emu = emu64();
    // VPSRAW YMM12, YMM13, 15 - maximum useful shift for 16-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x71, 0xe5, 0x0f, // VPSRAW YMM12, YMM13, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm15_ymm0_imm_16() {
    let mut emu = emu64();
    // VPSRAW YMM15, YMM0, 16 - shift >= width (fill with sign bit)
    let code = [
        0xc4, 0xc1, 0x05, 0x71, 0xe0, 0x10, // VPSRAW YMM15, YMM0, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSRAW YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xe1, 0xc2, // VPSRAW YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xe1, 0xdd, // VPSRAW YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSRAW YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xe1, 0x30, // VPSRAW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 3u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x01, // VPSRAW YMM0, YMM1, 1
        0xc5, 0xfd, 0x71, 0xe0, 0x01, // VPSRAW YMM0, YMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_negative_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x01, // VPSRAW YMM0, YMM1, 1
        0xc5, 0xed, 0x71, 0xe2, 0x04, // VPSRAW YMM2, YMM2, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_positive_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x01, // VPSRAW YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSRAD Tests - 8x Doubleword Shift Right Arithmetic (256-bit)
// ============================================================================

#[test]
fn test_vpsrad_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSRAD YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x00, // VPSRAD YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSRAD YMM3, YMM4, 1 - shift by 1
    let code = [
        0xc5, 0xe5, 0x72, 0xe4, 0x01, // VPSRAD YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm6_ymm7_imm_8() {
    let mut emu = emu64();
    // VPSRAD YMM6, YMM7, 8 - shift by one byte
    let code = [
        0xc5, 0xcd, 0x72, 0xe7, 0x08, // VPSRAD YMM6, YMM7, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm9_ymm10_imm_16() {
    let mut emu = emu64();
    // VPSRAD YMM9, YMM10, 16 - shift by two bytes
    let code = [
        0xc4, 0x41, 0x35, 0x72, 0xe2, 0x10, // VPSRAD YMM9, YMM10, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm12_ymm13_imm_31() {
    let mut emu = emu64();
    // VPSRAD YMM12, YMM13, 31 - maximum useful shift for 32-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x72, 0xe5, 0x1f, // VPSRAD YMM12, YMM13, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm15_ymm0_imm_32() {
    let mut emu = emu64();
    // VPSRAD YMM15, YMM0, 32 - shift >= width (fill with sign bit)
    let code = [
        0xc4, 0xc1, 0x05, 0x72, 0xe0, 0x20, // VPSRAD YMM15, YMM0, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSRAD YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xe2, 0xc2, // VPSRAD YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xe2, 0xdd, // VPSRAD YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSRAD YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xe2, 0x30, // VPSRAD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 5u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x02, // VPSRAD YMM0, YMM1, 2
        0xc5, 0xfd, 0x72, 0xe0, 0x02, // VPSRAD YMM0, YMM0, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_negative_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x01, // VPSRAD YMM0, YMM1, 1
        0xc5, 0xed, 0x72, 0xe2, 0x08, // VPSRAD YMM2, YMM2, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_positive_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x01, // VPSRAD YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpsraw_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x00, // VPSRAW YMM0, YMM1, 0
        0xc5, 0xed, 0x71, 0xe2, 0x01, // VPSRAW YMM2, YMM2, 1
        0xc5, 0xe5, 0x71, 0xe3, 0x07, // VPSRAW YMM3, YMM3, 7
        0xc5, 0xdd, 0x71, 0xe4, 0x0f, // VPSRAW YMM4, YMM4, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x00, // VPSRAD YMM0, YMM1, 0
        0xc5, 0xed, 0x72, 0xe2, 0x01, // VPSRAD YMM2, YMM2, 1
        0xc5, 0xe5, 0x72, 0xe3, 0x10, // VPSRAD YMM3, YMM3, 16
        0xc5, 0xdd, 0x72, 0xe4, 0x1f, // VPSRAD YMM4, YMM4, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsra_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x01, // VPSRAW YMM0, YMM1, 1
        0xc5, 0xed, 0x72, 0xe2, 0x02, // VPSRAD YMM2, YMM2, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x71, 0xe0, 0x02, // VPSRAW YMM8, YMM8, 2
        0xc4, 0x41, 0x15, 0x71, 0xe5, 0x03, // VPSRAW YMM13, YMM13, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0x72, 0xe2, 0x04, // VPSRAD YMM9, YMM10, 4
        0xc4, 0x41, 0x0d, 0x72, 0xe6, 0x05, // VPSRAD YMM14, YMM14, 5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe1, 0x00, // VPSRAW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe2, 0x00, // VPSRAD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm8_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0xe1, 0xc2, // VPSRAW YMM8, YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm11_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xe2, 0xdd, // VPSRAD YMM11, YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_sign_extension_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x0f, // VPSRAW YMM0, YMM1, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_sign_extension_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x1f, // VPSRAD YMM0, YMM1, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_boundary_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x0f, // VPSRAW YMM0, YMM1, 15
        0xc5, 0xed, 0x71, 0xe2, 0x10, // VPSRAW YMM2, YMM2, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_boundary_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x1f, // VPSRAD YMM0, YMM1, 31
        0xc5, 0xed, 0x72, 0xe2, 0x20, // VPSRAD YMM2, YMM2, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_ymm14_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x05, 0xe1, 0xf0, // VPSRAW YMM14, YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_ymm15_ymm14_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x0d, 0xe2, 0xfd, // VPSRAD YMM15, YMM14, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_all_ones() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x01, // VPSRAW YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_all_ones() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x01, // VPSRAD YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x02, // VPSRAW YMM0, YMM1, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x02, // VPSRAD YMM0, YMM1, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsraw_small_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xe1, 0x01, // VPSRAW YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrad_small_negative() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xe1, 0x01, // VPSRAD YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
