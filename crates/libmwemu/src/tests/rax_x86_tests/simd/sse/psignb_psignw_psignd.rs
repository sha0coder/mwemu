use crate::*;

// PSIGNB/PSIGNW/PSIGND - Packed SIGN
//
// Negates, zeroes, or preserves each data element of the destination operand
// based on the sign of the corresponding data element in the source operand:
//   - If source element < 0: negate destination element
//   - If source element == 0: set destination element to 0
//   - If source element > 0: preserve destination element
//
// PSIGNB: operates on bytes (16 elements)
// PSIGNW: operates on words (8 elements)
// PSIGND: operates on dwords (4 elements)
//
// Opcodes:
//   66 0F 38 08 /r    PSIGNB xmm1, xmm2/m128
//   66 0F 38 09 /r    PSIGNW xmm1, xmm2/m128
//   66 0F 38 0A /r    PSIGND xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// PSIGNB Tests (Packed Sign Bytes)
// ============================================================================

#[test]
fn test_psignb_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xc1, // PSIGNB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm2_xmm3_all_positive_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xd3, // PSIGNB XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm4_xmm5_all_negative_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xe5, // PSIGNB XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm6_xmm7_all_zero_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xf7, // PSIGNB XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm0_xmm1_mixed_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xc1, // PSIGNB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm1_xmm2_negate_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xca, // PSIGNB XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm3_xmm4_preserve_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xdc, // PSIGNB XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm5_xmm6_zero_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xee, // PSIGNB XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xf8, // PSIGNB XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x08, 0xc1, // PSIGNB XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x08, 0xd3, // PSIGNB XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x08, 0xe5, // PSIGNB XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x08, 0xf7, // PSIGNB XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x08, 0x00, // PSIGNB XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x01, 0xFF, 0x00, 0x7F, 0x80, 0x02, 0xFE, 0x00,
                           0x03, 0xFD, 0x00, 0x04, 0xFC, 0x05, 0xFB, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PSIGNW Tests (Packed Sign Words)
// ============================================================================

#[test]
fn test_psignw_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xc1, // PSIGNW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm2_xmm3_all_positive_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xd3, // PSIGNW XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm4_xmm5_all_negative_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xe5, // PSIGNW XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm6_xmm7_all_zero_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xf7, // PSIGNW XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm0_xmm1_mixed_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xc1, // PSIGNW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm1_xmm2_negate_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xca, // PSIGNW XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm3_xmm4_preserve_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xdc, // PSIGNW XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm5_xmm6_zero_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xee, // PSIGNW XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xf8, // PSIGNW XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x09, 0xc1, // PSIGNW XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x09, 0xd3, // PSIGNW XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x09, 0xe5, // PSIGNW XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x09, 0xf7, // PSIGNW XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x09, 0x00, // PSIGNW XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x80,
                           0xFF, 0x7F, 0x01, 0x00, 0x00, 0x00, 0xFE, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PSIGND Tests (Packed Sign Dwords)
// ============================================================================

#[test]
fn test_psignd_xmm0_xmm1_basic() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xc1, // PSIGND XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm2_xmm3_all_positive_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xd3, // PSIGND XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm4_xmm5_all_negative_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xe5, // PSIGND XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm6_xmm7_all_zero_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xf7, // PSIGND XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm0_xmm1_mixed_control() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xc1, // PSIGND XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm1_xmm2_negate_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xca, // PSIGND XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm3_xmm4_preserve_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xdc, // PSIGND XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm5_xmm6_zero_pattern() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xee, // PSIGND XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xf8, // PSIGND XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0a, 0xc1, // PSIGND XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0a, 0xd3, // PSIGND XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0a, 0xe5, // PSIGND XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0x66, 0x45, 0x0f, 0x38, 0x0a, 0xf7, // PSIGND XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_xmm0_mem() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x0a, 0x00, // PSIGND XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: [u8; 16] = [0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                           0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional edge case tests
// ============================================================================

#[test]
fn test_psignb_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xc0, // PSIGNB XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xc9, // PSIGNW XMM1, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_same_register() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xd2, // PSIGND XMM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psign_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xc1, // PSIGNB XMM0, XMM1
        0x66, 0x0f, 0x38, 0x09, 0xd3, // PSIGNW XMM2, XMM3
        0x66, 0x0f, 0x38, 0x0a, 0xe5, // PSIGND XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignb_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x08, 0xc1, // PSIGNB XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignw_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x09, 0xc1, // PSIGNW XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_psignd_alternating_signs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x38, 0x0a, 0xc1, // PSIGND XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
