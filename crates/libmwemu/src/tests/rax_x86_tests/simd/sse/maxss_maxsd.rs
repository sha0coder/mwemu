use crate::*;

// MAXSS - Return Maximum Scalar Single Precision Floating-Point Value
// MAXSD - Return Maximum Scalar Double Precision Floating-Point Value
//
// MAXSS compares the low single-precision (32-bit) floating-point values
// and returns the maximum to the destination.
//
// MAXSD compares the low double-precision (64-bit) floating-point values
// and returns the maximum to the destination.
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// F3 0F 5F /r             MAXSS xmm1, xmm2/m32   - Return maximum scalar single
// F2 0F 5F /r             MAXSD xmm1, xmm2/m64   - Return maximum scalar double

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

fn float_bits(val: f32) -> [u8; 4] {
    val.to_le_bytes()
}

fn double_bits(val: f64) -> [u8; 8] {
    val.to_le_bytes()
}

// ============================================================================
// MAXSS Tests - Scalar Single Precision Maximum
// ============================================================================

#[test]
fn test_maxss_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_first_larger() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_second_larger() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_equal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5f, 0xc0, // MAXSS XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(4.5));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_negative_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-5.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_positive_and_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_both_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5f, 0xc0, // MAXSS XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_positive_and_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_negative_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::NEG_INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_very_large_numbers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e30));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1e29));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_very_small_numbers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e-20));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1e-10));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_denormal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5f, 0xc0, // MAXSS XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let denormal = f32::from_bits(1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(denormal));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_both_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(f32::INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxss_fractional_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x10, 0x0b, // MOVSS XMM1, [RBX]
        0xf3, 0x0f, 0x5f, 0xc1, // MAXSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.333333));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(0.5));
    emu.run(None).unwrap();
}

// ============================================================================
// MAXSD Tests - Scalar Double Precision Maximum
// ============================================================================

#[test]
fn test_maxsd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_first_larger() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_second_larger() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_equal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(4.5));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_negative_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(-1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(-5.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_positive_and_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(-5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_both_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_positive_and_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(-0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_negative_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::NEG_INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_very_large_numbers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1e100));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(1e99));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_very_small_numbers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1e-200));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(1e-100));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_high_precision_e() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(2.71828182845904));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_denormal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5f, 0xc0, // MAXSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let denormal = f64::from_bits(1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(denormal));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_both_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(f64::INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_mixed_infinity_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(f64::INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::NEG_INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_maxsd_fractional_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x10, 0x0b, // MOVSD XMM1, [RBX]
        0xf2, 0x0f, 0x5f, 0xc1, // MAXSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(0.3333333333));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(0.5));
    emu.run(None).unwrap();
}
