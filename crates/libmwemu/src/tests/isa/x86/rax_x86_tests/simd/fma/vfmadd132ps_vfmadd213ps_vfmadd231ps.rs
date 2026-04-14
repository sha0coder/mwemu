use crate::*;

// VFMADD132PS - Fused Multiply-Add of Packed Single-Precision Floating-Point Values (dest = dest * src2 + src1)
// VFMADD213PS - Fused Multiply-Add of Packed Single-Precision Floating-Point Values (dest = src1 * dest + src2)
// VFMADD231PS - Fused Multiply-Add of Packed Single-Precision Floating-Point Values (dest = src1 * src2 + dest)
//
// These instructions perform fused multiply-add operations on packed single-precision floating-point values.
// The three variants differ in operand ordering for the multiply-add operation.
//
// Opcodes:
// VEX.128.66.0F38.W0 98 /r    VFMADD132PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 98 /r    VFMADD132PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W0 A8 /r    VFMADD213PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 A8 /r    VFMADD213PS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W0 B8 /r    VFMADD231PS xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W0 B8 /r    VFMADD231PS ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFMADD132PS Tests - 128-bit (dest = dest * src2 + src1)
// ============================================================================

#[test]
fn test_vfmadd132ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD132PS XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0x98, 0xc2, // VFMADD132PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD132PS XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0x98, 0xcb, // VFMADD132PS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD132PS XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0x98, 0xd4, // VFMADD132PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD132PS XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x59, 0x98, 0xdd, // VFMADD132PS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VFMADD132PS XMM4, XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0x51, 0x98, 0xe6, // VFMADD132PS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VFMADD132PS XMM5, XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0x49, 0x98, 0xef, // VFMADD132PS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VFMADD132PS XMM6, XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0x41, 0x98, 0xf0, // VFMADD132PS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VFMADD132PS XMM7, XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0x98, 0xf9, // VFMADD132PS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD132PS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x31, 0x98, 0xc2, // VFMADD132PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VFMADD132PS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x42, 0x29, 0x98, 0xcb, // VFMADD132PS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VFMADD132PS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x42, 0x21, 0x98, 0xd4, // VFMADD132PS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VFMADD132PS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x42, 0x19, 0x98, 0xdd, // VFMADD132PS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VFMADD132PS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x42, 0x11, 0x98, 0xe6, // VFMADD132PS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD132PS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x09, 0x98, 0xef, // VFMADD132PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VFMADD132PS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x42, 0x01, 0x98, 0xf0, // VFMADD132PS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VFMADD132PS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x42, 0x39, 0x98, 0xf9, // VFMADD132PS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD132PS Tests - 256-bit (dest = dest * src2 + src1)
// ============================================================================

#[test]
fn test_vfmadd132ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VFMADD132PS YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0x98, 0xc2, // VFMADD132PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VFMADD132PS YMM1, YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0x6d, 0x98, 0xcb, // VFMADD132PS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VFMADD132PS YMM2, YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0x65, 0x98, 0xd4, // VFMADD132PS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VFMADD132PS YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0x5d, 0x98, 0xdd, // VFMADD132PS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VFMADD132PS YMM4, YMM5, YMM6
    let code = [
        0xc4, 0xe2, 0x55, 0x98, 0xe6, // VFMADD132PS YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VFMADD132PS YMM5, YMM6, YMM7
    let code = [
        0xc4, 0xe2, 0x4d, 0x98, 0xef, // VFMADD132PS YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VFMADD132PS YMM6, YMM7, YMM0
    let code = [
        0xc4, 0xe2, 0x45, 0x98, 0xf0, // VFMADD132PS YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VFMADD132PS YMM7, YMM0, YMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x98, 0xf9, // VFMADD132PS YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VFMADD132PS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x42, 0x35, 0x98, 0xc2, // VFMADD132PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VFMADD132PS YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x42, 0x2d, 0x98, 0xcb, // VFMADD132PS YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VFMADD132PS YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x42, 0x25, 0x98, 0xd4, // VFMADD132PS YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VFMADD132PS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x42, 0x1d, 0x98, 0xdd, // VFMADD132PS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VFMADD132PS YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x42, 0x15, 0x98, 0xe6, // VFMADD132PS YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VFMADD132PS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x42, 0x0d, 0x98, 0xef, // VFMADD132PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VFMADD132PS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x42, 0x05, 0x98, 0xf0, // VFMADD132PS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ps_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VFMADD132PS YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x42, 0x3d, 0x98, 0xf9, // VFMADD132PS YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD213PS Tests - 128-bit (dest = src1 * dest + src2)
// ============================================================================

#[test]
fn test_vfmadd213ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD213PS XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0xa8, 0xc2, // VFMADD213PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD213PS XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0xa8, 0xcb, // VFMADD213PS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD213PS XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0xa8, 0xd4, // VFMADD213PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD213PS XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x59, 0xa8, 0xdd, // VFMADD213PS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD213PS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x31, 0xa8, 0xc2, // VFMADD213PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD213PS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x09, 0xa8, 0xef, // VFMADD213PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD213PS Tests - 256-bit (dest = src1 * dest + src2)
// ============================================================================

#[test]
fn test_vfmadd213ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VFMADD213PS YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0xa8, 0xc2, // VFMADD213PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VFMADD213PS YMM1, YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0x6d, 0xa8, 0xcb, // VFMADD213PS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VFMADD213PS YMM2, YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0x65, 0xa8, 0xd4, // VFMADD213PS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VFMADD213PS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x42, 0x35, 0xa8, 0xc2, // VFMADD213PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VFMADD213PS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x42, 0x0d, 0xa8, 0xef, // VFMADD213PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD231PS Tests - 128-bit (dest = src1 * src2 + dest)
// ============================================================================

#[test]
fn test_vfmadd231ps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD231PS XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0xb8, 0xc2, // VFMADD231PS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD231PS XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0xb8, 0xcb, // VFMADD231PS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD231PS XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0xb8, 0xd4, // VFMADD231PS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD231PS XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x59, 0xb8, 0xdd, // VFMADD231PS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD231PS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x31, 0xb8, 0xc2, // VFMADD231PS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD231PS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x09, 0xb8, 0xef, // VFMADD231PS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD231PS Tests - 256-bit (dest = src1 * src2 + dest)
// ============================================================================

#[test]
fn test_vfmadd231ps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VFMADD231PS YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0xb8, 0xc2, // VFMADD231PS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VFMADD231PS YMM1, YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0x6d, 0xb8, 0xcb, // VFMADD231PS YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VFMADD231PS YMM2, YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0x65, 0xb8, 0xd4, // VFMADD231PS YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VFMADD231PS YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x42, 0x35, 0xb8, 0xc2, // VFMADD231PS YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VFMADD231PS YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x42, 0x0d, 0xb8, 0xef, // VFMADD231PS YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfmadd132ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD132PS XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0x98, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD132PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ps_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VFMADD213PS YMM0, YMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x75, 0xa8, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD213PS YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ps_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD231PS XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0xb8, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD231PS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
