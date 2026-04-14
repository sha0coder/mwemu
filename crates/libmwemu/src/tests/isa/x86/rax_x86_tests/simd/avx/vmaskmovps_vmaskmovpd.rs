use crate::*;

// VMASKMOVPS - Conditional Load/Store Packed Single-Precision Floating-Point Values
// VMASKMOVPD - Conditional Load/Store Packed Double-Precision Floating-Point Values
//
// VMASKMOVPS conditionally loads or stores packed single-precision floating-point values
// based on a mask. The sign bit of each element in the mask determines if the corresponding
// element is loaded/stored (sign bit = 1) or zeroed/unchanged (sign bit = 0).
//
// VMASKMOVPD conditionally loads or stores packed double-precision floating-point values
// based on a mask.
//
// Load form: VMASKMOVPS xmm1, xmm2, m128  (xmm1 = conditionally loaded from m128 using xmm2 mask)
// Store form: VMASKMOVPS m128, xmm1, xmm2 (conditionally store xmm2 to m128 using xmm1 mask)
//
// Opcodes:
// VEX.128.66.0F38.W0 2C /r   VMASKMOVPS xmm1, xmm2, m128   - Conditional load
// VEX.256.66.0F38.W0 2C /r   VMASKMOVPS ymm1, ymm2, m256   - Conditional load
// VEX.128.66.0F38.W0 2E /r   VMASKMOVPS m128, xmm1, xmm2   - Conditional store
// VEX.256.66.0F38.W0 2E /r   VMASKMOVPS m256, ymm1, ymm2   - Conditional store
// VEX.128.66.0F38.W0 2D /r   VMASKMOVPD xmm1, xmm2, m128   - Conditional load
// VEX.256.66.0F38.W0 2D /r   VMASKMOVPD ymm1, ymm2, m256   - Conditional load
// VEX.128.66.0F38.W0 2F /r   VMASKMOVPD m128, xmm1, xmm2   - Conditional store
// VEX.256.66.0F38.W0 2F /r   VMASKMOVPD m256, ymm1, ymm2   - Conditional store

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VMASKMOVPS Tests - 128-bit Load (4x float32)
// ============================================================================

#[test]
fn test_vmaskmovps_load_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm1_xmm2_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM1, XMM2, [mem]
    let code = [
        0xc4, 0xe2, 0x69, 0x2c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM1, XMM2, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40, // 5.0
        0x00, 0x00, 0xc0, 0x40, // 6.0
        0x00, 0x00, 0xe0, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x41, // 8.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm2_xmm3_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM2, XMM3, [mem]
    let code = [
        0xc4, 0xe2, 0x61, 0x2c, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM2, XMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x10, 0x41, // 9.0
        0x00, 0x00, 0x20, 0x41, // 10.0
        0x00, 0x00, 0x30, 0x41, // 11.0
        0x00, 0x00, 0x40, 0x41, // 12.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm3_xmm4_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM3, XMM4, [mem]
    let code = [
        0xc4, 0xe2, 0x59, 0x2c, 0x1d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM3, XMM4, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x50, 0x41, // 13.0
        0x00, 0x00, 0x60, 0x41, // 14.0
        0x00, 0x00, 0x70, 0x41, // 15.0
        0x00, 0x00, 0x80, 0x41, // 16.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm4_xmm5_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM4, XMM5, [mem]
    let code = [
        0xc4, 0xe2, 0x51, 0x2c, 0x25, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM4, XMM5, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x88, 0x41, // 17.0
        0x00, 0x00, 0x90, 0x41, // 18.0
        0x00, 0x00, 0x98, 0x41, // 19.0
        0x00, 0x00, 0xa0, 0x41, // 20.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm5_xmm6_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM5, XMM6, [mem]
    let code = [
        0xc4, 0xe2, 0x49, 0x2c, 0x2d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM5, XMM6, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm6_xmm7_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM6, XMM7, [mem]
    let code = [
        0xc4, 0xe2, 0x41, 0x2c, 0x35, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM6, XMM7, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm7_xmm0_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM7, XMM0, [mem]
    let code = [
        0xc4, 0xe2, 0x79, 0x2c, 0x3d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM7, XMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPS Tests - Extended XMM registers Load
// ============================================================================

#[test]
fn test_vmaskmovps_load_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x62, 0x31, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x40]; // 2.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm9_xmm10_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM9, XMM10, [mem]
    let code = [
        0xc4, 0x62, 0x29, 0x2c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM9, XMM10, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00, 0x40, 0x40]; // 3.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm10_xmm11_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0x62, 0x21, 0x2c, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM10, XMM11, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x80, 0x40, 0x00, 0x00, 0x80, 0x40, 0x00, 0x00, 0x80, 0x40, 0x00, 0x00, 0x80, 0x40]; // 4.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_xmm15_xmm14_mem() {
    let mut emu = emu64();
    // VMASKMOVPS XMM15, XMM14, [mem]
    let code = [
        0xc4, 0x62, 0x09, 0x2c, 0x3d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM15, XMM14, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xa0, 0x40, 0x00, 0x00, 0xa0, 0x40]; // 5.0
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPS Tests - 256-bit Load (8x float32)
// ============================================================================

#[test]
fn test_vmaskmovps_load_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VMASKMOVPS YMM0, YMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x75, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS YMM0, YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0xa0, 0x40, // 5.0
        0x00, 0x00, 0xc0, 0x40, // 6.0
        0x00, 0x00, 0xe0, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x41, // 8.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_ymm1_ymm2_mem() {
    let mut emu = emu64();
    // VMASKMOVPS YMM1, YMM2, [mem]
    let code = [
        0xc4, 0xe2, 0x6d, 0x2c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS YMM1, YMM2, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_ymm2_ymm3_mem() {
    let mut emu = emu64();
    // VMASKMOVPS YMM2, YMM3, [mem]
    let code = [
        0xc4, 0xe2, 0x65, 0x2c, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS YMM2, YMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_load_ymm8_ymm9_mem() {
    let mut emu = emu64();
    // VMASKMOVPS YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x62, 0x35, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS YMM8, YMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPS Tests - 128-bit Store
// ============================================================================

#[test]
fn test_vmaskmovps_store_mem_xmm0_xmm1() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0x2e, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_store_mem_xmm1_xmm2() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0x2e, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_store_mem_xmm2_xmm3() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0x2e, 0x1d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_store_mem_xmm3_xmm4() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], XMM3, XMM4
    let code = [
        0xc4, 0xe2, 0x61, 0x2e, 0x25, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_store_mem_xmm8_xmm9() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], XMM8, XMM9
    let code = [
        0xc4, 0x62, 0x39, 0x2e, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPS Tests - 256-bit Store
// ============================================================================

#[test]
fn test_vmaskmovps_store_mem_ymm0_ymm1() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], YMM0, YMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x2e, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_store_mem_ymm1_ymm2() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0x2e, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovps_store_mem_ymm8_ymm9() {
    let mut emu = emu64();
    // VMASKMOVPS [mem], YMM8, YMM9
    let code = [
        0xc4, 0x62, 0x3d, 0x2e, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4000], YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPD Tests - 128-bit Load (2x float64)
// ============================================================================

#[test]
fn test_vmaskmovpd_load_xmm0_xmm1_mem() {
    let mut emu = emu64();
    // VMASKMOVPD XMM0, XMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x71, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD XMM0, XMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_load_xmm1_xmm2_mem() {
    let mut emu = emu64();
    // VMASKMOVPD XMM1, XMM2, [mem]
    let code = [
        0xc4, 0xe2, 0x69, 0x2d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD XMM1, XMM2, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_load_xmm2_xmm3_mem() {
    let mut emu = emu64();
    // VMASKMOVPD XMM2, XMM3, [mem]
    let code = [
        0xc4, 0xe2, 0x61, 0x2d, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD XMM2, XMM3, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x40, // 5.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x40, // 6.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_load_xmm8_xmm9_mem() {
    let mut emu = emu64();
    // VMASKMOVPD XMM8, XMM9, [mem]
    let code = [
        0xc4, 0x62, 0x31, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD XMM8, XMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x40, // 7.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x40, // 8.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPD Tests - 256-bit Load (4x float64)
// ============================================================================

#[test]
fn test_vmaskmovpd_load_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VMASKMOVPD YMM0, YMM1, [mem]
    let code = [
        0xc4, 0xe2, 0x75, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD YMM0, YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40, // 3.0
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_load_ymm1_ymm2_mem() {
    let mut emu = emu64();
    // VMASKMOVPD YMM1, YMM2, [mem]
    let code = [
        0xc4, 0xe2, 0x6d, 0x2d, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD YMM1, YMM2, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_load_ymm8_ymm9_mem() {
    let mut emu = emu64();
    // VMASKMOVPD YMM8, YMM9, [mem]
    let code = [
        0xc4, 0x62, 0x35, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD YMM8, YMM9, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPD Tests - 128-bit Store
// ============================================================================

#[test]
fn test_vmaskmovpd_store_mem_xmm0_xmm1() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], XMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x79, 0x2f, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_store_mem_xmm1_xmm2() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], XMM1, XMM2
    let code = [
        0xc4, 0xe2, 0x71, 0x2f, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_store_mem_xmm2_xmm3() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], XMM2, XMM3
    let code = [
        0xc4, 0xe2, 0x69, 0x2f, 0x1d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_store_mem_xmm8_xmm9() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], XMM8, XMM9
    let code = [
        0xc4, 0x62, 0x39, 0x2f, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VMASKMOVPD Tests - 256-bit Store
// ============================================================================

#[test]
fn test_vmaskmovpd_store_mem_ymm0_ymm1() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], YMM0, YMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x2f, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_store_mem_ymm1_ymm2() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], YMM1, YMM2
    let code = [
        0xc4, 0xe2, 0x75, 0x2f, 0x15, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_store_mem_ymm8_ymm9() {
    let mut emu = emu64();
    // VMASKMOVPD [mem], YMM8, YMM9
    let code = [
        0xc4, 0x62, 0x3d, 0x2f, 0x0d, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4000], YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined Load/Store Tests
// ============================================================================

#[test]
fn test_vmaskmovps_load_store_roundtrip() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x2c, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPS XMM0, XMM1, [rip + 0x4000]
        0xc4, 0xe2, 0x71, 0x2e, 0x0d, 0x10, 0x40, 0x00, 0x00, // VMASKMOVPS [rip + 0x4010], XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vmaskmovpd_load_store_roundtrip() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x71, 0x2d, 0x05, 0x00, 0x40, 0x00, 0x00, // VMASKMOVPD XMM0, XMM1, [rip + 0x4000]
        0xc4, 0xe2, 0x71, 0x2f, 0x0d, 0x10, 0x40, 0x00, 0x00, // VMASKMOVPD [rip + 0x4010], XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
