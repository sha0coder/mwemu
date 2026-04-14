use crate::*;

// VPALIGNR - Packed Align Right (AVX2)
//
// Concatenates the destination and source operands into an intermediate value,
// extracts a byte-aligned result shifted to the right by a constant number of bytes,
// and writes the result to the destination.
//
// For each 128-bit lane:
// temp = src2:src1 (concatenated 256 bits)
// dst = temp >> (imm8 * 8)  // Right shift by imm8 bytes
//
// The operation is performed independently on each 128-bit lane.
//
// VPALIGNR: Align 32 bytes from concatenated operands
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F3A.WIG 0F /r ib     VPALIGNR ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPALIGNR Tests - Packed Align Right (256-bit)
// ============================================================================

#[test]
fn test_vpalignr_ymm0_ymm1_ymm2_shift_0() {
    let mut emu = emu64();
    // VPALIGNR YMM0, YMM1, YMM2, 0 (no shift, copy src2)
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x00, // VPALIGNR YMM0, YMM1, YMM2, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm3_ymm4_ymm5_shift_1() {
    let mut emu = emu64();
    // VPALIGNR YMM3, YMM4, YMM5, 1 (shift right by 1 byte)
    let code = [
        0xc4, 0xe3, 0x5d, 0x0f, 0xdd, 0x01, // VPALIGNR YMM3, YMM4, YMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm6_ymm7_ymm8_shift_8() {
    let mut emu = emu64();
    // VPALIGNR YMM6, YMM7, YMM8, 8 (shift right by 8 bytes)
    let code = [
        0xc4, 0xc3, 0x45, 0x0f, 0xf0, 0x08, // VPALIGNR YMM6, YMM7, YMM8, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm9_ymm10_ymm11_shift_15() {
    let mut emu = emu64();
    // VPALIGNR YMM9, YMM10, YMM11, 15 (shift right by 15 bytes)
    let code = [
        0xc4, 0x43, 0x2d, 0x0f, 0xcb, 0x0f, // VPALIGNR YMM9, YMM10, YMM11, 15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm12_ymm13_ymm14_shift_16() {
    let mut emu = emu64();
    // VPALIGNR YMM12, YMM13, YMM14, 16 (shift by full 128 bits, copy src1)
    let code = [
        0xc4, 0x43, 0x15, 0x0f, 0xe6, 0x10, // VPALIGNR YMM12, YMM13, YMM14, 16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm15_ymm0_ymm1_shift_32() {
    let mut emu = emu64();
    // VPALIGNR YMM15, YMM0, YMM1, 32 (shift > 16, result is zero for that lane)
    let code = [
        0xc4, 0x63, 0x7d, 0x0f, 0xf9, 0x20, // VPALIGNR YMM15, YMM0, YMM1, 32
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm0_ymm1_ymm2_shift_2() {
    let mut emu = emu64();
    // VPALIGNR YMM0, YMM1, YMM2, 2
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x02, // VPALIGNR YMM0, YMM1, YMM2, 2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm3_ymm4_ymm5_shift_4() {
    let mut emu = emu64();
    // VPALIGNR YMM3, YMM4, YMM5, 4
    let code = [
        0xc4, 0xe3, 0x5d, 0x0f, 0xdd, 0x04, // VPALIGNR YMM3, YMM4, YMM5, 4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm6_ymm7_ymm8_shift_7() {
    let mut emu = emu64();
    // VPALIGNR YMM6, YMM7, YMM8, 7
    let code = [
        0xc4, 0xc3, 0x45, 0x0f, 0xf0, 0x07, // VPALIGNR YMM6, YMM7, YMM8, 7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm9_ymm10_ymm11_shift_12() {
    let mut emu = emu64();
    // VPALIGNR YMM9, YMM10, YMM11, 12
    let code = [
        0xc4, 0x43, 0x2d, 0x0f, 0xcb, 0x0c, // VPALIGNR YMM9, YMM10, YMM11, 12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm0_ymm1_mem_shift_1() {
    let mut emu = emu64();
    // VPALIGNR YMM0, YMM1, [memory], 1
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x01, // VPALIGNR YMM0, YMM1, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm2_ymm3_mem_shift_8() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x65, 0x0f, 0x10, 0x08, // VPALIGNR YMM2, YMM3, [RAX], 8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i * 2).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm4_ymm5_mem_shift_15() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x55, 0x0f, 0x20, 0x0f, // VPALIGNR YMM4, YMM5, [RAX], 15
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| 0xFF - i).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm6_ymm7_mem_shift_0() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x45, 0x0f, 0x30, 0x00, // VPALIGNR YMM6, YMM7, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_ymm8_ymm9_mem_shift_16() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0x35, 0x0f, 0x00, 0x10, // VPALIGNR YMM8, YMM9, [RAX], 16
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpalignr_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x04, // VPALIGNR YMM0, YMM1, YMM2, 4
        0xc4, 0xe3, 0x7d, 0x0f, 0xc3, 0x08, // VPALIGNR YMM0, YMM0, YMM3, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_same_register() {
    let mut emu = emu64();
    // VPALIGNR with same source registers
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc1, 0x08, // VPALIGNR YMM0, YMM1, YMM1, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x03, // VPALIGNR YMM0, YMM1, YMM2, 3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x05, // VPALIGNR YMM0, YMM1, YMM2, 5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x06, // VPALIGNR YMM0, YMM1, YMM2, 6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x09, // VPALIGNR YMM0, YMM1, YMM2, 9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x0a, // VPALIGNR YMM0, YMM1, YMM2, 10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x0b, // VPALIGNR YMM0, YMM1, YMM2, 11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x0d, // VPALIGNR YMM0, YMM1, YMM2, 13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_shift_14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x0e, // VPALIGNR YMM0, YMM1, YMM2, 14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_sequential_shift_2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x02, // VPALIGNR YMM0, YMM1, [RAX], 2
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_sequential_shift_4() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x04, // VPALIGNR YMM0, YMM1, [RAX], 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_pattern_shift_3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x03, // VPALIGNR YMM0, YMM1, [RAX], 3
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0xAA, 0x55, 0xFF, 0x00]
        .into_iter()
        .cycle()
        .take(32)
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_alternating_shift_7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x07, // VPALIGNR YMM0, YMM1, [RAX], 7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| if i % 2 == 0 { 0xF0 } else { 0x0F }).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_extended_regs_shift_5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x15, 0x0f, 0xef, 0x05, // VPALIGNR YMM13, YMM13, YMM15, 5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_boundary_shift_17() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x11, // VPALIGNR YMM0, YMM1, YMM2, 17
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_boundary_shift_255() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0xff, // VPALIGNR YMM0, YMM1, YMM2, 255
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_all_zeros_shift_8() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x08, // VPALIGNR YMM0, YMM1, [RAX], 8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_all_ones_shift_12() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x0c, // VPALIGNR YMM0, YMM1, [RAX], 12
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_extract_middle_bytes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x0f, 0xc2, 0x08, // VPALIGNR YMM0, YMM1, YMM2, 8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpalignr_mem_powers_of_two_shift_4() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x0f, 0x00, 0x04, // VPALIGNR YMM0, YMM1, [RAX], 4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| 1u8 << (i % 8)).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
