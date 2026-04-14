use crate::*;

// CVTPS2PD - Convert Packed Single Precision Floating-Point Values to Packed Double Precision
//
// Converts two packed single precision floating-point values in the source operand to
// two packed double precision floating-point values in the destination operand.
// This instruction increases precision from 32-bit to 64-bit floating-point format.
//
// Opcode:
// NP 0F 5A /r    CVTPS2PD xmm1, xmm2/m64
//
// The conversion extends the precision, preserving the value exactly for most numbers.
// Special values (NaN, Infinity, denormals) are handled according to IEEE 754 rules.

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// Basic Register to Register Tests
// ============================================================================

#[test]
fn test_cvtps2pd_xmm0_to_xmm1() {
    let mut emu = emu64();
    // CVTPS2PD XMM1, XMM0
    let code = [
        0x0f, 0x5a, 0xc8, // CVTPS2PD XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm2_to_xmm3() {
    let mut emu = emu64();
    // CVTPS2PD XMM3, XMM2
    let code = [
        0x0f, 0x5a, 0xda, // CVTPS2PD XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm4_to_xmm5() {
    let mut emu = emu64();
    // CVTPS2PD XMM5, XMM4
    let code = [
        0x0f, 0x5a, 0xec, // CVTPS2PD XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm6_to_xmm7() {
    let mut emu = emu64();
    // CVTPS2PD XMM7, XMM6
    let code = [
        0x0f, 0x5a, 0xfe, // CVTPS2PD XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm8_to_xmm9() {
    let mut emu = emu64();
    // CVTPS2PD XMM9, XMM8
    let code = [
        0x45, 0x0f, 0x5a, 0xc8, // CVTPS2PD XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm10_to_xmm11() {
    let mut emu = emu64();
    // CVTPS2PD XMM11, XMM10
    let code = [
        0x45, 0x0f, 0x5a, 0xda, // CVTPS2PD XMM11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm14_to_xmm15() {
    let mut emu = emu64();
    // CVTPS2PD XMM15, XMM14
    let code = [
        0x45, 0x0f, 0x5a, 0xfe, // CVTPS2PD XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm0_to_xmm15() {
    let mut emu = emu64();
    // CVTPS2PD XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x5a, 0xf8, // CVTPS2PD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_xmm15_to_xmm0() {
    let mut emu = emu64();
    // CVTPS2PD XMM0, XMM15
    let code = [
        0x41, 0x0f, 0x5a, 0xc7, // CVTPS2PD XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_cvtps2pd_mem_to_xmm0() {
    let mut emu = emu64();
    // CVTPS2PD XMM0, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 1.0;
    let f2: f32 = 2.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_mem_to_xmm1() {
    let mut emu = emu64();
    // CVTPS2PD XMM1, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x08, // CVTPS2PD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 3.5;
    let f2: f32 = -4.25;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_mem_to_xmm7() {
    let mut emu = emu64();
    // CVTPS2PD XMM7, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x3b, // CVTPS2PD XMM7, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 0.5;
    let f2: f32 = 100.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_mem_to_xmm8() {
    let mut emu = emu64();
    // CVTPS2PD XMM8, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x5a, 0x01, // CVTPS2PD XMM8, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = -1.5;
    let f2: f32 = 256.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_mem_to_xmm15() {
    let mut emu = emu64();
    // CVTPS2PD XMM15, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x44, 0x0f, 0x5a, 0x3a, // CVTPS2PD XMM15, [RDX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 0.125;
    let f2: f32 = -0.0625;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Special Float Values - Positive and Negative Zero
// ============================================================================

#[test]
fn test_cvtps2pd_positive_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 0.0;
    let f2: f32 = 0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = -0.0;
    let f2: f32 = -0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_mixed_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 0.0;
    let f2: f32 = -0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Special Float Values - Infinity
// ============================================================================

#[test]
fn test_cvtps2pd_positive_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::INFINITY;
    let f2: f32 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_negative_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::NEG_INFINITY;
    let f2: f32 = -1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_both_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::INFINITY;
    let f2: f32 = f32::NEG_INFINITY;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Special Float Values - NaN
// ============================================================================

#[test]
fn test_cvtps2pd_quiet_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::NAN;
    let f2: f32 = 0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_signaling_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let snan: u32 = 0x7F800001; // Signaling NaN
    let normal: u32 = 0x3F800000; // 1.0f

    let mut data = Vec::new();
    data.extend_from_slice(&snan.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_both_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::NAN;
    let f2: f32 = f32::NAN;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Special Float Values - Denormals
// ============================================================================

#[test]
fn test_cvtps2pd_denormal_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm: u32 = 0x00000001; // Smallest positive denormal
    let normal: u32 = 0x3F800000; // 1.0f

    let mut data = Vec::new();
    data.extend_from_slice(&denorm.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_denormal_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm: u32 = 0x80000001; // Smallest negative denormal
    let normal: u32 = 0xBF800000; // -1.0f

    let mut data = Vec::new();
    data.extend_from_slice(&denorm.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_denormal_largest() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm: u32 = 0x007FFFFF;
    let normal: u32 = 0x3F800000; // 1.0f

    let mut data = Vec::new();
    data.extend_from_slice(&denorm.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Normal Float Value Tests
// ============================================================================

#[test]
fn test_cvtps2pd_small_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 1.0e-10;
    let f2: f32 = -1.0e-10;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_large_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 1.0e30;
    let f2: f32 = -1.0e30;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_fractional_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 0.333333;
    let f2: f32 = 0.666666;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_powers_of_two() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 128.0;
    let f2: f32 = 0.0625; // 1/16
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 42.5;
    let f2: f32 = -17.25;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_cvtps2pd_with_displacement() {
    let mut emu = emu64();
    // CVTPS2PD XMM0, [RAX + 16]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x40, 0x10, // CVTPS2PD XMM0, [RAX + 16]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 7.5;
    let f2: f32 = 8.25;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_with_negative_displacement() {
    let mut emu = emu64();
    // CVTPS2PD XMM1, [RBX - 8]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR + 8).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x4b, 0xf8, // CVTPS2PD XMM1, [RBX - 8]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = -99.99;
    let f2: f32 = 88.88;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_rip_relative() {
    let mut emu = emu64();
    // CVTPS2PD XMM0, [RIP + disp]
    let code = [
        0x0f, 0x5a, 0x05, 0x00, 0x00, 0x00, 0x00, // CVTPS2PD XMM0, [RIP + 0]
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Multiple Conversion Tests
// ============================================================================

#[test]
fn test_cvtps2pd_multiple_conversions() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0x0f, 0x5a, 0x08, // CVTPS2PD XMM1, [RAX]
        0x0f, 0x5a, 0x10, // CVTPS2PD XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 3.14159;
    let f2: f32 = 2.71828;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_chain_registers() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x5a, 0xc8, // CVTPS2PD XMM1, XMM0
        0x0f, 0x5a, 0xd1, // CVTPS2PD XMM2, XMM1 (XMM1[63:0] are now f64, this tests upper bits)
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_cvtps2pd_max_float32() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::MAX;
    let f2: f32 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_min_positive_float32() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = f32::MIN_POSITIVE;
    let f2: f32 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtps2pd_one_and_minus_one() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x5a, 0x00, // CVTPS2PD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f32 = 1.0;
    let f2: f32 = -1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}
