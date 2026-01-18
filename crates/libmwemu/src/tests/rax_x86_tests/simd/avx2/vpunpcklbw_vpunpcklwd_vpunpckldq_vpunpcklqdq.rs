use crate::*;

// VPUNPCKLBW/VPUNPCKLWD/VPUNPCKLDQ/VPUNPCKLQDQ - Unpack Low Data (AVX2)
//
// Interleaves low-order elements from two source operands.
// Takes lower half of each source and interleaves them.
//
// VPUNPCKLBW: Unpack and interleave low-order bytes
//             Takes low 16 bytes from each 128-bit lane
//
// VPUNPCKLWD: Unpack and interleave low-order words
//             Takes low 8 words from each 128-bit lane
//
// VPUNPCKLDQ: Unpack and interleave low-order doublewords
//             Takes low 4 dwords from each 128-bit lane
//
// VPUNPCKLQDQ: Unpack and interleave low-order quadwords
//              Takes low 2 qwords from each 128-bit lane
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG 60 /r     VPUNPCKLBW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 61 /r     VPUNPCKLWD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 62 /r     VPUNPCKLDQ ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG 6C /r     VPUNPCKLQDQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPUNPCKLBW Tests - Unpack Low Bytes (256-bit)
// ============================================================================

#[test]
fn test_vpunpcklbw_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKLBW YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x60, 0xc2, // VPUNPCKLBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x60, 0xdd, // VPUNPCKLBW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x60, 0xf0, // VPUNPCKLBW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x60, 0xcb, // VPUNPCKLBW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x60, 0xe6, // VPUNPCKLBW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x60, 0xf9, // VPUNPCKLBW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKLBW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x60, 0x00, // VPUNPCKLBW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x60, 0xc2, // VPUNPCKLBW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x60, 0xdb, // VPUNPCKLBW YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x60, 0xc1, // VPUNPCKLBW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm2_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0x60, 0x10, // VPUNPCKLBW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

// ============================================================================
// VPUNPCKLWD Tests - Unpack Low Words (256-bit)
// ============================================================================

#[test]
fn test_vpunpcklwd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKLWD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x61, 0xc2, // VPUNPCKLWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x61, 0xdd, // VPUNPCKLWD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x61, 0xf0, // VPUNPCKLWD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x61, 0xcb, // VPUNPCKLWD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x61, 0xe6, // VPUNPCKLWD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x61, 0xf9, // VPUNPCKLWD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKLWD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x61, 0x00, // VPUNPCKLWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x61, 0xc2, // VPUNPCKLWD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x61, 0xdb, // VPUNPCKLWD YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x61, 0xc1, // VPUNPCKLWD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPUNPCKLDQ Tests - Unpack Low Doublewords (256-bit)
// ============================================================================

#[test]
fn test_vpunpckldq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKLDQ YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x62, 0xc2, // VPUNPCKLDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x62, 0xdd, // VPUNPCKLDQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x62, 0xf0, // VPUNPCKLDQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x62, 0xcb, // VPUNPCKLDQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x62, 0xe6, // VPUNPCKLDQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x62, 0xf9, // VPUNPCKLDQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKLDQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x62, 0x00, // VPUNPCKLDQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x62, 0xc2, // VPUNPCKLDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x62, 0xdb, // VPUNPCKLDQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x62, 0xc1, // VPUNPCKLDQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPUNPCKLQDQ Tests - Unpack Low Quadwords (256-bit)
// ============================================================================

#[test]
fn test_vpunpcklqdq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPUNPCKLQDQ YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x6c, 0xc2, // VPUNPCKLQDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdd, 0x6c, 0xdd, // VPUNPCKLQDQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm6_ymm7_ymm8() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0x6c, 0xf0, // VPUNPCKLQDQ YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x6c, 0xcb, // VPUNPCKLQDQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0x6c, 0xe6, // VPUNPCKLQDQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0x6c, 0xf9, // VPUNPCKLQDQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPUNPCKLQDQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0x6c, 0x00, // VPUNPCKLQDQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..4).flat_map(|i| (i as u64).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6c, 0xc2, // VPUNPCKLQDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6c, 0xdb, // VPUNPCKLQDQ YMM3, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_same_source() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6c, 0xc1, // VPUNPCKLQDQ YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpunpckl_mixed_sizes() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x60, 0xc2, // VPUNPCKLBW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x61, 0xdb, // VPUNPCKLWD YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x62, 0xe4, // VPUNPCKLDQ YMM4, YMM3, YMM4
        0xc5, 0xdd, 0x6c, 0xed, // VPUNPCKLQDQ YMM5, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0x60, 0xc1, // VPUNPCKLBW YMM8, YMM8, YMM9
        0xc4, 0x41, 0x15, 0x60, 0xee, // VPUNPCKLBW YMM13, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x35, 0x61, 0xcb, // VPUNPCKLWD YMM9, YMM9, YMM11
        0xc4, 0x41, 0x0d, 0x61, 0xf7, // VPUNPCKLWD YMM14, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0x62, 0xd4, // VPUNPCKLDQ YMM10, YMM10, YMM12
        0xc4, 0x41, 0x05, 0x62, 0xf8, // VPUNPCKLDQ YMM15, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x25, 0x6c, 0xdd, // VPUNPCKLQDQ YMM11, YMM11, YMM13
        0xc4, 0x41, 0x05, 0x6c, 0xf9, // VPUNPCKLQDQ YMM15, YMM15, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x60, 0xc2, // VPUNPCKLBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x61, 0xc2, // VPUNPCKLWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x62, 0xc2, // VPUNPCKLDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_all_zeros() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6c, 0xc2, // VPUNPCKLQDQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x60, 0xc2, // VPUNPCKLBW YMM0, YMM1, YMM2
        0xc5, 0xed, 0x60, 0xdb, // VPUNPCKLBW YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x60, 0xe4, // VPUNPCKLBW YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x61, 0xc2, // VPUNPCKLWD YMM0, YMM1, YMM2
        0xc5, 0xed, 0x61, 0xdb, // VPUNPCKLWD YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x61, 0xe4, // VPUNPCKLWD YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x62, 0xc2, // VPUNPCKLDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x62, 0xdb, // VPUNPCKLDQ YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x62, 0xe4, // VPUNPCKLDQ YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_sequential() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0x6c, 0xc2, // VPUNPCKLQDQ YMM0, YMM1, YMM2
        0xc5, 0xed, 0x6c, 0xdb, // VPUNPCKLQDQ YMM3, YMM2, YMM3
        0xc5, 0xe5, 0x6c, 0xe4, // VPUNPCKLQDQ YMM4, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklbw_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x60, 0x20, // VPUNPCKLBW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklwd_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x61, 0x20, // VPUNPCKLWD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpckldq_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x62, 0x20, // VPUNPCKLDQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpunpcklqdq_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0x6c, 0x20, // VPUNPCKLQDQ YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}
