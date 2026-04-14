use crate::*;

// VPMAXUB/VPMAXUW/VPMAXUD - Maximum of Packed Unsigned Integers (AVX2)
//
// Compare packed unsigned integers from source and destination operands and
// return the maximum values. Stores packed maximum results in destination.
//
// VPMAXUB: Maximum of 32 packed unsigned byte integers (8-bit each) in YMM registers
// VPMAXUW: Maximum of 16 packed unsigned word integers (16-bit each) in YMM registers
// VPMAXUD: Maximum of 8 packed unsigned doubleword integers (32-bit each) in YMM registers
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG DE /r       VPMAXUB ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 3E /r     VPMAXUW ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.WIG 3F /r     VPMAXUD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMAXUB Tests - 32x Unsigned Byte Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxub_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMAXUB YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xde, 0xc2, // VPMAXUB YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm3_ymm4_ymm5_small_values() {
    let mut emu = emu64();
    // VPMAXUB YMM3, YMM4, YMM5 with small values
    let code = [
        0xc5, 0xdd, 0xde, 0xdd, // VPMAXUB YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xde, 0xf0, // VPMAXUB YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm9_ymm10_ymm11_mixed_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xde, 0xcb, // VPMAXUB YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm12_ymm13_ymm14_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xde, 0xe6, // VPMAXUB YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xde, 0xf9, // VPMAXUB YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMAXUB YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xde, 0x00, // VPMAXUB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm2_ymm3_mem_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xde, 0x10, // VPMAXUB YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xde, 0x20, // VPMAXUB YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let seq: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &seq);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xde, 0x30, // VPMAXUB YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0xFF } else { 0x00 }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMAXUW Tests - 16x Unsigned Word Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxuw_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMAXUW YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3e, 0xc2, // VPMAXUW YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm3_ymm4_ymm5_small_values() {
    let mut emu = emu64();
    // VPMAXUW YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x3e, 0xdd, // VPMAXUW YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x45, 0x3e, 0xf0, // VPMAXUW YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm9_ymm10_ymm11_mixed_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x3e, 0xcb, // VPMAXUW YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm12_ymm13_ymm14_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x3e, 0xe6, // VPMAXUW YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x3e, 0xf9, // VPMAXUW YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMAXUW YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3e, 0x00, // VPMAXUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| ((i * 0x1111u16) as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm2_ymm3_mem_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3e, 0x10, // VPMAXUW YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3e, 0x20, // VPMAXUW YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3e, 0x30, // VPMAXUW YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0xFFFFu16 } else { 0x0000u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMAXUD Tests - 8x Unsigned Doubleword Maximum (256-bit)
// ============================================================================

#[test]
fn test_vpmaxud_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMAXUD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc4, 0xe2, 0x75, 0x3f, 0xc2, // VPMAXUD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm3_ymm4_ymm5_small_values() {
    let mut emu = emu64();
    // VPMAXUD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x3f, 0xdd, // VPMAXUD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm6_ymm7_ymm8_max_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x45, 0x3f, 0xf0, // VPMAXUD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm9_ymm10_ymm11_mixed_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x3f, 0xcb, // VPMAXUD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm12_ymm13_ymm14_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x3f, 0xe6, // VPMAXUD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x3f, 0xf9, // VPMAXUD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMAXUD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3f, 0x00, // VPMAXUD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| ((i * 0x11111111u32) as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm2_ymm3_mem_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x3f, 0x10, // VPMAXUD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x3f, 0x20, // VPMAXUD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).flat_map(|i| (i as u32).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x45, 0x3f, 0x30, // VPMAXUD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..8).flat_map(|i| if i % 2 == 0 { 0xFFFFFFFFu32 } else { 0x00000000u32 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests mixing different operations
// ============================================================================

#[test]
fn test_vpmaxub_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xde, 0xc2, // VPMAXUB YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xde, 0xc3, // VPMAXUB YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3e, 0xc2, // VPMAXUW YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3e, 0xc3, // VPMAXUW YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3f, 0xc2, // VPMAXUD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x3f, 0xc3, // VPMAXUD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_mem_unaligned_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xde, 0x00, // VPMAXUB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3e, 0x00, // VPMAXUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0xAAAAu16 } else { 0x5555u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_mem_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3f, 0x00, // VPMAXUD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..8).flat_map(|i| (1u32 << i).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_reverse_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xde, 0x00, // VPMAXUB YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..32).rev().collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3e, 0x00, // VPMAXUW YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0xFFFEu16, 0xFFFFu16,
        0x7FFFu16, 0x8000u16, 0x8001u16, 0x7FFEu16,
        0x0000u16, 0x0001u16, 0xFFFEu16, 0xFFFFu16,
        0x7FFFu16, 0x8000u16, 0x8001u16, 0x7FFEu16,
    ].into_iter().flat_map(|v| v.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &boundary);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_large_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x3f, 0x00, // VPMAXUD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let large_vals: Vec<u8> = vec![
        0xFFFFFFFFu32,
        0xFFFFFFFEu32,
        0x80000000u32,
        0x80000001u32,
        0x7FFFFFFFu32,
        0x7FFFFFFEu32,
        0x00000001u32,
        0x00000000u32,
    ].into_iter().flat_map(|v| v.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &large_vals);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xde, 0xc1, // VPMAXUB YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3e, 0xc1, // VPMAXUW YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_all_same_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x3f, 0xc1, // VPMAXUD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxub_extended_regs_r8_r9_r10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0xde, 0xc2, // VPMAXUB YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxuw_extended_regs_r11_r12_r13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x1d, 0x3e, 0xdd, // VPMAXUW YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaxud_extended_regs_r14_r15_r8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x05, 0x3f, 0xf0, // VPMAXUD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
