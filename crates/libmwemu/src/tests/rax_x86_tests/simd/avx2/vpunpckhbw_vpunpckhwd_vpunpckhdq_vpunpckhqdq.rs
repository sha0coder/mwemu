use crate::*;

// VPUNPCKHBW/VPUNPCKHWD/VPUNPCKHDQ/VPUNPCKHQDQ - Unpack High Data (AVX2)
//
// Interleaves high-order elements from two source operands.
// Takes upper half of each source and interleaves them.
//
// VPUNPCKHBW: Unpack and interleave high-order bytes
//             Takes high 16 bytes from each 128-bit lane
//
// VPUNPCKHWD: Unpack and interleave high-order words
//             Takes high 8 words from each 128-bit lane
//
// VPUNPCKHDQ: Unpack and interleave high-order doublewords
//             Takes high 4 dwords from each 128-bit lane
//
// VPUNPCKHQDQ: Unpack and interleave high-order quadwords
//              Takes high 2 qwords from each 128-bit lane
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 68 /r     VPUNPCKHBW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 69 /r     VPUNPCKHWD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 6A /r     VPUNPCKHDQ ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 6D /r     VPUNPCKHQDQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPUNPCKHBW Tests - Unpack High Bytes (256-bit)
// ============================================================================

#[test]
fn test_vpunpckhbw_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKHBW YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x68, 0xc2, // VPUNPCKHBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x68, 0xdd, // VPUNPCKHBW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x68, 0xf0, // VPUNPCKHBW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x68, 0xcb, // VPUNPCKHBW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x68, 0xe6, // VPUNPCKHBW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x68, 0xf9, // VPUNPCKHBW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKHBW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x68, 0x00, // VPUNPCKHBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x68, 0xc2, // VPUNPCKHBW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x68, 0xdb, // VPUNPCKHBW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x68, 0xc1, // VPUNPCKHBW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm2_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x68, 0x10, // VPUNPCKHBW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

// ============================================================================
// VPUNPCKHWD Tests - Unpack High Words (256-bit)
// ============================================================================

#[test]
fn test_vpunpckhwd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKHWD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x69, 0xc2, // VPUNPCKHWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x69, 0xdd, // VPUNPCKHWD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x69, 0xf0, // VPUNPCKHWD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x69, 0xcb, // VPUNPCKHWD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x69, 0xe6, // VPUNPCKHWD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x69, 0xf9, // VPUNPCKHWD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKHWD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x69, 0x00, // VPUNPCKHWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x69, 0xc2, // VPUNPCKHWD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x69, 0xdb, // VPUNPCKHWD YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x69, 0xc1, // VPUNPCKHWD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPUNPCKHDQ Tests - Unpack High Doublewords (256-bit)
// ============================================================================

#[test]
fn test_vpunpckhdq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKHDQ YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x6a, 0xc2, // VPUNPCKHDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x6a, 0xdd, // VPUNPCKHDQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x6a, 0xf0, // VPUNPCKHDQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x6a, 0xcb, // VPUNPCKHDQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x6a, 0xe6, // VPUNPCKHDQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x6a, 0xf9, // VPUNPCKHDQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKHDQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x6a, 0x00, // VPUNPCKHDQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6a, 0xc2, // VPUNPCKHDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6a, 0xdb, // VPUNPCKHDQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6a, 0xc1, // VPUNPCKHDQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPUNPCKHQDQ Tests - Unpack High Quadwords (256-bit)
// ============================================================================

#[test]
fn test_vpunpckhqdq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKHQDQ YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x6d, 0xc2, // VPUNPCKHQDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x6d, 0xdd, // VPUNPCKHQDQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x6d, 0xf0, // VPUNPCKHQDQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x6d, 0xcb, // VPUNPCKHQDQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x6d, 0xe6, // VPUNPCKHQDQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x6d, 0xf9, // VPUNPCKHQDQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKHQDQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x6d, 0x00, // VPUNPCKHQDQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6d, 0xc2, // VPUNPCKHQDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6d, 0xdb, // VPUNPCKHQDQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6d, 0xc1, // VPUNPCKHQDQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpunpckh_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x68, 0xc2, // VPUNPCKHBW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x69, 0xdb, // VPUNPCKHWD YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x6a, 0xe4, // VPUNPCKHDQ YMM4, YMM3, YMM4
        0xc5, 0xdd, 0x6d, 0xed, // VPUNPCKHQDQ YMM5, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x68, 0xc1, // VPUNPCKHBW YMM8, YMM8, YMM9
        0xc4, 0x41, 0x15, 0x68, 0xee, // VPUNPCKHBW YMM13, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0x69, 0xcb, // VPUNPCKHWD YMM9, YMM9, YMM11
        0xc4, 0x41, 0x0d, 0x69, 0xf7, // VPUNPCKHWD YMM14, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x6a, 0xd4, // VPUNPCKHDQ YMM10, YMM10, YMM12
        0xc4, 0x41, 0x05, 0x6a, 0xf8, // VPUNPCKHDQ YMM15, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x25, 0x6d, 0xdd, // VPUNPCKHQDQ YMM11, YMM11, YMM13
        0xc4, 0x41, 0x05, 0x6d, 0xf9, // VPUNPCKHQDQ YMM15, YMM15, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x68, 0xc2, // VPUNPCKHBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x69, 0xc2, // VPUNPCKHWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6a, 0xc2, // VPUNPCKHDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6d, 0xc2, // VPUNPCKHQDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x68, 0xc2, // VPUNPCKHBW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x68, 0xdb, // VPUNPCKHBW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x68, 0xe4, // VPUNPCKHBW YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x69, 0xc2, // VPUNPCKHWD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x69, 0xdb, // VPUNPCKHWD YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x69, 0xe4, // VPUNPCKHWD YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6a, 0xc2, // VPUNPCKHDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6a, 0xdb, // VPUNPCKHDQ YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x6a, 0xe4, // VPUNPCKHDQ YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6d, 0xc2, // VPUNPCKHQDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6d, 0xdb, // VPUNPCKHQDQ YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x6d, 0xe4, // VPUNPCKHQDQ YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x68, 0x20, // VPUNPCKHBW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x69, 0x20, // VPUNPCKHWD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x6a, 0x20, // VPUNPCKHDQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x6d, 0x20, // VPUNPCKHQDQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckh_vs_vpunpckl_complement() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x60, 0xc2, // VPUNPCKLBW YMM0, YMM1, YMM2 (low)
        0xc5, 0xed, 0x68, 0xdb, // VPUNPCKHBW YMM3, YMM2, YMM3 (high)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhbw_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x68, 0xc2, // VPUNPCKHBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhwd_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x69, 0xc2, // VPUNPCKHWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhdq_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6a, 0xc2, // VPUNPCKHDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckhqdq_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6d, 0xc2, // VPUNPCKHQDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
