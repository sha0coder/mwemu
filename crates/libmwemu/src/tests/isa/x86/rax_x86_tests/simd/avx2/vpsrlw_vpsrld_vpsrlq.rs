use crate::*;

// VPSRLW/VPSRLD/VPSRLQ - Packed Shift Right Logical (AVX2)
//
// Performs SIMD logical right shift of packed integers.
// Shifts are performed independently on each element.
// Shift count can come from XMM register/memory or immediate.
// Zeros are shifted in from the left.
//
// VPSRLW: Shift 16 packed word integers (16-bit each) right
// VPSRLD: Shift 8 packed doubleword integers (32-bit each) right
// VPSRLQ: Shift 4 packed quadword integers (64-bit each) right
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG D1 /r         VPSRLW ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 71 /2 ib      VPSRLW ymm1, ymm2, imm8
// VEX.256.66.0F.WIG D2 /r         VPSRLD ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 72 /2 ib      VPSRLD ymm1, ymm2, imm8
// VEX.256.66.0F.WIG D3 /r         VPSRLQ ymm1, ymm2, xmm3/m128
// VEX.256.66.0F.WIG 73 /2 ib      VPSRLQ ymm1, ymm2, imm8

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPSRLW Tests - 16x Word Shift Right Logical (256-bit)
// ============================================================================

#[test]
fn test_vpsrlw_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSRLW YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x00, // VPSRLW YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSRLW YMM3, YMM4, 1 - shift by 1 (divide by 2)
    let code = [
        0xc5, 0xe5, 0x71, 0xd4, 0x01, // VPSRLW YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm6_ymm7_imm_4() {
    let mut emu = emu64();
    // VPSRLW YMM6, YMM7, 4 - shift by 4
    let code = [
        0xc5, 0xcd, 0x71, 0xd7, 0x04, // VPSRLW YMM6, YMM7, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm9_ymm10_imm_8() {
    let mut emu = emu64();
    // VPSRLW YMM9, YMM10, 8 - shift by one byte
    let code = [
        0xc4, 0x41, 0x35, 0x71, 0xd2, 0x08, // VPSRLW YMM9, YMM10, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm12_ymm13_imm_15() {
    let mut emu = emu64();
    // VPSRLW YMM12, YMM13, 15 - maximum useful shift for 16-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x71, 0xd5, 0x0f, // VPSRLW YMM12, YMM13, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm15_ymm0_imm_16() {
    let mut emu = emu64();
    // VPSRLW YMM15, YMM0, 16 - shift out all bits
    let code = [
        0xc4, 0xc1, 0x05, 0x71, 0xd0, 0x10, // VPSRLW YMM15, YMM0, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSRLW YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xd1, 0xc2, // VPSRLW YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xd1, 0xdd, // VPSRLW YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSRLW YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xd1, 0x30, // VPSRLW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 3u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x01, // VPSRLW YMM0, YMM1, 1
        0xc5, 0xfd, 0x71, 0xd0, 0x01, // VPSRLW YMM0, YMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSRLD Tests - 8x Doubleword Shift Right Logical (256-bit)
// ============================================================================

#[test]
fn test_vpsrld_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSRLD YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x72, 0xd1, 0x00, // VPSRLD YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSRLD YMM3, YMM4, 1 - shift by 1 (divide by 2)
    let code = [
        0xc5, 0xe5, 0x72, 0xd4, 0x01, // VPSRLD YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm6_ymm7_imm_8() {
    let mut emu = emu64();
    // VPSRLD YMM6, YMM7, 8 - shift by one byte
    let code = [
        0xc5, 0xcd, 0x72, 0xd7, 0x08, // VPSRLD YMM6, YMM7, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm9_ymm10_imm_16() {
    let mut emu = emu64();
    // VPSRLD YMM9, YMM10, 16 - shift by two bytes
    let code = [
        0xc4, 0x41, 0x35, 0x72, 0xd2, 0x10, // VPSRLD YMM9, YMM10, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm12_ymm13_imm_31() {
    let mut emu = emu64();
    // VPSRLD YMM12, YMM13, 31 - maximum useful shift for 32-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x72, 0xd5, 0x1f, // VPSRLD YMM12, YMM13, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm15_ymm0_imm_32() {
    let mut emu = emu64();
    // VPSRLD YMM15, YMM0, 32 - shift out all bits
    let code = [
        0xc4, 0xc1, 0x05, 0x72, 0xd0, 0x20, // VPSRLD YMM15, YMM0, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSRLD YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xd2, 0xc2, // VPSRLD YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xd2, 0xdd, // VPSRLD YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSRLD YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xd2, 0x30, // VPSRLD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 5u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xd1, 0x02, // VPSRLD YMM0, YMM1, 2
        0xc5, 0xfd, 0x72, 0xd0, 0x02, // VPSRLD YMM0, YMM0, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPSRLQ Tests - 4x Quadword Shift Right Logical (256-bit)
// ============================================================================

#[test]
fn test_vpsrlq_ymm0_ymm1_imm_0() {
    let mut emu = emu64();
    // VPSRLQ YMM0, YMM1, 0 - shift by 0 (no change)
    let code = [
        0xc5, 0xf5, 0x73, 0xd1, 0x00, // VPSRLQ YMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm3_ymm4_imm_1() {
    let mut emu = emu64();
    // VPSRLQ YMM3, YMM4, 1 - shift by 1 (divide by 2)
    let code = [
        0xc5, 0xe5, 0x73, 0xd4, 0x01, // VPSRLQ YMM3, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm6_ymm7_imm_8() {
    let mut emu = emu64();
    // VPSRLQ YMM6, YMM7, 8 - shift by one byte
    let code = [
        0xc5, 0xcd, 0x73, 0xd7, 0x08, // VPSRLQ YMM6, YMM7, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm9_ymm10_imm_32() {
    let mut emu = emu64();
    // VPSRLQ YMM9, YMM10, 32 - shift by four bytes
    let code = [
        0xc4, 0x41, 0x35, 0x73, 0xd2, 0x20, // VPSRLQ YMM9, YMM10, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm12_ymm13_imm_63() {
    let mut emu = emu64();
    // VPSRLQ YMM12, YMM13, 63 - maximum useful shift for 64-bit
    let code = [
        0xc4, 0x41, 0x1d, 0x73, 0xd5, 0x3f, // VPSRLQ YMM12, YMM13, 63
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm15_ymm0_imm_64() {
    let mut emu = emu64();
    // VPSRLQ YMM15, YMM0, 64 - shift out all bits
    let code = [
        0xc4, 0xc1, 0x05, 0x73, 0xd0, 0x40, // VPSRLQ YMM15, YMM0, 64
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm0_ymm1_xmm2() {
    let mut emu = emu64();
    // VPSRLQ YMM0, YMM1, XMM2 - variable shift count from register
    let code = [
        0xc5, 0xf5, 0xd3, 0xc2, // VPSRLQ YMM0, YMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm3_ymm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0xd3, 0xdd, // VPSRLQ YMM3, YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm6_ymm7_mem() {
    let mut emu = emu64();
    // VPSRLQ YMM6, YMM7, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xd3, 0x30, // VPSRLQ YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 7u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xd1, 0x04, // VPSRLQ YMM0, YMM1, 4
        0xc5, 0xfd, 0x73, 0xd0, 0x04, // VPSRLQ YMM0, YMM0, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpsrlw_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x00, // VPSRLW YMM0, YMM1, 0
        0xc5, 0xed, 0x71, 0xd2, 0x01, // VPSRLW YMM2, YMM2, 1
        0xc5, 0xe5, 0x71, 0xd3, 0x07, // VPSRLW YMM3, YMM3, 7
        0xc5, 0xdd, 0x71, 0xd4, 0x0f, // VPSRLW YMM4, YMM4, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xd1, 0x00, // VPSRLD YMM0, YMM1, 0
        0xc5, 0xed, 0x72, 0xd2, 0x01, // VPSRLD YMM2, YMM2, 1
        0xc5, 0xe5, 0x72, 0xd3, 0x10, // VPSRLD YMM3, YMM3, 16
        0xc5, 0xdd, 0x72, 0xd4, 0x1f, // VPSRLD YMM4, YMM4, 31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_all_shift_amounts() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xd1, 0x00, // VPSRLQ YMM0, YMM1, 0
        0xc5, 0xed, 0x73, 0xd2, 0x01, // VPSRLQ YMM2, YMM2, 1
        0xc5, 0xe5, 0x73, 0xd3, 0x20, // VPSRLQ YMM3, YMM3, 32
        0xc5, 0xdd, 0x73, 0xd4, 0x3f, // VPSRLQ YMM4, YMM4, 63
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrl_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x01, // VPSRLW YMM0, YMM1, 1
        0xc5, 0xed, 0x72, 0xd2, 0x02, // VPSRLD YMM2, YMM2, 2
        0xc5, 0xe5, 0x73, 0xd3, 0x03, // VPSRLQ YMM3, YMM3, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x71, 0xd0, 0x02, // VPSRLW YMM8, YMM8, 2
        0xc4, 0x41, 0x15, 0x71, 0xd5, 0x03, // VPSRLW YMM13, YMM13, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0x72, 0xd2, 0x04, // VPSRLD YMM9, YMM10, 4
        0xc4, 0x41, 0x0d, 0x72, 0xd6, 0x05, // VPSRLD YMM14, YMM14, 5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x73, 0xd3, 0x06, // VPSRLQ YMM11, YMM11, 6
        0xc4, 0x41, 0x05, 0x73, 0xd7, 0x07, // VPSRLQ YMM15, YMM15, 7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_power_of_two_division() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x01, // VPSRLW YMM0, YMM1, 1  (/2)
        0xc5, 0xed, 0x71, 0xd2, 0x02, // VPSRLW YMM2, YMM2, 2  (/4)
        0xc5, 0xe5, 0x71, 0xd3, 0x03, // VPSRLW YMM3, YMM3, 3  (/8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_power_of_two_division() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xd1, 0x01, // VPSRLD YMM0, YMM1, 1  (/2)
        0xc5, 0xed, 0x72, 0xd2, 0x02, // VPSRLD YMM2, YMM2, 2  (/4)
        0xc5, 0xe5, 0x72, 0xd3, 0x03, // VPSRLD YMM3, YMM3, 3  (/8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_power_of_two_division() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xd1, 0x01, // VPSRLQ YMM0, YMM1, 1  (/2)
        0xc5, 0xed, 0x73, 0xd2, 0x02, // VPSRLQ YMM2, YMM2, 2  (/4)
        0xc5, 0xe5, 0x73, 0xd3, 0x03, // VPSRLQ YMM3, YMM3, 3  (/8)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd1, 0x00, // VPSRLW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd2, 0x00, // VPSRLD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_mem_large_shift_count() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xd3, 0x00, // VPSRLQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let shift_count: Vec<u8> = 100u64.to_le_bytes().to_vec();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &shift_count);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_ymm8_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0xd1, 0xc2, // VPSRLW YMM8, YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_ymm11_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xd2, 0xdd, // VPSRLD YMM11, YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_ymm14_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x05, 0xd3, 0xf0, // VPSRLQ YMM14, YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_max_value_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x01, // VPSRLW YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_max_value_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xd1, 0x01, // VPSRLD YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_max_value_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xd1, 0x01, // VPSRLQ YMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlw_boundary_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x71, 0xd1, 0x0f, // VPSRLW YMM0, YMM1, 15
        0xc5, 0xed, 0x71, 0xd2, 0x10, // VPSRLW YMM2, YMM2, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrld_boundary_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x72, 0xd1, 0x1f, // VPSRLD YMM0, YMM1, 31
        0xc5, 0xed, 0x72, 0xd2, 0x20, // VPSRLD YMM2, YMM2, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlq_boundary_shift() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x73, 0xd1, 0x3f, // VPSRLQ YMM0, YMM1, 63
        0xc5, 0xed, 0x73, 0xd2, 0x40, // VPSRLQ YMM2, YMM2, 64
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
