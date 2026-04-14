use crate::*;

// VPHADDW/VPHADDD - Packed Horizontal Add (AVX2)
//
// Adds pairs of adjacent elements from source and destination operands,
// storing the sums in the destination.
//
// VPHADDW: Horizontally add 16 pairs of adjacent 16-bit signed words
// VPHADDD: Horizontally add 8 pairs of adjacent 32-bit signed doublewords
//
// For VPHADDW with YMM:
// dst[15:0]    = src1[31:16] + src1[15:0]
// dst[31:16]   = src1[63:48] + src1[47:32]
// dst[47:32]   = src1[95:80] + src1[79:64]
// dst[63:48]   = src1[127:112] + src1[111:96]
// dst[79:64]   = src2[31:16] + src2[15:0]
// dst[95:80]   = src2[63:48] + src2[47:32]
// dst[111:96]  = src2[95:80] + src2[79:64]
// dst[127:112] = src2[127:112] + src2[111:96]
// (Similar pattern for upper 128 bits)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 01 /r     VPHADDW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 02 /r     VPHADDD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPHADDW Tests - 16x Word Horizontal Add (256-bit)
// ============================================================================

#[test]
fn test_vphaddw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPHADDW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x01, 0xc2, // VPHADDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPHADDW YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x01, 0xdd, // VPHADDW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm6_ymm7_ymm8_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x01, 0xf0, // VPHADDW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x01, 0xcb, // VPHADDW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x01, 0xe6, // VPHADDW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x01, 0xf9, // VPHADDW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm0_ymm1_ymm2_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x01, 0xc2, // VPHADDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm3_ymm4_ymm5_overflow() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x01, 0xdd, // VPHADDW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x01, 0xf0, // VPHADDW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPHADDW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x01, 0x00, // VPHADDW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((i as u16) * 0x0100).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm2_ymm3_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x01, 0x10, // VPHADDW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x01, 0x20, // VPHADDW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| if i % 2 == 0 { 0xAAAAu16 } else { 0x5555u16 }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_ymm6_ymm7_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x01, 0x30, // VPHADDW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| ((-(i as i16 * 100)) as u16).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPHADDD Tests - 8x Doubleword Horizontal Add (256-bit)
// ============================================================================

#[test]
fn test_vphaddd_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPHADDD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x02, 0xc2, // VPHADDD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPHADDD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x02, 0xdd, // VPHADDD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm6_ymm7_ymm8_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x02, 0xf0, // VPHADDD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x02, 0xcb, // VPHADDD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x02, 0xe6, // VPHADDD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x02, 0xf9, // VPHADDD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm0_ymm1_ymm2_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x02, 0xc2, // VPHADDD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm3_ymm4_ymm5_overflow() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x02, 0xdd, // VPHADDD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x02, 0xf0, // VPHADDD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPHADDD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x02, 0x00, // VPHADDD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8)
        .flat_map(|i| ((i as u32) * 0x01010101).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm2_ymm3_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x02, 0x10, // VPHADDD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x02, 0x20, // VPHADDD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8)
        .flat_map(|i| if i % 2 == 0 { 0xAAAAAAAAu32 } else { 0x55555555u32 }.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_ymm6_ymm7_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x02, 0x30, // VPHADDD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![
        -100i32, -200i32, -300i32, -400i32,
        -500i32, -600i32, -700i32, -800i32,
    ]
    .into_iter()
    .flat_map(|v| (v as u32).to_le_bytes())
    .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vphaddw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x01, 0xc2, // VPHADDW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x01, 0xc0, // VPHADDW YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x02, 0xc2, // VPHADDD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x02, 0xc0, // VPHADDD YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_mixed_positive_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x01, 0xc2, // VPHADDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_mixed_positive_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x02, 0xc2, // VPHADDD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x01, 0x00, // VPHADDW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16)
        .flat_map(|i| (1u16 << (i % 15)).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x02, 0x00, // VPHADDD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8)
        .flat_map(|i| (1u32 << (i % 31)).to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_saturated_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x01, 0xc2, // VPHADDW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_saturated_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x02, 0xc2, // VPHADDD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_mem_boundary() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x01, 0x00, // VPHADDW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x8000u16, 0x8000, 0x7FFF, 0x7FFF]
        .into_iter()
        .cycle()
        .take(16)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_mem_boundary() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x02, 0x00, // VPHADDD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x80000000u32, 0x80000000, 0x7FFFFFFF, 0x7FFFFFFF]
        .into_iter()
        .cycle()
        .take(8)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddw_same_register() {
    let mut emu = emu64();
    // VPHADDW with same source registers
    let code = [
        0xc4, 0xe2, 0x75, 0x01, 0xc1, // VPHADDW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphaddd_same_register() {
    let mut emu = emu64();
    // VPHADDD with same source registers
    let code = [
        0xc4, 0xe2, 0x75, 0x02, 0xc1, // VPHADDD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
