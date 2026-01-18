use crate::*;

// PHADDSW - Packed Horizontal Add and Saturate
// PHSUBSW - Packed Horizontal Subtract and Saturate
//
// PHADDSW adds two adjacent signed 16-bit integers horizontally from the source
// and destination operands and saturates the signed results; packs the signed,
// saturated 16-bit results to the destination operand.
//
// PHSUBSW performs horizontal subtraction on each adjacent pair of 16-bit signed
// integers by subtracting the most significant word from the least significant
// word of each pair in the source and destination operands. The signed, saturated
// 16-bit results are packed to the destination operand.
//
// Saturation: INT16_MIN = -32768, INT16_MAX = 32767
//
// Opcodes:
//   66 0F 38 03 /r    PHADDSW xmm1, xmm2/m128
//   66 0F 38 07 /r    PHSUBSW xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// PHADDSW Tests

#[test]
fn test_phaddsw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm2_xmm3_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xd3, // PHADDSW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xe5, // PHADDSW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm6_xmm7_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xf7, // PHADDSW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm0_xmm1_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm1_xmm2_saturate_max() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xca, // PHADDSW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm3_xmm4_saturate_min() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xdc, // PHADDSW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x03, 0xd3, // PHADDSW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x03, 0xe5, // PHADDSW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x03, 0x00, // PHADDSW XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_xmm1_mem_saturate() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x03, 0x08, // PHADDSW XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F,
                           0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc0, // PHADDSW XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_horizontal_layout() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_max_no_saturate() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// PHSUBSW Tests

#[test]
fn test_phsubsw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm2_xmm3_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xd3, // PHSUBSW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xe5, // PHSUBSW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm6_xmm7_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xf7, // PHSUBSW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm0_xmm1_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm1_xmm2_saturate_max() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xca, // PHSUBSW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm3_xmm4_saturate_min() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xdc, // PHSUBSW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x07, 0xd3, // PHSUBSW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x07, 0xe5, // PHSUBSW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x07, 0x00, // PHSUBSW XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [10, 0, 5, 0, 20, 0, 15, 0, 30, 0, 25, 0, 40, 0, 35, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_xmm1_mem_saturate() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x07, 0x08, // PHSUBSW XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F,
                           0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc0, // PHSUBSW XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_horizontal_layout() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_max_no_saturate() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Mixed PHADDSW and PHSUBSW Tests

#[test]
fn test_phaddsw_phsubsw_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x07, 0xd3, // PHSUBSW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_phsubsw_alternating() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_boundary_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_boundary_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_int16_max() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_int16_min() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_int16_max() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_int16_min() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x03, 0x40, 0x10, // PHADDSW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x07, 0x40, 0x10, // PHSUBSW XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_all_words_different() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_all_words_different() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_incremental_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phsubsw_incremental_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x07, 0xc1, // PHSUBSW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_phaddsw_phsubsw_combined_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x03, 0xc1, // PHADDSW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x07, 0xd3, // PHSUBSW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x03, 0xe5, // PHADDSW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
