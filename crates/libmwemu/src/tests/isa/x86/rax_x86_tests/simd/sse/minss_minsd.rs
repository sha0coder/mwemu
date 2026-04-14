use crate::*;

// MINSS - Return Minimum Scalar Single Precision Floating-Point Value
// MINSD - Return Minimum Scalar Double Precision Floating-Point Value
//
// MINSS compares the low single-precision (32-bit) floating-point values
// and returns the minimum to the destination.
//
// MINSD compares the low double-precision (64-bit) floating-point values
// and returns the minimum to the destination.
//
// Special cases:
// - If values are both 0.0s (either sign), return second operand
// - If second operand is SNaN, forward SNaN unchanged to destination
// - If only one value is NaN, return second operand
//
// Opcodes:
// F3 0F 5D /r             MINSS xmm1, xmm2/m32   - Return minimum scalar single
// F2 0F 5D /r             MINSD xmm1, xmm2/m64   - Return minimum scalar double

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

fn float_bits(val: f32) -> [u8; 4] {
    val.to_le_bytes()
}

fn double_bits(val: f64) -> [u8; 8] {
    val.to_le_bytes()
}

// ============================================================================
// MINSS Tests - Scalar Single Precision Minimum
// ============================================================================

#[test]
fn test_minss_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minss_first_smaller() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_second_smaller() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_equal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5d, 0xc0, // MINSS XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(4.5));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_negative_values() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-5.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_positive_and_negative() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_both_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5d, 0xc0, // MINSS XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_positive_and_negative_zero() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_infinity() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_negative_infinity() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::NEG_INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_very_large_numbers() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e30));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1e29));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_very_small_numbers() {
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
        0xf3, 0x0f, 0x5d, 0xc1, // MINSS XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e-10));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1e-20));
    emu.run(None).unwrap();
}

#[test]
fn test_minss_denormal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x10, 0x00, // MOVSS XMM0, [RAX]
        0xf3, 0x0f, 0x5d, 0xc0, // MINSS XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let denormal = f32::from_bits(1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(denormal));
    emu.run(None).unwrap();
}

// ============================================================================
// MINSD Tests - Scalar Double Precision Minimum
// ============================================================================

#[test]
fn test_minsd_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x28, 0xc1, // MOVAPS XMM0, XMM1
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_first_smaller() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_second_smaller() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_equal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(4.5));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_negative_values() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(-1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(-5.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_positive_and_negative() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(-3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_both_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_positive_and_negative_zero() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(-0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_infinity() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_negative_infinity() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::NEG_INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_very_large_numbers() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1e100));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(1e99));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_very_small_numbers() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(1e-100));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(1e-200));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_high_precision_pi() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(3.14159265358979));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_denormal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf2, 0x0f, 0x10, 0x00, // MOVSD XMM0, [RAX]
        0xf2, 0x0f, 0x5d, 0xc0, // MINSD XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let denormal = f64::from_bits(1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(denormal));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_both_infinities() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(f64::INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_minsd_mixed_infinity_values() {
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
        0xf2, 0x0f, 0x5d, 0xc1, // MINSD XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &double_bits(f64::INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &double_bits(f64::NEG_INFINITY));
    emu.run(None).unwrap();
}
