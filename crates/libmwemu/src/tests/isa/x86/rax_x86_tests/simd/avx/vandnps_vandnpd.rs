use crate::*;

// VANDNPS - Bitwise Logical AND NOT of Packed Single Precision Floating-Point Values
// VANDNPD - Bitwise Logical AND NOT of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise AND NOT on packed floating-point values.
// The operation is: dest = NOT(src1) AND src2
//
// Opcodes:
// VEX.128.NP 0F 55 /r    VANDNPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 55 /r    VANDNPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 55 /r    VANDNPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 55 /r    VANDNPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VANDNPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vandnps_xmm_basic() {
    let mut emu = emu64();
    // VANDNPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x55, 0xc2, // VANDNPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VANDNPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x55, 0xca, // VANDNPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_all_regs() {
    let mut emu = emu64();
    // VANDNPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x55, 0xdd, // VANDNPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_high_regs() {
    let mut emu = emu64();
    // VANDNPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x55, 0xf2, // VANDNPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_extended_dest() {
    let mut emu = emu64();
    // VANDNPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x55, 0xc2, // VANDNPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_extended_src1() {
    let mut emu = emu64();
    // VANDNPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x55, 0xca, // VANDNPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_extended_src2() {
    let mut emu = emu64();
    // VANDNPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x55, 0xca, // VANDNPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_all_extended() {
    let mut emu = emu64();
    // VANDNPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x55, 0xdd, // VANDNPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDNPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x55, 0xf0, // VANDNPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_mem() {
    let mut emu = emu64();
    // VANDNPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPS XMM1, XMM0, [rip+0x4000]
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
fn test_vandnps_xmm_mem_extended() {
    let mut emu = emu64();
    // VANDNPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x55, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDNPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_self() {
    let mut emu = emu64();
    // VANDNPS XMM0, XMM0, XMM0 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xf8, 0x55, 0xc0, // VANDNPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_xmm_different_operands() {
    let mut emu = emu64();
    // VANDNPS XMM5, XMM3, XMM7
    let code = [
        0xc5, 0xe0, 0x55, 0xef, // VANDNPS XMM5, XMM3, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VANDNPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vandnps_ymm_basic() {
    let mut emu = emu64();
    // VANDNPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x55, 0xc2, // VANDNPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VANDNPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x55, 0xca, // VANDNPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_all_regs() {
    let mut emu = emu64();
    // VANDNPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x55, 0xdd, // VANDNPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_high_regs() {
    let mut emu = emu64();
    // VANDNPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x55, 0xf2, // VANDNPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_extended_dest() {
    let mut emu = emu64();
    // VANDNPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x55, 0xc2, // VANDNPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_extended_src1() {
    let mut emu = emu64();
    // VANDNPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x55, 0xca, // VANDNPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_extended_src2() {
    let mut emu = emu64();
    // VANDNPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x55, 0xca, // VANDNPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_all_extended() {
    let mut emu = emu64();
    // VANDNPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x55, 0xdd, // VANDNPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDNPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x55, 0xf0, // VANDNPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_mem() {
    let mut emu = emu64();
    // VANDNPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPS YMM1, YMM0, [rip+0x4000]
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
fn test_vandnps_ymm_mem_extended() {
    let mut emu = emu64();
    // VANDNPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x55, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDNPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_self() {
    let mut emu = emu64();
    // VANDNPS YMM5, YMM5, YMM5 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xd4, 0x55, 0xed, // VANDNPS YMM5, YMM5, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnps_ymm_different_operands() {
    let mut emu = emu64();
    // VANDNPS YMM4, YMM2, YMM6
    let code = [
        0xc5, 0xec, 0x55, 0xe6, // VANDNPS YMM4, YMM2, YMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VANDNPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vandnpd_xmm_basic() {
    let mut emu = emu64();
    // VANDNPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x55, 0xc2, // VANDNPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VANDNPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x55, 0xca, // VANDNPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_all_regs() {
    let mut emu = emu64();
    // VANDNPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x55, 0xdd, // VANDNPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_high_regs() {
    let mut emu = emu64();
    // VANDNPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x55, 0xf2, // VANDNPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_extended_dest() {
    let mut emu = emu64();
    // VANDNPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x55, 0xc2, // VANDNPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_extended_src1() {
    let mut emu = emu64();
    // VANDNPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x55, 0xca, // VANDNPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_extended_src2() {
    let mut emu = emu64();
    // VANDNPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x55, 0xca, // VANDNPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_all_extended() {
    let mut emu = emu64();
    // VANDNPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x55, 0xdd, // VANDNPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDNPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x55, 0xf0, // VANDNPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_mem() {
    let mut emu = emu64();
    // VANDNPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPD XMM1, XMM0, [rip+0x4000]
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
fn test_vandnpd_xmm_mem_extended() {
    let mut emu = emu64();
    // VANDNPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x55, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDNPD XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_self() {
    let mut emu = emu64();
    // VANDNPD XMM2, XMM2, XMM2 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xe9, 0x55, 0xd2, // VANDNPD XMM2, XMM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_xmm_different_operands() {
    let mut emu = emu64();
    // VANDNPD XMM7, XMM1, XMM4
    let code = [
        0xc5, 0xf1, 0x55, 0xfc, // VANDNPD XMM7, XMM1, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VANDNPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vandnpd_ymm_basic() {
    let mut emu = emu64();
    // VANDNPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x55, 0xc2, // VANDNPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VANDNPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x55, 0xca, // VANDNPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_all_regs() {
    let mut emu = emu64();
    // VANDNPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x55, 0xdd, // VANDNPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_high_regs() {
    let mut emu = emu64();
    // VANDNPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x55, 0xf2, // VANDNPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_extended_dest() {
    let mut emu = emu64();
    // VANDNPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x55, 0xc2, // VANDNPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_extended_src1() {
    let mut emu = emu64();
    // VANDNPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x55, 0xca, // VANDNPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_extended_src2() {
    let mut emu = emu64();
    // VANDNPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x55, 0xca, // VANDNPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_all_extended() {
    let mut emu = emu64();
    // VANDNPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x55, 0xdd, // VANDNPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VANDNPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x55, 0xf0, // VANDNPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_mem() {
    let mut emu = emu64();
    // VANDNPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x55, 0x0d, 0x00, 0x40, 0x00, 0x00, // VANDNPD YMM1, YMM0, [rip+0x4000]
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
fn test_vandnpd_ymm_mem_extended() {
    let mut emu = emu64();
    // VANDNPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x55, 0x15, 0x00, 0x40, 0x00, 0x00, // VANDNPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_self() {
    let mut emu = emu64();
    // VANDNPD YMM7, YMM7, YMM7 (NOT(x) AND x = 0)
    let code = [
        0xc5, 0xc5, 0x55, 0xff, // VANDNPD YMM7, YMM7, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vandnpd_ymm_different_operands() {
    let mut emu = emu64();
    // VANDNPD YMM6, YMM3, YMM1
    let code = [
        0xc5, 0xe5, 0x55, 0xf1, // VANDNPD YMM6, YMM3, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
