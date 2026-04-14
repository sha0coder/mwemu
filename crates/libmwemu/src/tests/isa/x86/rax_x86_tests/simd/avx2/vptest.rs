use crate::*;

// VPTEST - Logical Compare and Set Flags (AVX2)
//
// Performs a bitwise AND or AND-NOT operation and sets flags based on the result.
// Sets ZF=1 if the bitwise AND of the two operands is all zeros.
// Sets CF=1 if the bitwise AND-NOT (first AND NOT second) is all zeros.
//
// This is useful for testing if any bits are set, or if one operand is a subset of another.
//
// Opcodes:
// VEX.128.66.0F38.WIG 17 /r       VPTEST xmm1, xmm2/m128
// VEX.256.66.0F38.WIG 17 /r       VPTEST ymm1, ymm2/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPTEST Tests - 128-bit (XMM)
// ============================================================================

#[test]
fn test_vptest_xmm0_xmm1() {
    let mut emu = emu64();
    // VPTEST XMM0, XMM1 - test and set flags
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc1, // VPTEST XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xdc, // VPTEST XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xf7, // VPTEST XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x79, 0x17, 0xca, // VPTEST XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x79, 0x17, 0xe5, // VPTEST XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x79, 0x17, 0xf8, // VPTEST XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm0_mem() {
    let mut emu = emu64();
    // VPTEST XMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x79, 0x17, 0x00, // VPTEST XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x79, 0x17, 0x28, // VPTEST XMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x79, 0x17, 0x18, // VPTEST XMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPTEST Tests - 256-bit (YMM)
// ============================================================================

#[test]
fn test_vptest_ymm0_ymm1() {
    let mut emu = emu64();
    // VPTEST YMM0, YMM1 - test and set flags
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xc1, // VPTEST YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xdc, // VPTEST YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xf7, // VPTEST YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x17, 0xca, // VPTEST YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm12_ymm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x17, 0xe5, // VPTEST YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm15_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x17, 0xf8, // VPTEST YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm0_mem() {
    let mut emu = emu64();
    // VPTEST YMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x17, 0x00, // VPTEST YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x17, 0x28, // VPTEST YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x17, 0x18, // VPTEST YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0, 0xF0,
        0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vptest_xmm_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc1, // VPTEST XMM0, XMM1
        0xc4, 0xe2, 0x79, 0x17, 0xd2, // VPTEST XMM2, XMM2
        0xc4, 0xe2, 0x79, 0x17, 0xe3, // VPTEST XMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xc1, // VPTEST YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x17, 0xd2, // VPTEST YMM2, YMM2
        0xc4, 0xe2, 0x7d, 0x17, 0xe3, // VPTEST YMM4, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x79, 0x17, 0xc1, // VPTEST XMM8, XMM9
        0xc4, 0x42, 0x79, 0x17, 0xd5, // VPTEST XMM10, XMM13
        0xc4, 0x42, 0x79, 0x17, 0xff, // VPTEST XMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x17, 0xc1, // VPTEST YMM8, YMM9
        0xc4, 0x42, 0x7d, 0x17, 0xd5, // VPTEST YMM10, YMM13
        0xc4, 0x42, 0x7d, 0x17, 0xff, // VPTEST YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_self() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc0, // VPTEST XMM0, XMM0
        0xc4, 0xe2, 0x79, 0x17, 0xdb, // VPTEST XMM3, XMM3
        0xc4, 0xe2, 0x79, 0x17, 0xff, // VPTEST XMM7, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_self() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xc0, // VPTEST YMM0, YMM0
        0xc4, 0xe2, 0x7d, 0x17, 0xdb, // VPTEST YMM3, YMM3
        0xc4, 0xe2, 0x7d, 0x17, 0xff, // VPTEST YMM7, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc1, // VPTEST XMM0, XMM1
        0xc4, 0xe2, 0x79, 0x17, 0xd4, // VPTEST XMM2, XMM4
        0xc4, 0xe2, 0x79, 0x17, 0xee, // VPTEST XMM5, XMM6
        0xc4, 0xe2, 0x79, 0x17, 0xf8, // VPTEST XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xc1, // VPTEST YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x17, 0xd4, // VPTEST YMM2, YMM4
        0xc4, 0xe2, 0x7d, 0x17, 0xee, // VPTEST YMM5, YMM6
        0xc4, 0xe2, 0x7d, 0x17, 0xf8, // VPTEST YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_mem_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x79, 0x17, 0x00, // VPTEST XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_mem_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x17, 0x00, // VPTEST YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_mem_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x79, 0x17, 0x00, // VPTEST XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_mem_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x17, 0x00, // VPTEST YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x79, 0x17, 0x00, // VPTEST XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
                    0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x17, 0x00, // VPTEST YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
                    0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55,
                    0xF0, 0x0F, 0xF0, 0x0F, 0xF0, 0x0F, 0xF0, 0x0F,
                    0xF0, 0x0F, 0xF0, 0x0F, 0xF0, 0x0F, 0xF0, 0x0F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc1, // VPTEST XMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x17, 0xd2, // VPTEST YMM2, YMM2
        0xc4, 0xe2, 0x79, 0x17, 0xe3, // VPTEST XMM4, XMM3
        0xc4, 0xe2, 0x7d, 0x17, 0xee, // VPTEST YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_consecutive_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc1, // VPTEST XMM0, XMM1
        0xc4, 0xe2, 0x79, 0x17, 0xca, // VPTEST XMM1, XMM2
        0xc4, 0xe2, 0x79, 0x17, 0xd3, // VPTEST XMM2, XMM3
        0xc4, 0xe2, 0x79, 0x17, 0xdc, // VPTEST XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_consecutive_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xc1, // VPTEST YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x17, 0xca, // VPTEST YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x17, 0xd3, // VPTEST YMM2, YMM3
        0xc4, 0xe2, 0x7d, 0x17, 0xdc, // VPTEST YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_xmm_reverse_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x79, 0x17, 0xc1, // VPTEST XMM0, XMM1
        0xc4, 0xe2, 0x79, 0x17, 0xc8, // VPTEST XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vptest_ymm_reverse_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x17, 0xc1, // VPTEST YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x17, 0xc8, // VPTEST YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
