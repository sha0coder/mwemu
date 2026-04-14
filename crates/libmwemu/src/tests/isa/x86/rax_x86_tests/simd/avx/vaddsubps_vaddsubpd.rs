use crate::*;

// VADDSUBPS - Add/Subtract Packed Single-Precision Floating-Point Values
// VADDSUBPD - Add/Subtract Packed Double-Precision Floating-Point Values
//
// These instructions perform addition and subtraction alternately on packed values.
// For VADDSUBPS: elements 0 and 2 are subtracted, elements 1 and 3 are added
// For VADDSUBPD: element 0 is subtracted, element 1 is added
//
// Opcodes:
// VEX.128.F2.0F.WIG D0 /r    VADDSUBPS xmm1, xmm2, xmm3/m128
// VEX.256.F2.0F.WIG D0 /r    VADDSUBPS ymm1, ymm2, ymm3/m256
// VEX.128.66.0F.WIG D0 /r    VADDSUBPD xmm1, xmm2, xmm3/m128
// VEX.256.66.0F.WIG D0 /r    VADDSUBPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VADDSUBPS Tests - 128-bit
// ============================================================================

#[test]
fn test_vaddsubps_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf3, 0xd0, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xeb, 0xd0, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe3, 0xd0, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xdb, 0xd0, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd3, 0xd0, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xcb, 0xd0, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x43, 0xd0, 0xf0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x3b, 0xd0, 0xf9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_xmm0_xmm1_mem128() {
    let mut emu = emu64();
    let code = [0xc5, 0xf3, 0xd0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

// ============================================================================
// VADDSUBPS Tests - 256-bit
// ============================================================================

#[test]
fn test_vaddsubps_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf7, 0xd0, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xef, 0xd0, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe7, 0xd0, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xdf, 0xd0, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd7, 0xd0, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xcf, 0xd0, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubps_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xf7, 0xd0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

// ============================================================================
// VADDSUBPD Tests - 128-bit
// ============================================================================

#[test]
fn test_vaddsubpd_xmm0_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0xd0, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm1_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xe9, 0xd0, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm2_xmm3_xmm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe1, 0xd0, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm3_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xd9, 0xd0, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm4_xmm5_xmm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd1, 0xd0, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm5_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xc9, 0xd0, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm6_xmm7_xmm8() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x41, 0xd0, 0xf0, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm7_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0xc4, 0xc1, 0x39, 0xd0, 0xf9, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_xmm0_xmm1_mem128() {
    let mut emu = emu64();
    let code = [0xc5, 0xf1, 0xd0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 16];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}

// ============================================================================
// VADDSUBPD Tests - 256-bit
// ============================================================================

#[test]
fn test_vaddsubpd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    let code = [0xc5, 0xf5, 0xd0, 0xc2, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [0xc5, 0xed, 0xd0, 0xcb, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [0xc5, 0xe5, 0xd0, 0xd4, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [0xc5, 0xdd, 0xd0, 0xdd, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_ymm4_ymm5_ymm6() {
    let mut emu = emu64();
    let code = [0xc5, 0xd5, 0xd0, 0xe6, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_ymm5_ymm6_ymm7() {
    let mut emu = emu64();
    let code = [0xc5, 0xcd, 0xd0, 0xef, 0xf4];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddsubpd_ymm0_ymm1_mem256() {
    let mut emu = emu64();
    let code = [0xc5, 0xf5, 0xd0, 0x04, 0x25, 0x00, 0x30, 0x00, 0x00, 0xf4];
    emu.load_code_bytes(&code);
    let test_data = [0u8; 32];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);
    emu.run(None).unwrap();
}
