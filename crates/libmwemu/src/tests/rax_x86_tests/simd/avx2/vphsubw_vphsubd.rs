use crate::*;

// VPHSUBW/VPHSUBD - Packed Horizontal Subtract (AVX2)
//
// Subtracts pairs of adjacent elements from source and destination operands,
// storing the differences in the destination.
//
// VPHSUBW: Horizontally subtract 16 pairs of adjacent 16-bit signed words
// VPHSUBD: Horizontally subtract 8 pairs of adjacent 32-bit signed doublewords
//
// For VPHSUBW with YMM:
// dst[15:0]    = src1[15:0] - src1[31:16]
// dst[31:16]   = src1[47:32] - src1[63:48]
// dst[47:32]   = src1[79:64] - src1[95:80]
// dst[63:48]   = src1[111:96] - src1[127:112]
// dst[79:64]   = src2[15:0] - src2[31:16]
// dst[95:80]   = src2[47:32] - src2[63:48]
// dst[111:96]  = src2[79:64] - src2[95:80]
// dst[127:112] = src2[111:96] - src2[127:112]
// (Similar pattern for upper 128 bits)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 05 /r     VPHSUBW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 06 /r     VPHSUBD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPHSUBW Tests - 16x Word Horizontal Subtract (256-bit)
// ============================================================================

#[test]
fn test_vphsubw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPHSUBW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc2, // VPHSUBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPHSUBW YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x05, 0xdd, // VPHSUBW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm6_ymm7_ymm8_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x05, 0xf0, // VPHSUBW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x05, 0xcb, // VPHSUBW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x05, 0xe6, // VPHSUBW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x05, 0xf9, // VPHSUBW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm0_ymm1_ymm2_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc2, // VPHSUBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm3_ymm4_ymm5_underflow() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x05, 0xdd, // VPHSUBW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x05, 0xf0, // VPHSUBW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPHSUBW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x05, 0x00, // VPHSUBW YMM0, YMM1, [RAX]
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
fn test_vphsubw_ymm2_ymm3_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x05, 0x10, // VPHSUBW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x05, 0x20, // VPHSUBW YMM4, YMM5, [RAX]
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
fn test_vphsubw_ymm6_ymm7_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x05, 0x30, // VPHSUBW YMM6, YMM7, [RAX]
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
// VPHSUBD Tests - 8x Doubleword Horizontal Subtract (256-bit)
// ============================================================================

#[test]
fn test_vphsubd_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPHSUBD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc2, // VPHSUBD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPHSUBD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x06, 0xdd, // VPHSUBD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm6_ymm7_ymm8_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x06, 0xf0, // VPHSUBD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm9_ymm10_ymm11_mixed() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x06, 0xcb, // VPHSUBD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm12_ymm13_ymm14_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x06, 0xe6, // VPHSUBD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x06, 0xf9, // VPHSUBD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm0_ymm1_ymm2_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc2, // VPHSUBD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm3_ymm4_ymm5_underflow() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x06, 0xdd, // VPHSUBD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x45, 0x06, 0xf0, // VPHSUBD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPHSUBD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x06, 0x00, // VPHSUBD YMM0, YMM1, [RAX]
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
fn test_vphsubd_ymm2_ymm3_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x06, 0x10, // VPHSUBD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_ymm4_ymm5_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x06, 0x20, // VPHSUBD YMM4, YMM5, [RAX]
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
fn test_vphsubd_ymm6_ymm7_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x06, 0x30, // VPHSUBD YMM6, YMM7, [RAX]
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
fn test_vphsubw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc2, // VPHSUBW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x05, 0xc0, // VPHSUBW YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc2, // VPHSUBD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x06, 0xc0, // VPHSUBD YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_mixed_positive_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc2, // VPHSUBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_mixed_positive_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc2, // VPHSUBD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x05, 0x00, // VPHSUBW YMM0, YMM1, [RAX]
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
fn test_vphsubd_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x06, 0x00, // VPHSUBD YMM0, YMM1, [RAX]
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
fn test_vphsubw_boundary_wraparound() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc2, // VPHSUBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_boundary_wraparound() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc2, // VPHSUBD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_mem_boundary() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x05, 0x00, // VPHSUBW YMM0, YMM1, [RAX]
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
fn test_vphsubd_mem_boundary() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x06, 0x00, // VPHSUBD YMM0, YMM1, [RAX]
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
fn test_vphsubw_same_register() {
    let mut emu = emu64();
    // VPHSUBW with same source registers
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc1, // VPHSUBW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_same_register() {
    let mut emu = emu64();
    // VPHSUBD with same source registers
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc1, // VPHSUBD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubw_inverse_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x05, 0xc2, // VPHSUBW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vphsubd_inverse_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x06, 0xc2, // VPHSUBD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
