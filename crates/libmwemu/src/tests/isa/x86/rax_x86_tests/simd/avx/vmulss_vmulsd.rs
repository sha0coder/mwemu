use crate::*;

// VMULSS - Multiply Scalar Single-Precision Floating-Point Values
// VMULSD - Multiply Scalar Double-Precision Floating-Point Values
//
// Opcodes:
// VEX.LIG.F3.0F.WIG 59 /r    VMULSS xmm1, xmm2, xmm3/m32
// VEX.LIG.F2.0F.WIG 59 /r    VMULSD xmm1, xmm2, xmm3/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VMULSS Tests
// ============================================================================

#[test]
fn test_vmulss_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf2, 0x59, 0xc2, // VMULSS XMM0, XMM1, XMM2
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xea, 0x59, 0xcb, // VMULSS XMM1, XMM2, XMM3
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe2, 0x59, 0xd4, // VMULSS XMM2, XMM3, XMM4
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xda, 0x59, 0xdd, // VMULSS XMM3, XMM4, XMM5
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xd2, 0x59, 0xe6, // VMULSS XMM4, XMM5, XMM6
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xca, 0x59, 0xef, // VMULSS XMM5, XMM6, XMM7
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x42, 0x59, 0xf0, // VMULSS XMM6, XMM7, XMM8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x3a, 0x59, 0xf9, // VMULSS XMM7, XMM8, XMM9
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x32, 0x59, 0xc2, // VMULSS XMM8, XMM9, XMM10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2a, 0x59, 0xcb, // VMULSS XMM9, XMM10, XMM11
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x22, 0x59, 0xd4, // VMULSS XMM10, XMM11, XMM12
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1a, 0x59, 0xdd, // VMULSS XMM11, XMM12, XMM13
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x12, 0x59, 0xe6, // VMULSS XMM12, XMM13, XMM14
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x0a, 0x59, 0xef, // VMULSS XMM13, XMM14, XMM15
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x02, 0x59, 0xf0, // VMULSS XMM14, XMM15, XMM0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm15_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x7a, 0x59, 0xf9, // VMULSS XMM15, XMM0, XMM1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm0_xmm1_mem32() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf2, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULSS XMM0, XMM1, [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x80, 0x3f]; // 1.0f
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulss_xmm2_xmm3_mem32() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe2, 0x59, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULSS XMM2, XMM3, [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x40]; // 2.0f
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

// ============================================================================
// VMULSD Tests
// ============================================================================

#[test]
fn test_vmulsd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf3, 0x59, 0xc2, // VMULSD XMM0, XMM1, XMM2
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xeb, 0x59, 0xcb, // VMULSD XMM1, XMM2, XMM3
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe3, 0x59, 0xd4, // VMULSD XMM2, XMM3, XMM4
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xdb, 0x59, 0xdd, // VMULSD XMM3, XMM4, XMM5
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xd3, 0x59, 0xe6, // VMULSD XMM4, XMM5, XMM6
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xcb, 0x59, 0xef, // VMULSD XMM5, XMM6, XMM7
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x43, 0x59, 0xf0, // VMULSD XMM6, XMM7, XMM8
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc1, 0x3b, 0x59, 0xf9, // VMULSD XMM7, XMM8, XMM9
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm8_xmm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x33, 0x59, 0xc2, // VMULSD XMM8, XMM9, XMM10
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm9_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x2b, 0x59, 0xcb, // VMULSD XMM9, XMM10, XMM11
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm10_xmm11_xmm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x23, 0x59, 0xd4, // VMULSD XMM10, XMM11, XMM12
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm11_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x1b, 0x59, 0xdd, // VMULSD XMM11, XMM12, XMM13
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm12_xmm13_xmm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x13, 0x59, 0xe6, // VMULSD XMM12, XMM13, XMM14
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm13_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x41, 0x0b, 0x59, 0xef, // VMULSD XMM13, XMM14, XMM15
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm14_xmm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x03, 0x59, 0xf0, // VMULSD XMM14, XMM15, XMM0
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm15_xmm0_xmm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x61, 0x7b, 0x59, 0xf9, // VMULSD XMM15, XMM0, XMM1
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm0_xmm1_mem64() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xf3, 0x59, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULSD XMM0, XMM1, [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f]; // 1.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm2_xmm3_mem64() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xe3, 0x59, 0x14, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULSD XMM2, XMM3, [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40]; // 2.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

#[test]
fn test_vmulsd_xmm4_xmm5_mem64() {
    let mut emu = emu64();
    let code = [
        0xc5, 0xd3, 0x59, 0x24, 0x25, 0x00, 0x30, 0x00, 0x00, // VMULSD XMM4, XMM5, [0x3000]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    let test_data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40]; // 3.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}
