use crate::*;

// ROUNDPS - Round Packed Single Precision Floating-Point Values
// ROUNDPD - Round Packed Double Precision Floating-Point Values
//
// ROUNDPS rounds 4 packed single-precision (32-bit) floating-point values
// ROUNDPD rounds 2 packed double-precision (64-bit) floating-point values
//
// Opcodes:
// 66 0F 3A 08 /r ib    ROUNDPS xmm1, xmm2/m128, imm8 - Round packed single from xmm2/m128 to xmm1 using imm8 mode
// 66 0F 3A 09 /r ib    ROUNDPD xmm1, xmm2/m128, imm8 - Round packed double from xmm2/m128 to xmm1 using imm8 mode
//
// Rounding modes (imm8 bits[1:0]):
//   00b - Round to nearest (even)
//   01b - Round down (toward -infinity)
//   10b - Round up (toward +infinity)
//   11b - Round toward zero (truncate)
// Bit 2: 0 = use imm8[1:0], 1 = use MXCSR.RC
// Bit 3: 0 = raise precision exception, 1 = suppress precision exception

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// ROUNDPS Tests - Packed Single Precision (4x float32)
// ============================================================================

// Round to nearest (even) tests - mode 0x00
#[test]
fn test_roundps_xmm0_xmm1_nearest() {
    let mut emu = emu64();
    // ROUNDPS XMM0, XMM1, 0x00 (round to nearest even)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x00, // ROUNDPS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_nearest() {
    let mut emu = emu64();
    // ROUNDPS XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x00, // ROUNDPS XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_nearest() {
    let mut emu = emu64();
    // ROUNDPS XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x00, // ROUNDPS XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_nearest() {
    let mut emu = emu64();
    // ROUNDPS XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x00, // ROUNDPS XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Round down (toward -infinity) tests - mode 0x01
#[test]
fn test_roundps_xmm0_xmm1_down() {
    let mut emu = emu64();
    // ROUNDPS XMM0, XMM1, 0x01 (round down/floor)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x01, // ROUNDPS XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_down() {
    let mut emu = emu64();
    // ROUNDPS XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x01, // ROUNDPS XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_down() {
    let mut emu = emu64();
    // ROUNDPS XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x01, // ROUNDPS XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_down() {
    let mut emu = emu64();
    // ROUNDPS XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x01, // ROUNDPS XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Round up (toward +infinity) tests - mode 0x02
#[test]
fn test_roundps_xmm0_xmm1_up() {
    let mut emu = emu64();
    // ROUNDPS XMM0, XMM1, 0x02 (round up/ceil)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x02, // ROUNDPS XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_up() {
    let mut emu = emu64();
    // ROUNDPS XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x02, // ROUNDPS XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_up() {
    let mut emu = emu64();
    // ROUNDPS XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x02, // ROUNDPS XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_up() {
    let mut emu = emu64();
    // ROUNDPS XMM6, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x02, // ROUNDPS XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Round toward zero (truncate) tests - mode 0x03
#[test]
fn test_roundps_xmm0_xmm1_trunc() {
    let mut emu = emu64();
    // ROUNDPS XMM0, XMM1, 0x03 (round toward zero/truncate)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x03, // ROUNDPS XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_trunc() {
    let mut emu = emu64();
    // ROUNDPS XMM2, XMM3, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x03, // ROUNDPS XMM2, XMM3, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_trunc() {
    let mut emu = emu64();
    // ROUNDPS XMM4, XMM5, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x03, // ROUNDPS XMM4, XMM5, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_trunc() {
    let mut emu = emu64();
    // ROUNDPS XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x03, // ROUNDPS XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests with precision exception suppression (bit 3 set) - mode 0x08
#[test]
fn test_roundps_xmm0_xmm1_nearest_suppress() {
    let mut emu = emu64();
    // ROUNDPS XMM0, XMM1, 0x08 (suppress precision exception)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xc1, 0x08, // ROUNDPS XMM0, XMM1, 0x08
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm2_xmm3_down_suppress() {
    let mut emu = emu64();
    // ROUNDPS XMM2, XMM3, 0x09
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xd3, 0x09, // ROUNDPS XMM2, XMM3, 0x09
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm4_xmm5_up_suppress() {
    let mut emu = emu64();
    // ROUNDPS XMM4, XMM5, 0x0A
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xe5, 0x0a, // ROUNDPS XMM4, XMM5, 0x0A
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm6_xmm7_trunc_suppress() {
    let mut emu = emu64();
    // ROUNDPS XMM6, XMM7, 0x0B
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xf7, 0x0b, // ROUNDPS XMM6, XMM7, 0x0B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests with extended registers (XMM8-XMM15)
#[test]
fn test_roundps_xmm8_xmm9_nearest() {
    let mut emu = emu64();
    // ROUNDPS XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xc1, 0x00, // ROUNDPS XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm10_xmm11_down() {
    let mut emu = emu64();
    // ROUNDPS XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xd3, 0x01, // ROUNDPS XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm12_xmm13_up() {
    let mut emu = emu64();
    // ROUNDPS XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xe5, 0x02, // ROUNDPS XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm14_xmm15_trunc() {
    let mut emu = emu64();
    // ROUNDPS XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x08, 0xf7, 0x03, // ROUNDPS XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory operand tests
#[test]
fn test_roundps_xmm0_mem_nearest() {
    let mut emu = emu64();
    // ROUNDPS XMM0, [ALIGNED_ADDR], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, // ROUNDPS XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm1_mem_down() {
    let mut emu = emu64();
    // ROUNDPS XMM1, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, // ROUNDPS XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm7_mem_up() {
    let mut emu = emu64();
    // ROUNDPS XMM7, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, // ROUNDPS XMM7, [0x3000], 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundps_xmm15_mem_trunc() {
    let mut emu = emu64();
    // ROUNDPS XMM15, [ALIGNED_ADDR], 0x03
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x08, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, // ROUNDPS XMM15, [0x3000], 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// ROUNDPD Tests - Packed Double Precision (2x float64)
// ============================================================================

// Round to nearest (even) tests - mode 0x00
#[test]
fn test_roundpd_xmm0_xmm1_nearest() {
    let mut emu = emu64();
    // ROUNDPD XMM0, XMM1, 0x00 (round to nearest even)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x00, // ROUNDPD XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_nearest() {
    let mut emu = emu64();
    // ROUNDPD XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x00, // ROUNDPD XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_nearest() {
    let mut emu = emu64();
    // ROUNDPD XMM4, XMM5, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x00, // ROUNDPD XMM4, XMM5, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_nearest() {
    let mut emu = emu64();
    // ROUNDPD XMM6, XMM7, 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x00, // ROUNDPD XMM6, XMM7, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Round down (toward -infinity) tests - mode 0x01
#[test]
fn test_roundpd_xmm0_xmm1_down() {
    let mut emu = emu64();
    // ROUNDPD XMM0, XMM1, 0x01 (round down/floor)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x01, // ROUNDPD XMM0, XMM1, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_down() {
    let mut emu = emu64();
    // ROUNDPD XMM2, XMM3, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x01, // ROUNDPD XMM2, XMM3, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_down() {
    let mut emu = emu64();
    // ROUNDPD XMM4, XMM5, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x01, // ROUNDPD XMM4, XMM5, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_down() {
    let mut emu = emu64();
    // ROUNDPD XMM6, XMM7, 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x01, // ROUNDPD XMM6, XMM7, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Round up (toward +infinity) tests - mode 0x02
#[test]
fn test_roundpd_xmm0_xmm1_up() {
    let mut emu = emu64();
    // ROUNDPD XMM0, XMM1, 0x02 (round up/ceil)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x02, // ROUNDPD XMM0, XMM1, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_up() {
    let mut emu = emu64();
    // ROUNDPD XMM2, XMM3, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x02, // ROUNDPD XMM2, XMM3, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_up() {
    let mut emu = emu64();
    // ROUNDPD XMM4, XMM5, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x02, // ROUNDPD XMM4, XMM5, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_up() {
    let mut emu = emu64();
    // ROUNDPD XMM6, XMM7, 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x02, // ROUNDPD XMM6, XMM7, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Round toward zero (truncate) tests - mode 0x03
#[test]
fn test_roundpd_xmm0_xmm1_trunc() {
    let mut emu = emu64();
    // ROUNDPD XMM0, XMM1, 0x03 (round toward zero/truncate)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x03, // ROUNDPD XMM0, XMM1, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_trunc() {
    let mut emu = emu64();
    // ROUNDPD XMM2, XMM3, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x03, // ROUNDPD XMM2, XMM3, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_trunc() {
    let mut emu = emu64();
    // ROUNDPD XMM4, XMM5, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x03, // ROUNDPD XMM4, XMM5, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_trunc() {
    let mut emu = emu64();
    // ROUNDPD XMM6, XMM7, 0x03
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x03, // ROUNDPD XMM6, XMM7, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests with precision exception suppression (bit 3 set) - mode 0x08
#[test]
fn test_roundpd_xmm0_xmm1_nearest_suppress() {
    let mut emu = emu64();
    // ROUNDPD XMM0, XMM1, 0x08 (suppress precision exception)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xc1, 0x08, // ROUNDPD XMM0, XMM1, 0x08
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm2_xmm3_down_suppress() {
    let mut emu = emu64();
    // ROUNDPD XMM2, XMM3, 0x09
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xd3, 0x09, // ROUNDPD XMM2, XMM3, 0x09
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm4_xmm5_up_suppress() {
    let mut emu = emu64();
    // ROUNDPD XMM4, XMM5, 0x0A
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xe5, 0x0a, // ROUNDPD XMM4, XMM5, 0x0A
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm6_xmm7_trunc_suppress() {
    let mut emu = emu64();
    // ROUNDPD XMM6, XMM7, 0x0B
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xf7, 0x0b, // ROUNDPD XMM6, XMM7, 0x0B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Tests with extended registers (XMM8-XMM15)
#[test]
fn test_roundpd_xmm8_xmm9_nearest() {
    let mut emu = emu64();
    // ROUNDPD XMM8, XMM9, 0x00 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xc1, 0x00, // ROUNDPD XMM8, XMM9, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm10_xmm11_down() {
    let mut emu = emu64();
    // ROUNDPD XMM10, XMM11, 0x01
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xd3, 0x01, // ROUNDPD XMM10, XMM11, 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm12_xmm13_up() {
    let mut emu = emu64();
    // ROUNDPD XMM12, XMM13, 0x02
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xe5, 0x02, // ROUNDPD XMM12, XMM13, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm14_xmm15_trunc() {
    let mut emu = emu64();
    // ROUNDPD XMM14, XMM15, 0x03
    let code = [
        0x66, 0x45, 0x0f, 0x3a, 0x09, 0xf7, 0x03, // ROUNDPD XMM14, XMM15, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Memory operand tests
#[test]
fn test_roundpd_xmm0_mem_nearest() {
    let mut emu = emu64();
    // ROUNDPD XMM0, [ALIGNED_ADDR], 0x00
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0x00, // ROUNDPD XMM0, [0x3000], 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm1_mem_down() {
    let mut emu = emu64();
    // ROUNDPD XMM1, [ALIGNED_ADDR], 0x01
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0x0c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x01, // ROUNDPD XMM1, [0x3000], 0x01
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm7_mem_up() {
    let mut emu = emu64();
    // ROUNDPD XMM7, [ALIGNED_ADDR], 0x02
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x02, // ROUNDPD XMM7, [0x3000], 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm15_mem_trunc() {
    let mut emu = emu64();
    // ROUNDPD XMM15, [ALIGNED_ADDR], 0x03
    let code = [
        0x66, 0x44, 0x0f, 0x3a, 0x09, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, 0x03, // ROUNDPD XMM15, [0x3000], 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// Additional mode combinations
#[test]
fn test_roundps_xmm1_xmm2_mode_0x04() {
    let mut emu = emu64();
    // ROUNDPS XMM1, XMM2, 0x04 (use MXCSR.RC)
    let code = [
        0x66, 0x0f, 0x3a, 0x08, 0xca, 0x04, // ROUNDPS XMM1, XMM2, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_roundpd_xmm1_xmm2_mode_0x04() {
    let mut emu = emu64();
    // ROUNDPD XMM1, XMM2, 0x04 (use MXCSR.RC)
    let code = [
        0x66, 0x0f, 0x3a, 0x09, 0xca, 0x04, // ROUNDPD XMM1, XMM2, 0x04
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
