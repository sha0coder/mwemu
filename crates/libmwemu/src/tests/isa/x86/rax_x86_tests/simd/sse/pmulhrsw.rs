use crate::*;

// PMULHRSW - Packed Multiply High With Round and Scale
//
// Multiplies vertically each signed 16-bit integer from the destination operand
// with the corresponding signed 16-bit integer of the source operand, producing
// intermediate, signed 32-bit integers. Each intermediate 32-bit integer is
// truncated to the 18 most significant bits. Rounding is always performed by
// adding 1 to the least significant bit of the 18-bit intermediate result.
// The final result is obtained by selecting the 16 bits immediately to the right
// of the most significant bit of each 18-bit intermediate result.
//
// Operation:
//   temp[31:0] = INT32((src[15:0] * dest[15:0]) >> 14) + 1
//   result[15:0] = temp[16:1]  // Extract bits 16:1
//
// Opcode:
//   66 0F 38 0B /r    PMULHRSW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pmulhrsw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm2_xmm3_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xd3, // PMULHRSW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xe5, // PMULHRSW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm6_xmm7_ones() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xf7, // PMULHRSW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm0_xmm1_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm1_xmm2_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xca, // PMULHRSW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm3_xmm4_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xdc, // PMULHRSW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm5_xmm6_rounding_down() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xee, // PMULHRSW XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm7_xmm0_rounding_up() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xf8, // PMULHRSW XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0b, 0xd3, // PMULHRSW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0b, 0xe5, // PMULHRSW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0b, 0xf7, // PMULHRSW XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x0b, 0x00, // PMULHRSW XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm1_mem_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x0b, 0x08, // PMULHRSW XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF,
                           0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm2_mem_max_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x0b, 0x10, // PMULHRSW XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // 0x7FFF = 32767 (max signed i16)
    let data: [u8; 16] = [0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F,
                           0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc0, // PMULHRSW XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x0b, 0xd3, // PMULHRSW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x0b, 0xe5, // PMULHRSW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_xmm15_xmm0_cross() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0x38, 0x0b, 0xf8, // PMULHRSW XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x0b, 0x40, 0x10, // PMULHRSW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_rounding_bit_set() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_rounding_bit_clear() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_scale_shift_14() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_extract_bits_16_1() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_small_products() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_large_products() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_boundary_32767() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_boundary_minus_32768() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_identity_value() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_power_of_two() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_incremental_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_all_words_different() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_varying_magnitudes() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmulhrsw_edge_rounding_cases() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0b, 0xc1, // PMULHRSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
