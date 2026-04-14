use crate::*;

// VXORPS - Bitwise Logical XOR of Packed Single Precision Floating-Point Values
// VXORPD - Bitwise Logical XOR of Packed Double Precision Floating-Point Values
//
// These instructions perform bitwise XOR on packed floating-point values.
// XORing a register with itself is commonly used to zero a register.
//
// Opcodes:
// VEX.128.NP 0F 57 /r    VXORPS xmm1, xmm2, xmm3/m128
// VEX.256.NP 0F 57 /r    VXORPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 57 /r    VXORPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 57 /r    VXORPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VXORPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vxorps_xmm_basic() {
    let mut emu = emu64();
    // VXORPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x57, 0xc2, // VXORPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_zero_self() {
    let mut emu = emu64();
    // VXORPS XMM0, XMM0, XMM0 (zero register idiom)
    let code = [
        0xc5, 0xf8, 0x57, 0xc0, // VXORPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VXORPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf0, 0x57, 0xca, // VXORPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_all_regs() {
    let mut emu = emu64();
    // VXORPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd8, 0x57, 0xdd, // VXORPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_high_regs() {
    let mut emu = emu64();
    // VXORPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc0, 0x57, 0xf2, // VXORPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_extended_dest() {
    let mut emu = emu64();
    // VXORPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x70, 0x57, 0xc2, // VXORPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_extended_src1() {
    let mut emu = emu64();
    // VXORPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x30, 0x57, 0xca, // VXORPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_extended_src2() {
    let mut emu = emu64();
    // VXORPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x68, 0x57, 0xca, // VXORPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_all_extended() {
    let mut emu = emu64();
    // VXORPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x18, 0x57, 0xdd, // VXORPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VXORPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x00, 0x57, 0xf0, // VXORPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_zero_extended() {
    let mut emu = emu64();
    // VXORPS XMM8, XMM8, XMM8 (zero extended register)
    let code = [
        0xc4, 0xc1, 0x38, 0x57, 0xc0, // VXORPS XMM8, XMM8, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_zero_r15() {
    let mut emu = emu64();
    // VXORPS XMM15, XMM15, XMM15 (zero XMM15)
    let code = [
        0xc4, 0xc1, 0x00, 0x57, 0xff, // VXORPS XMM15, XMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_mem() {
    let mut emu = emu64();
    // VXORPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf8, 0x57, 0x0d, 0x00, 0x40, 0x00, 0x00, // VXORPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xaa, 0xaa, 0x55, 0x55, 0x33, 0x33, 0xcc, 0xcc,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_xmm_mem_extended() {
    let mut emu = emu64();
    // VXORPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x20, 0x57, 0x15, 0x00, 0x40, 0x00, 0x00, // VXORPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VXORPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vxorps_ymm_basic() {
    let mut emu = emu64();
    // VXORPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x57, 0xc2, // VXORPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_zero_self() {
    let mut emu = emu64();
    // VXORPS YMM0, YMM0, YMM0 (zero register idiom)
    let code = [
        0xc5, 0xfc, 0x57, 0xc0, // VXORPS YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VXORPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf4, 0x57, 0xca, // VXORPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_all_regs() {
    let mut emu = emu64();
    // VXORPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdc, 0x57, 0xdd, // VXORPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_high_regs() {
    let mut emu = emu64();
    // VXORPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc4, 0x57, 0xf2, // VXORPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_extended_dest() {
    let mut emu = emu64();
    // VXORPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x74, 0x57, 0xc2, // VXORPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_extended_src1() {
    let mut emu = emu64();
    // VXORPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x34, 0x57, 0xca, // VXORPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_extended_src2() {
    let mut emu = emu64();
    // VXORPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6c, 0x57, 0xca, // VXORPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_all_extended() {
    let mut emu = emu64();
    // VXORPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1c, 0x57, 0xdd, // VXORPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VXORPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x04, 0x57, 0xf0, // VXORPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_zero_extended() {
    let mut emu = emu64();
    // VXORPS YMM10, YMM10, YMM10 (zero extended register)
    let code = [
        0xc4, 0xc1, 0x2c, 0x57, 0xd2, // VXORPS YMM10, YMM10, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_zero_r15() {
    let mut emu = emu64();
    // VXORPS YMM15, YMM15, YMM15 (zero YMM15)
    let code = [
        0xc4, 0xc1, 0x04, 0x57, 0xff, // VXORPS YMM15, YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_mem() {
    let mut emu = emu64();
    // VXORPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfc, 0x57, 0x0d, 0x00, 0x40, 0x00, 0x00, // VXORPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xaa, 0xaa, 0x55, 0x55, 0x33, 0x33, 0xcc, 0xcc,
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vxorps_ymm_mem_extended() {
    let mut emu = emu64();
    // VXORPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x24, 0x57, 0x15, 0x00, 0x40, 0x00, 0x00, // VXORPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VXORPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vxorpd_xmm_basic() {
    let mut emu = emu64();
    // VXORPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x57, 0xc2, // VXORPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_zero_self() {
    let mut emu = emu64();
    // VXORPD XMM0, XMM0, XMM0 (zero register idiom)
    let code = [
        0xc5, 0xf9, 0x57, 0xc0, // VXORPD XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VXORPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf1, 0x57, 0xca, // VXORPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_all_regs() {
    let mut emu = emu64();
    // VXORPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xd9, 0x57, 0xdd, // VXORPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_high_regs() {
    let mut emu = emu64();
    // VXORPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc1, 0x57, 0xf2, // VXORPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_extended_dest() {
    let mut emu = emu64();
    // VXORPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x71, 0x57, 0xc2, // VXORPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_extended_src1() {
    let mut emu = emu64();
    // VXORPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x31, 0x57, 0xca, // VXORPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_extended_src2() {
    let mut emu = emu64();
    // VXORPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x69, 0x57, 0xca, // VXORPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_all_extended() {
    let mut emu = emu64();
    // VXORPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x19, 0x57, 0xdd, // VXORPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VXORPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x01, 0x57, 0xf0, // VXORPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_zero_extended() {
    let mut emu = emu64();
    // VXORPD XMM12, XMM12, XMM12 (zero extended register)
    let code = [
        0xc4, 0xc1, 0x19, 0x57, 0xe4, // VXORPD XMM12, XMM12, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_mem() {
    let mut emu = emu64();
    // VXORPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xf9, 0x57, 0x0d, 0x00, 0x40, 0x00, 0x00, // VXORPD XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_xmm_mem_extended() {
    let mut emu = emu64();
    // VXORPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x21, 0x57, 0x15, 0x00, 0x40, 0x00, 0x00, // VXORPD XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VXORPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vxorpd_ymm_basic() {
    let mut emu = emu64();
    // VXORPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x57, 0xc2, // VXORPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_zero_self() {
    let mut emu = emu64();
    // VXORPD YMM0, YMM0, YMM0 (zero register idiom)
    let code = [
        0xc5, 0xfd, 0x57, 0xc0, // VXORPD YMM0, YMM0, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VXORPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf5, 0x57, 0xca, // VXORPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_all_regs() {
    let mut emu = emu64();
    // VXORPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdd, 0x57, 0xdd, // VXORPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_high_regs() {
    let mut emu = emu64();
    // VXORPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc5, 0x57, 0xf2, // VXORPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_extended_dest() {
    let mut emu = emu64();
    // VXORPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x75, 0x57, 0xc2, // VXORPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_extended_src1() {
    let mut emu = emu64();
    // VXORPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x35, 0x57, 0xca, // VXORPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_extended_src2() {
    let mut emu = emu64();
    // VXORPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6d, 0x57, 0xca, // VXORPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_all_extended() {
    let mut emu = emu64();
    // VXORPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1d, 0x57, 0xdd, // VXORPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VXORPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x05, 0x57, 0xf0, // VXORPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_zero_extended() {
    let mut emu = emu64();
    // VXORPD YMM13, YMM13, YMM13 (zero extended register)
    let code = [
        0xc4, 0xc1, 0x15, 0x57, 0xed, // VXORPD YMM13, YMM13, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_mem() {
    let mut emu = emu64();
    // VXORPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xfd, 0x57, 0x0d, 0x00, 0x40, 0x00, 0x00, // VXORPD YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
        0xaa, 0xaa, 0xaa, 0xaa, 0x55, 0x55, 0x55, 0x55,
        0x33, 0x33, 0x33, 0x33, 0xcc, 0xcc, 0xcc, 0xcc,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vxorpd_ymm_mem_extended() {
    let mut emu = emu64();
    // VXORPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x25, 0x57, 0x15, 0x00, 0x40, 0x00, 0x00, // VXORPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
