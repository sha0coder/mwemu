use crate::*;

// VANDPS - Bitwise Logical AND of Packed Single Precision Floating-Point Values
// VANDPD - Bitwise Logical AND of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise AND on packed floating-point values.
//
// Opcodes:
// VEX.128.NP 0F 54 /r    VANDPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 54 /r    VANDPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 54 /r    VANDPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 54 /r    VANDPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VANDPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vandps_xmm_basic() {
    let mut emu = emu64();
    // VANDPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x54, 0xc2, // VANDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VANDPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x54, 0xca, // VANDPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_all_regs() {
    let mut emu = emu64();
    // VANDPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x54, 0xdd, // VANDPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_high_regs() {
    let mut emu = emu64();
    // VANDPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x54, 0xf2, // VANDPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_extended_dest() {
    let mut emu = emu64();
    // VANDPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x54, 0xc2, // VANDPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_extended_src1() {
    let mut emu = emu64();
    // VANDPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x54, 0xca, // VANDPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_extended_src2() {
    let mut emu = emu64();
    // VANDPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x54, 0xca, // VANDPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_all_extended() {
    let mut emu = emu64();
    // VANDPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x54, 0xdd, // VANDPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x54, 0xf0, // VANDPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_mem() {
    let mut emu = emu64();
    // VANDPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0x00, 0x00, 0xaa, 0xaa, 0xaa, 0xaa,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandps_xmm_mem_extended() {
    let mut emu = emu64();
    // VANDPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x54, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VANDPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vandps_ymm_basic() {
    let mut emu = emu64();
    // VANDPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x54, 0xc2, // VANDPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VANDPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x54, 0xca, // VANDPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_all_regs() {
    let mut emu = emu64();
    // VANDPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x54, 0xdd, // VANDPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_high_regs() {
    let mut emu = emu64();
    // VANDPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x54, 0xf2, // VANDPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_extended_dest() {
    let mut emu = emu64();
    // VANDPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x54, 0xc2, // VANDPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_extended_src1() {
    let mut emu = emu64();
    // VANDPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x54, 0xca, // VANDPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_extended_src2() {
    let mut emu = emu64();
    // VANDPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x54, 0xca, // VANDPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_all_extended() {
    let mut emu = emu64();
    // VANDPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x54, 0xdd, // VANDPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x54, 0xf0, // VANDPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_mem() {
    let mut emu = emu64();
    // VANDPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0x00, 0x00, 0xaa, 0xaa, 0xaa, 0xaa,
        0x55, 0x55, 0x55, 0x55, 0x12, 0x34, 0x56, 0x78,
        0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandps_ymm_mem_extended() {
    let mut emu = emu64();
    // VANDPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x54, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VANDPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vandpd_xmm_basic() {
    let mut emu = emu64();
    // VANDPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x54, 0xc2, // VANDPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VANDPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x54, 0xca, // VANDPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_all_regs() {
    let mut emu = emu64();
    // VANDPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x54, 0xdd, // VANDPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_high_regs() {
    let mut emu = emu64();
    // VANDPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x54, 0xf2, // VANDPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_extended_dest() {
    let mut emu = emu64();
    // VANDPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x54, 0xc2, // VANDPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_extended_src1() {
    let mut emu = emu64();
    // VANDPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x54, 0xca, // VANDPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_extended_src2() {
    let mut emu = emu64();
    // VANDPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x54, 0xca, // VANDPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_all_extended() {
    let mut emu = emu64();
    // VANDPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x54, 0xdd, // VANDPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x54, 0xf0, // VANDPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_mem() {
    let mut emu = emu64();
    // VANDPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPD XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_xmm_mem_extended() {
    let mut emu = emu64();
    // VANDPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x54, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDPD XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VANDPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vandpd_ymm_basic() {
    let mut emu = emu64();
    // VANDPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x54, 0xc2, // VANDPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VANDPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x54, 0xca, // VANDPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_all_regs() {
    let mut emu = emu64();
    // VANDPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x54, 0xdd, // VANDPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_high_regs() {
    let mut emu = emu64();
    // VANDPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x54, 0xf2, // VANDPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_extended_dest() {
    let mut emu = emu64();
    // VANDPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x54, 0xc2, // VANDPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_extended_src1() {
    let mut emu = emu64();
    // VANDPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x54, 0xca, // VANDPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_extended_src2() {
    let mut emu = emu64();
    // VANDPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x54, 0xca, // VANDPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_all_extended() {
    let mut emu = emu64();
    // VANDPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x54, 0xdd, // VANDPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x54, 0xf0, // VANDPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_mem() {
    let mut emu = emu64();
    // VANDPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x54, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDPD YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandpd_ymm_mem_extended() {
    let mut emu = emu64();
    // VANDPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x54, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
