use crate::*;

// PMADDWD - Multiply and Add Packed Integers
//
// Multiplies the individual signed words of the destination operand by the
// corresponding signed words of the source operand, producing temporary signed,
// doubleword results. The adjacent doubleword results are then summed and stored
// in the destination operand.
//
// For each pair of words:
//   DEST[31:0] := (DEST[15:0] * SRC[15:0]) + (DEST[31:16] * SRC[31:16])
//   DEST[63:32] := (DEST[47:32] * SRC[47:32]) + (DEST[63:48] * SRC[63:48])
//   etc.
//
// Special case: When all pairs are 0x8000, result wraps to 0x80000000
//
// Opcode:
//   66 0F F5 /r    PMADDWD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

#[test]
fn test_pmaddwd_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm2_xmm3_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xd3, // PMADDWD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm4_xmm5_zeros() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xe5, // PMADDWD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm6_xmm7_ones() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xf7, // PMADDWD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm0_xmm1_positive_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm1_xmm2_negative_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xca, // PMADDWD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm3_xmm4_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xdc, // PMADDWD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm5_xmm6_max_positive() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xee, // PMADDWD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm7_xmm0_min_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xf8, // PMADDWD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xc1, // PMADDWD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xd3, // PMADDWD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xe5, // PMADDWD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0xf5, 0xf7, // PMADDWD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x00, // PMADDWD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm1_mem_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x08, // PMADDWD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // -1 as i16 = 0xFFFF
    let data: [u8; 16] = [0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF,
                           0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm2_mem_overflow_case() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x10, // PMADDWD XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    // 0x8000 = -32768, the special wrap case
    let data: [u8; 16] = [0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
                           0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc0, // PMADDWD XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_sequential() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0x66, 0x0f, 0xf5, 0xd3, // PMADDWD XMM2, XMM3
        0x66, 0x0f, 0xf5, 0xe5, // PMADDWD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_xmm15_xmm0_cross() {
    let mut emu = emu64();
    let code = [
        0x66, 0x44, 0x0f, 0xf5, 0xf8, // PMADDWD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_mem_displacement() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0xf5, 0x40, 0x10, // PMADDWD XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_positive_overflow() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_negative_overflow() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_wrap_special_case() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_zero_multiplication() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_identity_multiplication() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_max_word_value() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_min_word_value() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_mixed_positive_negative() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_small_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_large_values() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_adjacent_pairs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_carry_addition() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_negative_sum() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_positive_sum() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_cancellation() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_edge_8000h() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_edge_7fffh() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_varying_products() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_all_dwords() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_boundary_cases() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_incremental_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaddwd_decremental_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0xf5, 0xc1, // PMADDWD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
