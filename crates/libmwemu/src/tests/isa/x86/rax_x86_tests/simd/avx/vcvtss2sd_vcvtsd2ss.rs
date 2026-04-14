use crate::*;

// VCVTSS2SD - Convert Scalar Single-Precision to Scalar Double-Precision Floating-Point Value
// VCVTSD2SS - Convert Scalar Double-Precision to Scalar Single-Precision Floating-Point Value
//
// VCVTSS2SD converts a scalar single-precision floating-point value to a scalar double-precision
// floating-point value. The conversion is exact.
// VCVTSD2SS converts a scalar double-precision floating-point value to a scalar single-precision
// floating-point value. Rounding is controlled by MXCSR.RC.
//
// Opcodes:
// VEX.LIG.F3.0F.WIG 5A /r VCVTSS2SD xmm1, xmm2, xmm3/m32 - Convert scalar single to double
// VEX.LIG.F2.0F.WIG 5A /r VCVTSD2SS xmm1, xmm2, xmm3/m64 - Convert scalar double to single

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VCVTSS2SD Tests - Convert scalar single to double
// ============================================================================

#[test]
fn test_vcvtss2sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTSS2SD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf2, 0x5a, 0xc2, // VCVTSS2SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTSS2SD XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xea, 0x5a, 0xcb, // VCVTSS2SD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTSS2SD XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe2, 0x5a, 0xd4, // VCVTSS2SD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTSS2SD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xda, 0x5a, 0xdd, // VCVTSS2SD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTSS2SD XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd2, 0x5a, 0xe6, // VCVTSS2SD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTSS2SD XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xca, 0x5a, 0xef, // VCVTSS2SD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTSS2SD XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc2, 0x5a, 0xf0, // VCVTSS2SD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTSS2SD XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xfa, 0x5a, 0xf9, // VCVTSS2SD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SD Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtss2sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTSS2SD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x32, 0x5a, 0xc2, // VCVTSS2SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTSS2SD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x2a, 0x5a, 0xcb, // VCVTSS2SD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTSS2SD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x22, 0x5a, 0xd4, // VCVTSS2SD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTSS2SD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x1a, 0x5a, 0xdd, // VCVTSS2SD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTSS2SD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x12, 0x5a, 0xe6, // VCVTSS2SD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTSS2SD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x0a, 0x5a, 0xef, // VCVTSS2SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTSS2SD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x02, 0x5a, 0xf0, // VCVTSS2SD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTSS2SD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x3a, 0x5a, 0xf9, // VCVTSS2SD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SD Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vcvtss2sd_xmm0_xmm8_xmm15() {
    let mut emu = emu64();
    // VCVTSS2SD XMM0, XMM8, XMM15
    let code = [
        0xc4, 0xc1, 0x3a, 0x5a, 0xc7, // VCVTSS2SD XMM0, XMM8, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm8_xmm0_xmm7() {
    let mut emu = emu64();
    // VCVTSS2SD XMM8, XMM0, XMM7
    let code = [
        0xc4, 0xc1, 0x7a, 0x5a, 0xc7, // VCVTSS2SD XMM8, XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm7_xmm8_xmm0() {
    let mut emu = emu64();
    // VCVTSS2SD XMM7, XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x3a, 0x5a, 0xf8, // VCVTSS2SD XMM7, XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSS2SD Tests - Memory operands
// ============================================================================

#[test]
fn test_vcvtss2sd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VCVTSS2SD XMM0, XMM1, [mem] (reads 32 bits)
    let code = [
        0xc5, 0xf2, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SD XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VCVTSS2SD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x32, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SD XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x00, 0x40, // 2.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm2_xmm3_mem_denormal() {
    let mut emu = emu64();
    // VCVTSS2SD XMM2, XMM3, [mem] - Test with denormal value
    let code = [
        0xc5, 0xe2, 0x5a, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SD XMM2, XMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x01, 0x00, 0x00, 0x00, // Very small denormal float
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm4_xmm5_mem_inf() {
    let mut emu = emu64();
    // VCVTSS2SD XMM4, XMM5, [mem] - Test with infinity
    let code = [
        0xc5, 0xd2, 0x5a, 0x25, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SD XMM4, XMM5, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0x80, 0x7f, // +Infinity
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtss2sd_xmm6_xmm7_mem_nan() {
    let mut emu = emu64();
    // VCVTSS2SD XMM6, XMM7, [mem] - Test with NaN
    let code = [
        0xc5, 0xc2, 0x5a, 0x35, 0x00, 0x40, 0x00, 0x00, // VCVTSS2SD XMM6, XMM7, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [
        0x00, 0x00, 0xc0, 0x7f, // Quiet NaN
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SS Tests - Convert scalar double to single
// ============================================================================

#[test]
fn test_vcvtsd2ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VCVTSD2SS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x5a, 0xc2, // VCVTSD2SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VCVTSD2SS XMM1, XMM2, XMM3
    let code = [
        0xc5, 0xeb, 0x5a, 0xcb, // VCVTSD2SS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VCVTSD2SS XMM2, XMM3, XMM4
    let code = [
        0xc5, 0xe3, 0x5a, 0xd4, // VCVTSD2SS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VCVTSD2SS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xdb, 0x5a, 0xdd, // VCVTSD2SS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VCVTSD2SS XMM4, XMM5, XMM6
    let code = [
        0xc5, 0xd3, 0x5a, 0xe6, // VCVTSD2SS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VCVTSD2SS XMM5, XMM6, XMM7
    let code = [
        0xc5, 0xcb, 0x5a, 0xef, // VCVTSD2SS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VCVTSD2SS XMM6, XMM7, XMM0
    let code = [
        0xc5, 0xc3, 0x5a, 0xf0, // VCVTSD2SS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VCVTSD2SS XMM7, XMM0, XMM1
    let code = [
        0xc5, 0xfb, 0x5a, 0xf9, // VCVTSD2SS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SS Tests - Extended XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_vcvtsd2ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VCVTSD2SS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x41, 0x33, 0x5a, 0xc2, // VCVTSD2SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VCVTSD2SS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x41, 0x2b, 0x5a, 0xcb, // VCVTSD2SS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VCVTSD2SS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x41, 0x23, 0x5a, 0xd4, // VCVTSD2SS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VCVTSD2SS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x41, 0x1b, 0x5a, 0xdd, // VCVTSD2SS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VCVTSD2SS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x41, 0x13, 0x5a, 0xe6, // VCVTSD2SS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VCVTSD2SS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x41, 0x0b, 0x5a, 0xef, // VCVTSD2SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VCVTSD2SS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x41, 0x03, 0x5a, 0xf0, // VCVTSD2SS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VCVTSD2SS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x41, 0x3b, 0x5a, 0xf9, // VCVTSD2SS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SS Tests - Cross-domain XMM registers
// ============================================================================

#[test]
fn test_vcvtsd2ss_xmm0_xmm8_xmm15() {
    let mut emu = emu64();
    // VCVTSD2SS XMM0, XMM8, XMM15
    let code = [
        0xc4, 0xc1, 0x3b, 0x5a, 0xc7, // VCVTSD2SS XMM0, XMM8, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm8_xmm0_xmm7() {
    let mut emu = emu64();
    // VCVTSD2SS XMM8, XMM0, XMM7
    let code = [
        0xc4, 0xc1, 0x7b, 0x5a, 0xc7, // VCVTSD2SS XMM8, XMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm7_xmm8_xmm0() {
    let mut emu = emu64();
    // VCVTSD2SS XMM7, XMM8, XMM0
    let code = [
        0xc4, 0xc1, 0x3b, 0x5a, 0xf8, // VCVTSD2SS XMM7, XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VCVTSD2SS Tests - Memory operands
// ============================================================================

#[test]
fn test_vcvtsd2ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VCVTSD2SS XMM0, XMM1, [mem] (reads 64 bits)
    let code = [
        0xc5, 0xf3, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SS XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VCVTSD2SS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x41, 0x33, 0x5a, 0x05, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm2_xmm3_mem_large() {
    let mut emu = emu64();
    // VCVTSD2SS XMM2, XMM3, [mem] - Test with large value
    let code = [
        0xc5, 0xe3, 0x5a, 0x15, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SS XMM2, XMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x8f, 0x40, // 1000.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm4_xmm5_mem_small() {
    let mut emu = emu64();
    // VCVTSD2SS XMM4, XMM5, [mem] - Test with small value
    let code = [
        0xc5, 0xd3, 0x5a, 0x25, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SS XMM4, XMM5, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x97, 0xff, 0x57, 0x14, 0xae, 0x8a, 0xa4, 0x3e, // 1.2345e-8
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm6_xmm7_mem_inf() {
    let mut emu = emu64();
    // VCVTSD2SS XMM6, XMM7, [mem] - Test with infinity
    let code = [
        0xc5, 0xc3, 0x5a, 0x35, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SS XMM6, XMM7, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x7f, // +Infinity
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vcvtsd2ss_xmm7_xmm0_mem_nan() {
    let mut emu = emu64();
    // VCVTSD2SS XMM7, XMM0, [mem] - Test with NaN
    let code = [
        0xc5, 0xfb, 0x5a, 0x3d, 0x00, 0x40, 0x00, 0x00, // VCVTSD2SS XMM7, XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x7f, // Quiet NaN
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
