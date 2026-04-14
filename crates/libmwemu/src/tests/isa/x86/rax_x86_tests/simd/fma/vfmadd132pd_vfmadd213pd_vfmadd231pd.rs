use crate::*;

// VFMADD132PD - Fused Multiply-Add of Packed Double-Precision Floating-Point Values (dest = dest * src2 + src1)
// VFMADD213PD - Fused Multiply-Add of Packed Double-Precision Floating-Point Values (dest = src1 * dest + src2)
// VFMADD231PD - Fused Multiply-Add of Packed Double-Precision Floating-Point Values (dest = src1 * src2 + dest)
//
// These instructions perform fused multiply-add operations on packed double-precision floating-point values.
// The three variants differ in operand ordering for the multiply-add operation.
//
// Opcodes:
// VEX.128.66.0F38.W1 98 /r    VFMADD132PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 98 /r    VFMADD132PD ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 A8 /r    VFMADD213PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 A8 /r    VFMADD213PD ymm1, ymm2, ymm3/m256
// VEX.128.66.0F38.W1 B8 /r    VFMADD231PD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F38.W1 B8 /r    VFMADD231PD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFMADD132PD Tests - 128-bit (dest = dest * src2 + src1)
// ============================================================================

#[test]
fn test_vfmadd132pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD132PD XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0xf1, 0x98, 0xc2, // VFMADD132PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD132PD XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0xe9, 0x98, 0xcb, // VFMADD132PD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD132PD XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0xe1, 0x98, 0xd4, // VFMADD132PD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD132PD XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0xd9, 0x98, 0xdd, // VFMADD132PD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VFMADD132PD XMM4, XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0xd1, 0x98, 0xe6, // VFMADD132PD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VFMADD132PD XMM5, XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0xc9, 0x98, 0xef, // VFMADD132PD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VFMADD132PD XMM6, XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0xc1, 0x98, 0xf0, // VFMADD132PD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VFMADD132PD XMM7, XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0xf9, 0x98, 0xf9, // VFMADD132PD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD132PD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0xb1, 0x98, 0xc2, // VFMADD132PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VFMADD132PD XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x42, 0xa9, 0x98, 0xcb, // VFMADD132PD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VFMADD132PD XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x42, 0xa1, 0x98, 0xd4, // VFMADD132PD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VFMADD132PD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x42, 0x99, 0x98, 0xdd, // VFMADD132PD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VFMADD132PD XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x42, 0x91, 0x98, 0xe6, // VFMADD132PD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD132PD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x89, 0x98, 0xef, // VFMADD132PD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VFMADD132PD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x42, 0x81, 0x98, 0xf0, // VFMADD132PD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VFMADD132PD XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x42, 0xb9, 0x98, 0xf9, // VFMADD132PD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD132PD Tests - 256-bit (dest = dest * src2 + src1)
// ============================================================================

#[test]
fn test_vfmadd132pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VFMADD132PD YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0xf5, 0x98, 0xc2, // VFMADD132PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VFMADD132PD YMM1, YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0xed, 0x98, 0xcb, // VFMADD132PD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VFMADD132PD YMM2, YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0xe5, 0x98, 0xd4, // VFMADD132PD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    // VFMADD132PD YMM3, YMM4, YMM5
    let code = [
        0xc4, 0xe2, 0xdd, 0x98, 0xdd, // VFMADD132PD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    // VFMADD132PD YMM4, YMM5, YMM6
    let code = [
        0xc4, 0xe2, 0xd5, 0x98, 0xe6, // VFMADD132PD YMM4, YMM5, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    // VFMADD132PD YMM5, YMM6, YMM7
    let code = [
        0xc4, 0xe2, 0xcd, 0x98, 0xef, // VFMADD132PD YMM5, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    // VFMADD132PD YMM6, YMM7, YMM0
    let code = [
        0xc4, 0xe2, 0xc5, 0x98, 0xf0, // VFMADD132PD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm7_ymm0_ymm1() {
    let mut emu = emu64();
    // VFMADD132PD YMM7, YMM0, YMM1
    let code = [
        0xc4, 0xe2, 0xfd, 0x98, 0xf9, // VFMADD132PD YMM7, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VFMADD132PD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x42, 0xb5, 0x98, 0xc2, // VFMADD132PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    // VFMADD132PD YMM9, YMM10, YMM11
    let code = [
        0xc4, 0x42, 0xad, 0x98, 0xcb, // VFMADD132PD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    // VFMADD132PD YMM10, YMM11, YMM12
    let code = [
        0xc4, 0x42, 0xa5, 0x98, 0xd4, // VFMADD132PD YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    // VFMADD132PD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0x42, 0x9d, 0x98, 0xdd, // VFMADD132PD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    // VFMADD132PD YMM12, YMM13, YMM14
    let code = [
        0xc4, 0x42, 0x95, 0x98, 0xe6, // VFMADD132PD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VFMADD132PD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x42, 0x8d, 0x98, 0xef, // VFMADD132PD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm14_ymm15_ymm8() {
    let mut emu = emu64();
    // VFMADD132PD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0x42, 0x85, 0x98, 0xf0, // VFMADD132PD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132pd_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    // VFMADD132PD YMM15, YMM8, YMM9
    let code = [
        0xc4, 0x42, 0xbd, 0x98, 0xf9, // VFMADD132PD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD213PD Tests - 128-bit (dest = src1 * dest + src2)
// ============================================================================

#[test]
fn test_vfmadd213pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD213PD XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0xf1, 0xa8, 0xc2, // VFMADD213PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD213PD XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0xe9, 0xa8, 0xcb, // VFMADD213PD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD213PD XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0xe1, 0xa8, 0xd4, // VFMADD213PD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD213PD XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0xd9, 0xa8, 0xdd, // VFMADD213PD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD213PD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0xb1, 0xa8, 0xc2, // VFMADD213PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD213PD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x89, 0xa8, 0xef, // VFMADD213PD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD213PD Tests - 256-bit (dest = src1 * dest + src2)
// ============================================================================

#[test]
fn test_vfmadd213pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VFMADD213PD YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0xf5, 0xa8, 0xc2, // VFMADD213PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VFMADD213PD YMM1, YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0xed, 0xa8, 0xcb, // VFMADD213PD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VFMADD213PD YMM2, YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0xe5, 0xa8, 0xd4, // VFMADD213PD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VFMADD213PD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x42, 0xb5, 0xa8, 0xc2, // VFMADD213PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VFMADD213PD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x42, 0x8d, 0xa8, 0xef, // VFMADD213PD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD231PD Tests - 128-bit (dest = src1 * src2 + dest)
// ============================================================================

#[test]
fn test_vfmadd231pd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD231PD XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0xf1, 0xb8, 0xc2, // VFMADD231PD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD231PD XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0xe9, 0xb8, 0xcb, // VFMADD231PD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD231PD XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0xe1, 0xb8, 0xd4, // VFMADD231PD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD231PD XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0xd9, 0xb8, 0xdd, // VFMADD231PD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD231PD XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0xb1, 0xb8, 0xc2, // VFMADD231PD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD231PD XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x89, 0xb8, 0xef, // VFMADD231PD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD231PD Tests - 256-bit (dest = src1 * src2 + dest)
// ============================================================================

#[test]
fn test_vfmadd231pd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VFMADD231PD YMM0, YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0xf5, 0xb8, 0xc2, // VFMADD231PD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    // VFMADD231PD YMM1, YMM2, YMM3
    let code = [
        0xc4, 0xe2, 0xed, 0xb8, 0xcb, // VFMADD231PD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    // VFMADD231PD YMM2, YMM3, YMM4
    let code = [
        0xc4, 0xe2, 0xe5, 0xb8, 0xd4, // VFMADD231PD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    // VFMADD231PD YMM8, YMM9, YMM10
    let code = [
        0xc4, 0x42, 0xb5, 0xb8, 0xc2, // VFMADD231PD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_ymm13_ymm14_ymm15() {
    let mut emu = emu64();
    // VFMADD231PD YMM13, YMM14, YMM15
    let code = [
        0xc4, 0x42, 0x8d, 0xb8, 0xef, // VFMADD231PD YMM13, YMM14, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfmadd132pd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD132PD XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0xf1, 0x98, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD132PD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213pd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VFMADD213PD YMM0, YMM1, [mem]
    let code = [
        0xc4, 0xe2, 0xf5, 0xa8, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD213PD YMM0, YMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231pd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD231PD XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0xf1, 0xb8, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD231PD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
