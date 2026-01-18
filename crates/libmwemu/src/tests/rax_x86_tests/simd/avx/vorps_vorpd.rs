use crate::*;

// VORPS - Bitwise Logical OR of Packed Single Precision Floating-Point Values
// VORPD - Bitwise Logical OR of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise OR on packed floating-point values.
//
// Opcodes:
// VEX.128.NP 0F 56 /r    VORPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 56 /r    VORPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 56 /r    VORPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 56 /r    VORPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VORPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vorps_xmm_basic() {
    let mut emu = emu64();
    // VORPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x56, 0xc2, // VORPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VORPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x56, 0xca, // VORPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_all_regs() {
    let mut emu = emu64();
    // VORPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x56, 0xdd, // VORPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_high_regs() {
    let mut emu = emu64();
    // VORPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x56, 0xf2, // VORPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_extended_dest() {
    let mut emu = emu64();
    // VORPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x56, 0xc2, // VORPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_extended_src1() {
    let mut emu = emu64();
    // VORPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x56, 0xca, // VORPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_extended_src2() {
    let mut emu = emu64();
    // VORPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x56, 0xca, // VORPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_all_extended() {
    let mut emu = emu64();
    // VORPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x56, 0xdd, // VORPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VORPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x56, 0xf0, // VORPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_mem() {
    let mut emu = emu64();
    // VORPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff,
        0xaa, 0x55, 0xaa, 0x55, 0x33, 0xcc, 0x33, 0xcc,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_mem_extended() {
    let mut emu = emu64();
    // VORPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x56, 0x15, 0x00, 0x40, 0x00, 0x00, // VORPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorps_xmm_self() {
    let mut emu = emu64();
    // VORPS XMM0, XMM0, XMM0 (self OR should produce same value)
    let code = [
        0xc5, 0xf8, 0x56, 0xc0, // VORPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VORPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vorps_ymm_basic() {
    let mut emu = emu64();
    // VORPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x56, 0xc2, // VORPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VORPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x56, 0xca, // VORPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_all_regs() {
    let mut emu = emu64();
    // VORPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x56, 0xdd, // VORPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_high_regs() {
    let mut emu = emu64();
    // VORPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x56, 0xf2, // VORPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_extended_dest() {
    let mut emu = emu64();
    // VORPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x56, 0xc2, // VORPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_extended_src1() {
    let mut emu = emu64();
    // VORPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x56, 0xca, // VORPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_extended_src2() {
    let mut emu = emu64();
    // VORPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x56, 0xca, // VORPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_all_extended() {
    let mut emu = emu64();
    // VORPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x56, 0xdd, // VORPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VORPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x56, 0xf0, // VORPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_mem() {
    let mut emu = emu64();
    // VORPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0xff, 0x00, 0xff, 0x00, 0x00, 0xff, 0x00, 0xff,
        0xaa, 0x55, 0xaa, 0x55, 0x33, 0xcc, 0x33, 0xcc,
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
        0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_mem_extended() {
    let mut emu = emu64();
    // VORPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x56, 0x15, 0x00, 0x40, 0x00, 0x00, // VORPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorps_ymm_self() {
    let mut emu = emu64();
    // VORPS YMM5, YMM5, YMM5 (self OR should produce same value)
    let code = [
        0xc5, 0xd4, 0x56, 0xed, // VORPS YMM5, YMM5, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VORPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vorpd_xmm_basic() {
    let mut emu = emu64();
    // VORPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x56, 0xc2, // VORPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VORPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x56, 0xca, // VORPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_all_regs() {
    let mut emu = emu64();
    // VORPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x56, 0xdd, // VORPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_high_regs() {
    let mut emu = emu64();
    // VORPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x56, 0xf2, // VORPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_extended_dest() {
    let mut emu = emu64();
    // VORPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x56, 0xc2, // VORPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_extended_src1() {
    let mut emu = emu64();
    // VORPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x56, 0xca, // VORPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_extended_src2() {
    let mut emu = emu64();
    // VORPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x56, 0xca, // VORPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_all_extended() {
    let mut emu = emu64();
    // VORPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x56, 0xdd, // VORPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VORPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x56, 0xf0, // VORPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_mem() {
    let mut emu = emu64();
    // VORPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPD XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00,
        0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_mem_extended() {
    let mut emu = emu64();
    // VORPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x56, 0x15, 0x00, 0x40, 0x00, 0x00, // VORPD XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_xmm_self() {
    let mut emu = emu64();
    // VORPD XMM2, XMM2, XMM2 (self OR should produce same value)
    let code = [
        0xc5, 0xe9, 0x56, 0xd2, // VORPD XMM2, XMM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VORPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vorpd_ymm_basic() {
    let mut emu = emu64();
    // VORPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x56, 0xc2, // VORPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VORPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x56, 0xca, // VORPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_all_regs() {
    let mut emu = emu64();
    // VORPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x56, 0xdd, // VORPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_high_regs() {
    let mut emu = emu64();
    // VORPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x56, 0xf2, // VORPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_extended_dest() {
    let mut emu = emu64();
    // VORPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x56, 0xc2, // VORPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_extended_src1() {
    let mut emu = emu64();
    // VORPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x56, 0xca, // VORPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_extended_src2() {
    let mut emu = emu64();
    // VORPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x56, 0xca, // VORPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_all_extended() {
    let mut emu = emu64();
    // VORPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x56, 0xdd, // VORPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VORPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x56, 0xf0, // VORPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_mem() {
    let mut emu = emu64();
    // VORPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x56, 0x0d, 0x00, 0x40, 0x00, 0x00, // VORPD YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00,
        0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff,
        0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55,
        0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_mem_extended() {
    let mut emu = emu64();
    // VORPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x56, 0x15, 0x00, 0x40, 0x00, 0x00, // VORPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vorpd_ymm_self() {
    let mut emu = emu64();
    // VORPD YMM7, YMM7, YMM7 (self OR should produce same value)
    let code = [
        0xc5, 0xc5, 0x56, 0xff, // VORPD YMM7, YMM7, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
