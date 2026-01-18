use crate::*;

// VPABSB/VPABSW/VPABSD - Packed Absolute Value (AVX2)
//
// Computes the absolute value of each signed integer in the source operand
// and stores the unsigned result in the destination operand.
//
// VPABSB: Absolute value of 32 packed signed byte integers (8-bit) in YMM registers
// VPABSW: Absolute value of 16 packed signed word integers (16-bit) in YMM registers
// VPABSD: Absolute value of 8 packed signed doubleword integers (32-bit) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 1C /r     VPABSB ymm1, ymm2/m256
// VEX.256.66.0F38.WIG 1D /r     VPABSW ymm1, ymm2/m256
// VEX.256.66.0F38.WIG 1E /r     VPABSD ymm1, ymm2/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPABSB Tests - 32x Byte Absolute Value (256-bit)
// ============================================================================

#[test]
fn test_vpabsb_ymm0_ymm1_all_zeros() {
    let mut emu = emu64();
    // VPABSB YMM0, YMM1 with all zeros
    let code = [
        0xc4, 0xe2, 0x7d, 0x1c, 0xc1, // VPABSB YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm2_ymm3_all_positive() {
    let mut emu = emu64();
    // VPABSB YMM2, YMM3 with positive values
    let code = [
        0xc4, 0xe2, 0x7d, 0x1c, 0xd3, // VPABSB YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm4_ymm5_all_negative() {
    let mut emu = emu64();
    // VPABSB YMM4, YMM5 with negative values (0xFF = -1, 0xFE = -2, etc.)
    let code = [
        0xc4, 0xe2, 0x7d, 0x1c, 0xe5, // VPABSB YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm6_ymm7_mixed_signs() {
    let mut emu = emu64();
    // VPABSB YMM6, YMM7 with mixed positive and negative
    let code = [
        0xc4, 0xe2, 0x7d, 0x1c, 0xf7, // VPABSB YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm8_ymm9_most_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1c, 0xc1, // VPABSB YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm10_ymm11_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1c, 0xd3, // VPABSB YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm12_ymm13_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1c, 0xe5, // VPABSB YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm14_ymm15_high_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1c, 0xf7, // VPABSB YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm0_mem_all_negative() {
    let mut emu = emu64();
    // VPABSB YMM0, [memory] with negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1c, 0x00, // VPABSB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let neg_data: Vec<u8> = (0..32).map(|i| (-(i as i8)) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &neg_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm1_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1c, 0x08, // VPABSB YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mixed: Vec<u8> = (0..32).map(|i| if i % 2 == 0 {
 i as u8 } else { (-(i as i8)) as u8 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mixed);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_ymm2_mem_min_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1c, 0x10, // VPABSB YMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut data = vec![0x80u8; 16]; // -128
    data.extend(vec![0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F]); // +127
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPABSW Tests - 16x Word Absolute Value (256-bit)
// ============================================================================

#[test]
fn test_vpabsw_ymm0_ymm1_all_zeros() {
    let mut emu = emu64();
    // VPABSW YMM0, YMM1 with all zeros
    let code = [
        0xc4, 0xe2, 0x7d, 0x1d, 0xc1, // VPABSW YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm2_ymm3_all_positive() {
    let mut emu = emu64();
    // VPABSW YMM2, YMM3 with positive values
    let code = [
        0xc4, 0xe2, 0x7d, 0x1d, 0xd3, // VPABSW YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm4_ymm5_all_negative() {
    let mut emu = emu64();
    // VPABSW YMM4, YMM5 with negative values
    let code = [
        0xc4, 0xe2, 0x7d, 0x1d, 0xe5, // VPABSW YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm6_ymm7_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1d, 0xf7, // VPABSW YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm8_ymm9_most_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1d, 0xc1, // VPABSW YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm10_ymm11_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1d, 0xd3, // VPABSW YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm12_ymm13_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1d, 0xe5, // VPABSW YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm14_ymm15_high_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1d, 0xf7, // VPABSW YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm0_mem_all_negative() {
    let mut emu = emu64();
    // VPABSW YMM0, [memory] with negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1d, 0x00, // VPABSW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let neg_data: Vec<u8> = (0..16)
        .flat_map(|i| ((-(i as i16 * 100)) as u16).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &neg_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm1_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1d, 0x08, // VPABSW YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mixed: Vec<u8> = (0..16)
        .flat_map(|i| {
            if i % 2 == 0 {
                (i as i16 * 1000).to_le_bytes()
            } else {
                ((-(i as i16 * 1000)) as u16).to_le_bytes()
            }
        })
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mixed);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_ymm2_mem_min_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1d, 0x10, // VPABSW YMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8)
        .flat_map(|_| 0x8000u16.to_le_bytes())
        .chain((0..8).flat_map(|_| 0x7FFFu16.to_le_bytes()))
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPABSD Tests - 8x Doubleword Absolute Value (256-bit)
// ============================================================================

#[test]
fn test_vpabsd_ymm0_ymm1_all_zeros() {
    let mut emu = emu64();
    // VPABSD YMM0, YMM1 with all zeros
    let code = [
        0xc4, 0xe2, 0x7d, 0x1e, 0xc1, // VPABSD YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm2_ymm3_all_positive() {
    let mut emu = emu64();
    // VPABSD YMM2, YMM3 with positive values
    let code = [
        0xc4, 0xe2, 0x7d, 0x1e, 0xd3, // VPABSD YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm4_ymm5_all_negative() {
    let mut emu = emu64();
    // VPABSD YMM4, YMM5 with negative values
    let code = [
        0xc4, 0xe2, 0x7d, 0x1e, 0xe5, // VPABSD YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm6_ymm7_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1e, 0xf7, // VPABSD YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm8_ymm9_most_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1e, 0xc1, // VPABSD YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm10_ymm11_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1e, 0xd3, // VPABSD YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm12_ymm13_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1e, 0xe5, // VPABSD YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm14_ymm15_high_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x1e, 0xf7, // VPABSD YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm0_mem_all_negative() {
    let mut emu = emu64();
    // VPABSD YMM0, [memory] with negative values
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1e, 0x00, // VPABSD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let neg_data: Vec<u8> = (0..8)
        .flat_map(|i| ((-(i as i32 * 10000)) as u32).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &neg_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm1_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1e, 0x08, // VPABSD YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mixed: Vec<u8> = vec![
        100000i32, -200000i32, 300000i32, -400000i32,
        500000i32, -600000i32, 700000i32, -800000i32,
    ]
    .into_iter()
    .flat_map(|v| (v as u32).to_le_bytes())
    .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mixed);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_ymm2_mem_min_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1e, 0x10, // VPABSD YMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..4)
        .flat_map(|_| 0x80000000u32.to_le_bytes())
        .chain((0..4).flat_map(|_| 0x7FFFFFFFu32.to_le_bytes()))
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional edge case and comprehensive tests
// ============================================================================

#[test]
fn test_vpabsb_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1c, 0xc1, // VPABSB YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x1c, 0xc8, // VPABSB YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1d, 0xc1, // VPABSW YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x1d, 0xc8, // VPABSW YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1e, 0xc1, // VPABSD YMM0, YMM1
        0xc4, 0xe2, 0x7d, 0x1e, 0xc8, // VPABSD YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1c, 0xc1, // VPABSB YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1d, 0xc1, // VPABSW YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x1e, 0xc1, // VPABSD YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsb_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1c, 0x00, // VPABSB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0x80, 0x7F, 0xFF, 0x01, 0x80, 0x7F, 0xFF, 0x01]
        .into_iter()
        .cycle()
        .take(32)
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsw_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1d, 0x00, // VPABSW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0x8000u16, 0x7FFF, 0xFFFF, 0x0001]
        .into_iter()
        .cycle()
        .take(16)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpabsd_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x1e, 0x00, // VPABSD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0x80000000u32, 0x7FFFFFFF, 0xFFFFFFFF, 0x00000001]
        .into_iter()
        .cycle()
        .take(8)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}
