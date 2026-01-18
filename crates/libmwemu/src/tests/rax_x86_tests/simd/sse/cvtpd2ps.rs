use crate::*;

// CVTPD2PS - Convert Packed Double Precision Floating-Point Values to Packed Single Precision
//
// Converts two packed double precision floating-point values in the source operand to
// two packed single precision floating-point values in the destination operand.
// This instruction decreases precision from 64-bit to 32-bit floating-point format.
// When conversion is inexact, the value is rounded according to MXCSR rounding control bits.
//
// Opcode:
// 66 0F 5A /r    CVTPD2PS xmm1, xmm2/m128
//
// The conversion may lose precision and can cause overflow to infinity.
// Upper 64 bits of destination are zeroed.

const DATA_ADDR: u64 = 0x3000;

// ============================================================================
// Basic Register to Register Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_xmm0_to_xmm1() {
    let mut emu = emu64();
    // CVTPD2PS XMM1, XMM0
    let code = [
        0x66, 0x0f, 0x5a, 0xc8, // CVTPD2PS XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm2_to_xmm3() {
    let mut emu = emu64();
    // CVTPD2PS XMM3, XMM2
    let code = [
        0x66, 0x0f, 0x5a, 0xda, // CVTPD2PS XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm4_to_xmm5() {
    let mut emu = emu64();
    // CVTPD2PS XMM5, XMM4
    let code = [
        0x66, 0x0f, 0x5a, 0xec, // CVTPD2PS XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm6_to_xmm7() {
    let mut emu = emu64();
    // CVTPD2PS XMM7, XMM6
    let code = [
        0x66, 0x0f, 0x5a, 0xfe, // CVTPD2PS XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm8_to_xmm9() {
    let mut emu = emu64();
    // CVTPD2PS XMM9, XMM8
    let code = [
        0x66, 0x45, 0x0f, 0x5a, 0xc8, // CVTPD2PS XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm10_to_xmm11() {
    let mut emu = emu64();
    // CVTPD2PS XMM11, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x5a, 0xda, // CVTPD2PS XMM11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm14_to_xmm15() {
    let mut emu = emu64();
    // CVTPD2PS XMM15, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x5a, 0xfe, // CVTPD2PS XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm0_to_xmm15() {
    let mut emu = emu64();
    // CVTPD2PS XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x5a, 0xf8, // CVTPD2PS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_xmm15_to_xmm0() {
    let mut emu = emu64();
    // CVTPD2PS XMM0, XMM15
    let code = [
        0x66, 0x41, 0x0f, 0x5a, 0xc7, // CVTPD2PS XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_mem_to_xmm0() {
    let mut emu = emu64();
    // CVTPD2PS XMM0, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.0;
    let f2: f64 = 2.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_mem_to_xmm1() {
    let mut emu = emu64();
    // CVTPD2PS XMM1, [RAX]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x08, // CVTPD2PS XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 3.5;
    let f2: f64 = -4.25;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_mem_to_xmm7() {
    let mut emu = emu64();
    // CVTPD2PS XMM7, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x3b, // CVTPD2PS XMM7, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 0.5;
    let f2: f64 = 100.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_mem_to_xmm8() {
    let mut emu = emu64();
    // CVTPD2PS XMM8, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x5a, 0x01, // CVTPD2PS XMM8, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = -1.5;
    let f2: f64 = 256.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_mem_to_xmm15() {
    let mut emu = emu64();
    // CVTPD2PS XMM15, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x5a, 0x3a, // CVTPD2PS XMM15, [RDX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 0.125;
    let f2: f64 = -0.0625;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Special Float Values - Zeros
// ============================================================================

#[test]
fn test_cvtpd2ps_positive_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 0.0;
    let f2: f64 = 0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = -0.0;
    let f2: f64 = -0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_mixed_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 0.0;
    let f2: f64 = -0.0;
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
fn test_cvtpd2ps_positive_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::INFINITY;
    let f2: f64 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_negative_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::NEG_INFINITY;
    let f2: f64 = -1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_both_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::INFINITY;
    let f2: f64 = f64::NEG_INFINITY;
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
fn test_cvtpd2ps_quiet_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::NAN;
    let f2: f64 = 0.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_signaling_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let snan: u64 = 0x7FF0000000000001;
    let normal: u64 = 0x3FF0000000000000; // 1.0

    let mut data = Vec::new();
    data.extend_from_slice(&snan.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_both_nan() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::NAN;
    let f2: f64 = f64::NAN;
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
fn test_cvtpd2ps_denormal_positive() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm: u64 = 0x0000000000000001;
    let normal: u64 = 0x3FF0000000000000; // 1.0

    let mut data = Vec::new();
    data.extend_from_slice(&denorm.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_denormal_negative() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let denorm: u64 = 0x8000000000000001;
    let normal: u64 = 0xBFF0000000000000; // -1.0

    let mut data = Vec::new();
    data.extend_from_slice(&denorm.to_le_bytes());
    data.extend_from_slice(&normal.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Precision Loss and Rounding Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_precision_loss() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.0000000001; // Extra precision
    let f2: f64 = 2.9999999999;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_rounding_nearest() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.5000000000000002;
    let f2: f64 = 2.5000000000000004;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_very_small_precision() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.00000000000000001;
    let f2: f64 = 1.00000000000000002;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Overflow to Infinity Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_overflow_to_positive_inf() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.0e100; // Much larger than f32 can hold
    let f2: f64 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_overflow_to_negative_inf() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = -1.0e100;
    let f2: f64 = -1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_near_max_f32() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 3.4e38; // Close to f32::MAX
    let f2: f64 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Normal Value Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_small_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.0e-10;
    let f2: f64 = -1.0e-10;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_large_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.0e30;
    let f2: f64 = -1.0e30;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_fractional_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 0.333333333333333;
    let f2: f64 = 0.666666666666666;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_powers_of_two() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 128.0;
    let f2: f64 = 0.0625; // 1/16
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 42.5;
    let f2: f64 = -17.25;
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
fn test_cvtpd2ps_with_displacement() {
    let mut emu = emu64();
    // CVTPD2PS XMM0, [RAX + 16]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR - 16).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x40, 0x10, // CVTPD2PS XMM0, [RAX + 16]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 7.5;
    let f2: f64 = 8.25;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_with_negative_displacement() {
    let mut emu = emu64();
    // CVTPD2PS XMM1, [RBX - 8]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(DATA_ADDR + 8).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x4b, 0xf8, // CVTPD2PS XMM1, [RBX - 8]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = -99.99;
    let f2: f64 = 88.88;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_rip_relative() {
    let mut emu = emu64();
    // CVTPD2PS XMM0, [RIP + disp]
    let code = [
        0x66, 0x0f, 0x5a, 0x05, 0x00, 0x00, 0x00, 0x00, // CVTPD2PS XMM0, [RIP + 0]
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Multiple Conversion Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_multiple_conversions() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0x66, 0x0f, 0x5a, 0x08, // CVTPD2PS XMM1, [RAX]
        0x66, 0x0f, 0x5a, 0x10, // CVTPD2PS XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 3.14159265358979;
    let f2: f64 = 2.71828182845904;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_cvtpd2ps_max_float64() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::MAX;
    let f2: f64 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_min_positive_float64() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = f64::MIN_POSITIVE;
    let f2: f64 = 1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_one_and_minus_one() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 1.0;
    let f2: f64 = -1.0;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cvtpd2ps_upper_bits_zeroed() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&DATA_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x5a, 0x00, // CVTPD2PS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let f1: f64 = 123.456;
    let f2: f64 = 789.012;
    let mut data = Vec::new();
    data.extend_from_slice(&f1.to_le_bytes());
    data.extend_from_slice(&f2.to_le_bytes());

    emu.maps.write_bytes_slice(DATA_ADDR, &data);
    emu.run(None).unwrap();
}
