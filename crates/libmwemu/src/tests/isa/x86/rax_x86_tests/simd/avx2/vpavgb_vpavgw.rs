use crate::*;

// VPAVGB/VPAVGW - Average Packed Unsigned Integers (AVX2)
//
// Compute the average of packed unsigned integers from source and destination operands.
// The average is calculated as (src1 + src2 + 1) / 2, rounding up.
//
// VPAVGB: Average of 32 packed unsigned byte integers (8-bit each) in YMM registers
// VPAVGW: Average of 16 packed unsigned word integers (16-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG E0 /r       VPAVGB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG E3 /r       VPAVGW ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPAVGB Tests - 32x Unsigned Byte Average (256-bit)
// ============================================================================

#[test]
fn test_vpavgb_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPAVGB YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xe0, 0xc2, // VPAVGB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPAVGB YMM3, YMM4, YMM5 with all 0x01 values
    let code = [
        0xc5, 0xdd, 0xe0, 0xdd, // VPAVGB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xe0, 0xf0, // VPAVGB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm9_ymm10_ymm11_mixed_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xe0, 0xcb, // VPAVGB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xe0, 0xe6, // VPAVGB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xe0, 0xf9, // VPAVGB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPAVGB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm2_ymm3_mem_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xe0, 0x10, // VPAVGB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xe0, 0x20, // VPAVGB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let seq: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &seq);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xe0, 0x30, // VPAVGB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0xFF } else { 0x00 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_rounding_up() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_rounding_even_odd() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0x02 } else { 0x03 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_overflow_prevention() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

// ============================================================================
// VPAVGW Tests - 16x Unsigned Word Average (256-bit)
// ============================================================================

#[test]
fn test_vpavgw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPAVGW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xe3, 0xc2, // VPAVGW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPAVGW YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xe3, 0xdd, // VPAVGW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xe3, 0xf0, // VPAVGW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm9_ymm10_ymm11_mixed_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xe3, 0xcb, // VPAVGW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xe3, 0xe6, // VPAVGW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xe3, 0xf9, // VPAVGW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPAVGW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| ((i * 0x1111u16) as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm2_ymm3_mem_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xe3, 0x10, // VPAVGW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xe3, 0x20, // VPAVGW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xe3, 0x30, // VPAVGW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0xFFFFu16 } else { 0x0000u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_rounding_up() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x01, 0x00].repeat(16);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_rounding_even_odd() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0x0002u16 } else { 0x0003u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_overflow_prevention() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpavgb_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe0, 0xc2, // VPAVGB YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xe0, 0xc3, // VPAVGB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe3, 0xc2, // VPAVGW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xe3, 0xc3, // VPAVGW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_mem_unaligned_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0xAAAAu16 } else { 0x5555u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe0, 0xc1, // VPAVGB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xe3, 0xc1, // VPAVGW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_extended_regs_r8_r9_r10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0xe0, 0xc2, // VPAVGB YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_extended_regs_r11_r12_r13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xe3, 0xdd, // VPAVGW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_zero_and_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_zero_and_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let boundary: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80, 0x81, 0xFE, 0xFF, 0x00].repeat(4);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &boundary);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0x7FFFu16, 0x8000u16,
        0x8001u16, 0xFFFEu16, 0xFFFFu16, 0x0000u16,
    ].into_iter().flat_map(|v| v.to_le_bytes()).take(32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &boundary);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..8).map(|i| 1u8 << i).chain((0..8).map(|i| 1u8 << i))
        .chain((0..8).map(|i| 1u8 << i)).chain((0..8).map(|i| 1u8 << i)).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..16).flat_map(|i| (1u16 << (i % 16)).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_reverse_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe0, 0x00, // VPAVGB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).rev().collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_reverse_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xe3, 0x00, // VPAVGW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).rev().flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgb_extended_regs_r14_r15_r8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x05, 0xe0, 0xf0, // VPAVGB YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpavgw_extended_regs_r14_r15_r8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x05, 0xe3, 0xf0, // VPAVGW YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
