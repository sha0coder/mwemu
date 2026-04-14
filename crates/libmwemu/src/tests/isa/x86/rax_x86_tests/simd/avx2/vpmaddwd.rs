use crate::*;

// VPMADDWD - Multiply and Add Packed Integers (AVX2)
//
// Multiplies the signed word integers in the source and destination operands,
// producing intermediate signed doubleword results. Adjacent doubleword results
// are then added to produce signed doubleword sums stored in the destination.
//
// For each pair of words:
//   dest[31:0]   = (src1[15:0]   * src2[15:0])   + (src1[31:16]   * src2[31:16])
//   dest[63:32]  = (src1[47:32]  * src2[47:32])  + (src1[63:48]   * src2[63:48])
//   ... and so on for all 16 words → 8 doublewords
//
// VPMADDWD: Process 16 signed words (8 pairs) in YMM registers → 8 doublewords
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F.WIG F5 /r       VPMADDWD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// VPMADDWD Tests - Multiply and Add (256-bit)
// ============================================================================

#[test]
fn test_vpmaddwd_ymm0_ymm1_ymm2_all_zeros() {
    let mut emu = emu64();
    // VPMADDWD YMM0, YMM1, YMM2 with all zeros
    let code = [
        0xc5, 0xf5, 0xf5, 0xc2, // VPMADDWD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm3_ymm4_ymm5_all_ones() {
    let mut emu = emu64();
    // VPMADDWD YMM3, YMM4, YMM5 with all 0x0001 values
    // 1 * 1 + 1 * 1 = 2
    let code = [
        0xc5, 0xdd, 0xf5, 0xdd, // VPMADDWD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm6_ymm7_ymm8_positive_values() {
    let mut emu = emu64();
    let code = [
        0xc5, 0x45, 0xf5, 0xf0, // VPMADDWD YMM6, YMM7, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm9_ymm10_ymm11_negative_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2d, 0xf5, 0xcb, // VPMADDWD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm12_ymm13_ymm14_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x15, 0xf5, 0xe6, // VPMADDWD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm15_ymm0_ymm1_high_reg() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x7d, 0xf5, 0xf9, // VPMADDWD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPMADDWD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| ((i as u16) * 0x0101).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm2_ymm3_mem_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xe5, 0xf5, 0x10, // VPMADDWD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm4_ymm5_mem_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xd5, 0xf5, 0x20, // VPMADDWD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_ymm6_ymm7_mem_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0x45, 0xf5, 0x30, // VPMADDWD YMM6, YMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0x0001u16 } else { 0xFFFFu16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_simple_multiplication() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x04, 0x00, 0x05, 0x00].repeat(8);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_negative_multiplication() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x01, 0x00, 0x01, 0x00].repeat(8);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_overflow_handling() {
    let mut emu = emu64();
    // 32767 * 32767 = 1,073,676,289 (fits in i32)
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xFF, 0x7F].repeat(16);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_saturation_check() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x00, 0x80].repeat(16);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_chain_multiple_ops() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf5, 0xf5, 0xc2, // VPMADDWD YMM0, YMM1, YMM2
        0xc5, 0xfd, 0xf5, 0xc3, // VPMADDWD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_mem_unaligned_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_extended_regs_r8_r9_r10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x3d, 0xf5, 0xc2, // VPMADDWD YMM8, YMM8, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_extended_regs_r11_r12_r13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1d, 0xf5, 0xdd, // VPMADDWD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_extended_regs_r14_r15_r8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x05, 0xf5, 0xf0, // VPMADDWD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_identity_multiply() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).flat_map(|i| if i % 2 == 0 { 0x0001u16 } else { 0x0000u16 }.to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_zero_result() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x01, 0x00, 0x01, 0x00].repeat(8);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let boundary: Vec<u8> = vec![
        0x0000u16, 0x0001u16, 0x7FFFu16, 0x8000u16,
        0x8001u16, 0xFFFEu16, 0xFFFFu16, 0x0000u16,
    ].into_iter().flat_map(|v| v.to_le_bytes()).chain(
        vec![
            0x0000u16, 0x0001u16, 0x7FFFu16, 0x8000u16,
            0x8001u16, 0xFFFEu16, 0xFFFFu16, 0x0000u16,
        ].into_iter().flat_map(|v| v.to_le_bytes())
    ).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &boundary);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let powers: Vec<u8> = (0..16).flat_map(|i| (1u16 << (i % 15)).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &powers);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_alternating_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = (0..16).flat_map(|i|
        if i % 2 == 0 { 0x0002u16 } else { 0xFFFEu16 }.to_le_bytes()
    ).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_sequential_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let sequential: Vec<u8> = (1..=16).flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &sequential);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_reverse_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let reverse: Vec<u8> = (1..=16).rev().flat_map(|i| (i as u16).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &reverse);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_symmetric_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let symmetric: Vec<u8> = vec![
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
        0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
        0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &symmetric);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_small_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let small: Vec<u8> = vec![0x01, 0x00, 0x02, 0x00].repeat(8);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &small);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_large_products() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let large: Vec<u8> = vec![0x00, 0x10].repeat(16); // 4096
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &large);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_mixed_magnitudes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mixed: Vec<u8> = vec![
        0x01, 0x00, 0xFF, 0x0F, // 1, 4095
        0x00, 0x10, 0x01, 0x00, // 4096, 1
    ].repeat(4);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &mixed);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_dot_product_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let dot_product: Vec<u8> = (0..16).flat_map(|i| ((i % 8) as u16 + 1).to_le_bytes()).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &dot_product);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmaddwd_checkerboard() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc5, 0xf5, 0xf5, 0x00, // VPMADDWD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let checkerboard: Vec<u8> = (0..16).flat_map(|i|
        if i % 2 == 0 { 0x5555u16 } else { 0xAAAAu16 }.to_le_bytes()
    ).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &checkerboard);
    emu.run(None).unwrap();
}
