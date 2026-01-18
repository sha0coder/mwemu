use crate::*;

// VFMADD132SS - Fused Multiply-Add of Scalar Single-Precision Floating-Point Values (dest[31:0] = dest[31:0] * src2[31:0] + src1[31:0])
// VFMADD213SS - Fused Multiply-Add of Scalar Single-Precision Floating-Point Values (dest[31:0] = src1[31:0] * dest[31:0] + src2[31:0])
// VFMADD231SS - Fused Multiply-Add of Scalar Single-Precision Floating-Point Values (dest[31:0] = src1[31:0] * src2[31:0] + dest[31:0])
//
// These instructions perform fused multiply-add operations on the low single-precision floating-point value.
// The upper bits of the destination are copied from src1.
//
// Opcodes:
// VEX.LIG.66.0F38.W0 99 /r    VFMADD132SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W0 A9 /r    VFMADD213SS xmm1, xmm2, xmm3/m32
// VEX.LIG.66.0F38.W0 B9 /r    VFMADD231SS xmm1, xmm2, xmm3/m32

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFMADD132SS Tests (dest[31:0] = dest[31:0] * src2[31:0] + src1[31:0])
// ============================================================================

#[test]
fn test_vfmadd132ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD132SS XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0x99, 0xc2, // VFMADD132SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD132SS XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0x99, 0xcb, // VFMADD132SS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD132SS XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0x99, 0xd4, // VFMADD132SS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD132SS XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x59, 0x99, 0xdd, // VFMADD132SS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VFMADD132SS XMM4, XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0x51, 0x99, 0xe6, // VFMADD132SS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VFMADD132SS XMM5, XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0x49, 0x99, 0xef, // VFMADD132SS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VFMADD132SS XMM6, XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0x41, 0x99, 0xf0, // VFMADD132SS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VFMADD132SS XMM7, XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0x99, 0xf9, // VFMADD132SS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD132SS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x31, 0x99, 0xc2, // VFMADD132SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VFMADD132SS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x42, 0x29, 0x99, 0xcb, // VFMADD132SS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VFMADD132SS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x42, 0x21, 0x99, 0xd4, // VFMADD132SS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    // VFMADD132SS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0x42, 0x19, 0x99, 0xdd, // VFMADD132SS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    // VFMADD132SS XMM12, XMM13, XMM14
    let code = [
        0xc4, 0x42, 0x11, 0x99, 0xe6, // VFMADD132SS XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD132SS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x09, 0x99, 0xef, // VFMADD132SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    // VFMADD132SS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0x42, 0x01, 0x99, 0xf0, // VFMADD132SS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    // VFMADD132SS XMM15, XMM8, XMM9
    let code = [
        0xc4, 0x42, 0x39, 0x99, 0xf9, // VFMADD132SS XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD213SS Tests (dest[31:0] = src1[31:0] * dest[31:0] + src2[31:0])
// ============================================================================

#[test]
fn test_vfmadd213ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD213SS XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0xa9, 0xc2, // VFMADD213SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD213SS XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0xa9, 0xcb, // VFMADD213SS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD213SS XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0xa9, 0xd4, // VFMADD213SS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD213SS XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x59, 0xa9, 0xdd, // VFMADD213SS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VFMADD213SS XMM4, XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0x51, 0xa9, 0xe6, // VFMADD213SS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VFMADD213SS XMM5, XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0x49, 0xa9, 0xef, // VFMADD213SS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VFMADD213SS XMM6, XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0x41, 0xa9, 0xf0, // VFMADD213SS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VFMADD213SS XMM7, XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0xa9, 0xf9, // VFMADD213SS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD213SS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x31, 0xa9, 0xc2, // VFMADD213SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VFMADD213SS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x42, 0x29, 0xa9, 0xcb, // VFMADD213SS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VFMADD213SS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x42, 0x21, 0xa9, 0xd4, // VFMADD213SS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD213SS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x09, 0xa9, 0xef, // VFMADD213SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD231SS Tests (dest[31:0] = src1[31:0] * src2[31:0] + dest[31:0])
// ============================================================================

#[test]
fn test_vfmadd231ss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    // VFMADD231SS XMM0, XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0xb9, 0xc2, // VFMADD231SS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    // VFMADD231SS XMM1, XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0xb9, 0xcb, // VFMADD231SS XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    // VFMADD231SS XMM2, XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0xb9, 0xd4, // VFMADD231SS XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    // VFMADD231SS XMM3, XMM4, XMM5
    let code = [
        0xc4, 0xe2, 0x59, 0xb9, 0xdd, // VFMADD231SS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    // VFMADD231SS XMM4, XMM5, XMM6
    let code = [
        0xc4, 0xe2, 0x51, 0xb9, 0xe6, // VFMADD231SS XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    // VFMADD231SS XMM5, XMM6, XMM7
    let code = [
        0xc4, 0xe2, 0x49, 0xb9, 0xef, // VFMADD231SS XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    // VFMADD231SS XMM6, XMM7, XMM0
    let code = [
        0xc4, 0xe2, 0x41, 0xb9, 0xf0, // VFMADD231SS XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    // VFMADD231SS XMM7, XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0xb9, 0xf9, // VFMADD231SS XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    // VFMADD231SS XMM8, XMM9, XMM10
    let code = [
        0xc4, 0x42, 0x31, 0xb9, 0xc2, // VFMADD231SS XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    // VFMADD231SS XMM9, XMM10, XMM11
    let code = [
        0xc4, 0x42, 0x29, 0xb9, 0xcb, // VFMADD231SS XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    // VFMADD231SS XMM10, XMM11, XMM12
    let code = [
        0xc4, 0x42, 0x21, 0xb9, 0xd4, // VFMADD231SS XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    // VFMADD231SS XMM13, XMM14, XMM15
    let code = [
        0xc4, 0x42, 0x09, 0xb9, 0xef, // VFMADD231SS XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfmadd132ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD132SS XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0x99, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD132SS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD213SS XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0xa9, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD213SS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VFMADD231SS XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0xb9, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD231SS XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132ss_xmm15_xmm8_mem() {
    let mut emu = emu64();
    // VFMADD132SS XMM15, XMM8, [mem]
    let code = [
        0xc4, 0x62, 0x39, 0x99, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD132SS XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213ss_xmm15_xmm8_mem() {
    let mut emu = emu64();
    // VFMADD213SS XMM15, XMM8, [mem]
    let code = [
        0xc4, 0x62, 0x39, 0xa9, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD213SS XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231ss_xmm15_xmm8_mem() {
    let mut emu = emu64();
    // VFMADD231SS XMM15, XMM8, [mem]
    let code = [
        0xc4, 0x62, 0x39, 0xb9, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD231SS XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
