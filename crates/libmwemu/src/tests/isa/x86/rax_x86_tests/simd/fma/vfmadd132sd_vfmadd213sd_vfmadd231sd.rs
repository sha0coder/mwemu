use crate::*;

// VFMADD132SD - Fused Multiply-Add of Scalar Double-Precision Floating-Point Values (dest[63:0] = dest[63:0] * src2[63:0] + src1[63:0])
// VFMADD213SD - Fused Multiply-Add of Scalar Double-Precision Floating-Point Values (dest[63:0] = src1[63:0] * dest[63:0] + src2[63:0])
// VFMADD231SD - Fused Multiply-Add of Scalar Double-Precision Floating-Point Values (dest[63:0] = src1[63:0] * src2[63:0] + dest[63:0])
//
// These instructions perform fused multiply-add operations on the low double-precision floating-point value.
// The upper bits of the destination are copied from src1.
//
// Opcodes:
// VEX.LIG.66.0F38.W1 99 /r    VFMADD132SD xmm1, xmm2, xmm3/m64
// VEX.LIG.66.0F38.W1 A9 /r    VFMADD213SD xmm1, xmm2, xmm3/m64
// VEX.LIG.66.0F38.W1 B9 /r    VFMADD231SD xmm1, xmm2, xmm3/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VFMADD132SD Tests (dest[63:0] = dest[63:0] * src2[63:0] + src1[63:0])
// ============================================================================

#[test]
fn test_vfmadd132sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x99, 0xc2, // VFMADD132SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe9, 0x99, 0xcb, // VFMADD132SD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0x99, 0xd4, // VFMADD132SD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xd9, 0x99, 0xdd, // VFMADD132SD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xd1, 0x99, 0xe6, // VFMADD132SD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc9, 0x99, 0xef, // VFMADD132SD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc1, 0x99, 0xf0, // VFMADD132SD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf9, 0x99, 0xf9, // VFMADD132SD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0x99, 0xc2, // VFMADD132SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xa9, 0x99, 0xcb, // VFMADD132SD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xa1, 0x99, 0xd4, // VFMADD132SD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x99, 0x99, 0xdd, // VFMADD132SD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x91, 0x99, 0xe6, // VFMADD132SD XMM12, XMM13, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0x99, 0xef, // VFMADD132SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm14_xmm15_xmm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x81, 0x99, 0xf0, // VFMADD132SD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm15_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb9, 0x99, 0xf9, // VFMADD132SD XMM15, XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD213SD Tests (dest[63:0] = src1[63:0] * dest[63:0] + src2[63:0])
// ============================================================================

#[test]
fn test_vfmadd213sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xa9, 0xc2, // VFMADD213SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe9, 0xa9, 0xcb, // VFMADD213SD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0xa9, 0xd4, // VFMADD213SD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xd9, 0xa9, 0xdd, // VFMADD213SD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xd1, 0xa9, 0xe6, // VFMADD213SD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc9, 0xa9, 0xef, // VFMADD213SD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc1, 0xa9, 0xf0, // VFMADD213SD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf9, 0xa9, 0xf9, // VFMADD213SD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xa9, 0xc2, // VFMADD213SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xa9, 0xa9, 0xcb, // VFMADD213SD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xa1, 0xa9, 0xd4, // VFMADD213SD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0xa9, 0xef, // VFMADD213SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VFMADD231SD Tests (dest[63:0] = src1[63:0] * src2[63:0] + dest[63:0])
// ============================================================================

#[test]
fn test_vfmadd231sd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xb9, 0xc2, // VFMADD231SD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe9, 0xb9, 0xcb, // VFMADD231SD XMM1, XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xe1, 0xb9, 0xd4, // VFMADD231SD XMM2, XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xd9, 0xb9, 0xdd, // VFMADD231SD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xd1, 0xb9, 0xe6, // VFMADD231SD XMM4, XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc9, 0xb9, 0xef, // VFMADD231SD XMM5, XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm6_xmm7_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc1, 0xb9, 0xf0, // VFMADD231SD XMM6, XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm7_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf9, 0xb9, 0xf9, // VFMADD231SD XMM7, XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xb1, 0xb9, 0xc2, // VFMADD231SD XMM8, XMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xa9, 0xb9, 0xcb, // VFMADD231SD XMM9, XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xa1, 0xb9, 0xd4, // VFMADD231SD XMM10, XMM11, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x89, 0xb9, 0xef, // VFMADD231SD XMM13, XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_vfmadd132sd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0x99, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD132SD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xa9, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD213SD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm0_xmm1_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf1, 0xb9, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD231SD XMM0, XMM1, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd132sd_xmm15_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xb9, 0x99, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD132SD XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd213sd_xmm15_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xb9, 0xa9, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD213SD XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vfmadd231sd_xmm15_xmm8_mem() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xb9, 0xb9, 0x3c, 0x25, 0x00, 0x30, 0x00, 0x00, // VFMADD231SD XMM15, XMM8, [0x3000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
