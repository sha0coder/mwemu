use crate::*;

// VPMAXSB/VPMAXSW/VPMAXSD - Maximum of Packed Signed Integers (AVX2)
//
// Compare packed signed integers from source and destination operands and
// return the maximum values. Stores packed maximum results in destination.
//
// VPMAXSB: Maximum of 32 packed signed byte integers (8-bit each) in YMM registers
// VPMAXSW: Maximum of 16 packed signed word integers (16-bit each) in YMM registers
// VPMAXSD: Maximum of 8 packed signed doubleword integers (32-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 3C /r     VPMAXSB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F.WIG EE /r       VPMAXSW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 3D /r     VPMAXSD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMAXSB Tests - 32x Signed Byte Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxsb_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMAXSB YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3c, 0xc2, // VPMAXSB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm3_ymm4_ymm5_positive_values() {
    let mut emu = emu64();
    // VPMAXSB YMM3, YMM4, YMM5 with positive values
    let code = [
        0xc4, 0xe2, 0x5d, 0x3c, 0xdd, // VPMAXSB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm6_ymm7_ymm8_negative_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x45, 0x3c, 0xf0, // VPMAXSB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm9_ymm10_ymm11_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x3c, 0xcb, // VPMAXSB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm12_ymm13_ymm14_min_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x3c, 0xe6, // VPMAXSB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x3c, 0xf9, // VPMAXSB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMAXSB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3c, 0x00, // VPMAXSB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm2_ymm3_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3c, 0x10, // VPMAXSB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3c, 0x20, // VPMAXSB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let seq: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &seq);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3c, 0x30, // VPMAXSB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0x7F } else { 0x80 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMAXSW Tests - 16x Signed Word Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxsw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMAXSW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xee, 0xc2, // VPMAXSW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm3_ymm4_ymm5_positive_values() {
    let mut emu = emu64();
    // VPMAXSW YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0xee, 0xdd, // VPMAXSW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm6_ymm7_ymm8_negative_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xee, 0xf0, // VPMAXSW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm9_ymm10_ymm11_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xee, 0xcb, // VPMAXSW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm12_ymm13_ymm14_min_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xee, 0xe6, // VPMAXSW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xee, 0xf9, // VPMAXSW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMAXSW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xee, 0x00, // VPMAXSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| ((i * 0x1111u16) as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm2_ymm3_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xee, 0x10, // VPMAXSW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xee, 0x20, // VPMAXSW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xee, 0x30, // VPMAXSW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0x7FFFu16 } else { 0x8000u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMAXSD Tests - 8x Signed Doubleword Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxsd_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMAXSD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3d, 0xc2, // VPMAXSD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm3_ymm4_ymm5_positive_values() {
    let mut emu = emu64();
    // VPMAXSD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x3d, 0xdd, // VPMAXSD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm6_ymm7_ymm8_negative_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x45, 0x3d, 0xf0, // VPMAXSD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm9_ymm10_ymm11_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x3d, 0xcb, // VPMAXSD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm12_ymm13_ymm14_min_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x3d, 0xe6, // VPMAXSD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x3d, 0xf9, // VPMAXSD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMAXSD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3d, 0x00, // VPMAXSD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| ((i * 0x11111111u32) as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm2_ymm3_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3d, 0x10, // VPMAXSD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3d, 0x20, // VPMAXSD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3d, 0x30, // VPMAXSD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..8).flat_map(|i| if i % 2 == 0 { 0x7FFFFFFFu32 } else { 0x80000000u32 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests mixing different operations
// ============================================================================

#[test]
fn test_vpmaxsb_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3c, 0xc2, // VPMAXSB YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3c, 0xc3, // VPMAXSB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xee, 0xc2, // VPMAXSW YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xee, 0xc3, // VPMAXSW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3d, 0xc2, // VPMAXSD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3d, 0xc3, // VPMAXSD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_mem_unaligned_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3c, 0x00, // VPMAXSB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xee, 0x00, // VPMAXSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0xAAAAu16 } else { 0x5555u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3d, 0x00, // VPMAXSD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_mixed_signs_comprehensive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3c, 0x00, // VPMAXSB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 4 < 2 { (i as i8) as u8 } else { (-(i as i8)) as u8 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xee, 0x00, // VPMAXSW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0x7FFEu16, 0x7FFFu16,
        0x8000u16, 0x8001u16, 0xFFFEu16, 0xFFFFu16,
        0x0000u16, 0x0001u16, 0x7FFEu16, 0x7FFFu16,
        0x8000u16, 0x8001u16, 0xFFFEu16, 0xFFFFu16,
    ].into_iter().flat_map(|v| v.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &boundary);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_large_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3d, 0x00, // VPMAXSD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let large_vals: Vec<u8> = vec![
        0x80000000u32,
        0x80000001u32,
        0xFFFFFFFFu32,
        0x00000000u32,
        0x7FFFFFFFu32,
        0x7FFFFFFEu32,
        0x00000001u32,
        0xFFFFFFFEu32,
    ].into_iter().flat_map(|v| v.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &large_vals);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3c, 0xc1, // VPMAXSB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xee, 0xc1, // VPMAXSW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3d, 0xc1, // VPMAXSD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsb_extended_regs_r8_r9_r10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x3d, 0x3c, 0xc2, // VPMAXSB YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsw_extended_regs_r11_r12_r13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xee, 0xdd, // VPMAXSW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxsd_extended_regs_r14_r15_r8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x05, 0x3d, 0xf0, // VPMAXSD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
