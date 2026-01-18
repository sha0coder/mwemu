use crate::*;

// VHADDPS - Packed Single Precision Floating-Point Horizontal Add
// VHADDPD - Packed Double Precision Floating-Point Horizontal Add
//
// These instructions perform horizontal addition on packed floating-point values.
// For VHADDPS: adds adjacent pairs of single-precision values
// For VHADDPD: adds adjacent pairs of double-precision values
//
// Operation:
// VHADDPS xmm1, xmm2, xmm3:
//   xmm1[31:0]   = xmm2[63:32]   + xmm2[31:0]
//   xmm1[63:32]  = xmm2[127:96]  + xmm2[95:64]
//   xmm1[95:64]  = xmm3[63:32]   + xmm3[31:0]
//   xmm1[127:96] = xmm3[127:96]  + xmm3[95:64]
//
// VHADDPD xmm1, xmm2, xmm3:
//   xmm1[63:0]   = xmm2[127:64] + xmm2[63:0]
//   xmm1[127:64] = xmm3[127:64] + xmm3[63:0]
//
// Opcodes:
// VEX.128.F2 0F 7C /r    VHADDPS xmm1, xmm2, xmm3/m128
// VEX.256.F2 0F 7C /r    VHADDPS ymm1, ymm2, ymm3/m256
// VEX.128.66 0F 7C /r    VHADDPD xmm1, xmm2, xmm3/m128
// VEX.256.66 0F 7C /r    VHADDPD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VHADDPS Tests - 128-bit (4x float32)
// ============================================================================

#[test]
fn test_vhaddps_xmm_basic() {
    let mut emu = emu64();
    // VHADDPS XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7c, 0xc2, // VHADDPS XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VHADDPS XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7c, 0xca, // VHADDPS XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_all_regs() {
    let mut emu = emu64();
    // VHADDPS XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xdb, 0x7c, 0xdd, // VHADDPS XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_high_regs() {
    let mut emu = emu64();
    // VHADDPS XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc3, 0x7c, 0xf2, // VHADDPS XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_extended_dest() {
    let mut emu = emu64();
    // VHADDPS XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x73, 0x7c, 0xc2, // VHADDPS XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_extended_src1() {
    let mut emu = emu64();
    // VHADDPS XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x33, 0x7c, 0xca, // VHADDPS XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_extended_src2() {
    let mut emu = emu64();
    // VHADDPS XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x6b, 0x7c, 0xca, // VHADDPS XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_all_extended() {
    let mut emu = emu64();
    // VHADDPS XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x1b, 0x7c, 0xdd, // VHADDPS XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VHADDPS XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x03, 0x7c, 0xf0, // VHADDPS XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_mem() {
    let mut emu = emu64();
    // VHADDPS XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xfb, 0x7c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHADDPS XMM1, XMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
        0x00, 0x00, 0x00, 0x40, // 2.0f
        0x00, 0x00, 0x40, 0x40, // 3.0f
        0x00, 0x00, 0x80, 0x40, // 4.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_mem_extended() {
    let mut emu = emu64();
    // VHADDPS XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x23, 0x7c, 0x15, 0x00, 0x40, 0x00, 0x00, // VHADDPS XMM10, XMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40, // 5.0f
        0x00, 0x00, 0xc0, 0x40, // 6.0f
        0x00, 0x00, 0xe0, 0x40, // 7.0f
        0x00, 0x00, 0x00, 0x41, // 8.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_xmm_self() {
    let mut emu = emu64();
    // VHADDPS XMM0, XMM0, XMM0
    let code = [
        0xc5, 0xfb, 0x7c, 0xc0, // VHADDPS XMM0, XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VHADDPS Tests - 256-bit (8x float32)
// ============================================================================

#[test]
fn test_vhaddps_ymm_basic() {
    let mut emu = emu64();
    // VHADDPS YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7c, 0xc2, // VHADDPS YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VHADDPS YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7c, 0xca, // VHADDPS YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_all_regs() {
    let mut emu = emu64();
    // VHADDPS YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdf, 0x7c, 0xdd, // VHADDPS YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_high_regs() {
    let mut emu = emu64();
    // VHADDPS YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc7, 0x7c, 0xf2, // VHADDPS YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_extended_dest() {
    let mut emu = emu64();
    // VHADDPS YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x77, 0x7c, 0xc2, // VHADDPS YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_extended_src1() {
    let mut emu = emu64();
    // VHADDPS YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x37, 0x7c, 0xca, // VHADDPS YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_extended_src2() {
    let mut emu = emu64();
    // VHADDPS YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6f, 0x7c, 0xca, // VHADDPS YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_all_extended() {
    let mut emu = emu64();
    // VHADDPS YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1f, 0x7c, 0xdd, // VHADDPS YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VHADDPS YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x07, 0x7c, 0xf0, // VHADDPS YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_mem() {
    let mut emu = emu64();
    // VHADDPS YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xff, 0x7c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHADDPS YMM1, YMM0, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0f
        0x00, 0x00, 0x00, 0x40, // 2.0f
        0x00, 0x00, 0x40, 0x40, // 3.0f
        0x00, 0x00, 0x80, 0x40, // 4.0f
        0x00, 0x00, 0xa0, 0x40, // 5.0f
        0x00, 0x00, 0xc0, 0x40, // 6.0f
        0x00, 0x00, 0xe0, 0x40, // 7.0f
        0x00, 0x00, 0x00, 0x41, // 8.0f
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_mem_extended() {
    let mut emu = emu64();
    // VHADDPS YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x27, 0x7c, 0x15, 0x00, 0x40, 0x00, 0x00, // VHADDPS YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vhaddps_ymm_self() {
    let mut emu = emu64();
    // VHADDPS YMM5, YMM5, YMM5
    let code = [
        0xc5, 0xd7, 0x7c, 0xed, // VHADDPS YMM5, YMM5, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VHADDPD Tests - 128-bit (2x float64)
// ============================================================================

#[test]
fn test_vhaddpd_xmm_basic() {
    let mut emu = emu64();
    // VHADDPD XMM0, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7c, 0xc2, // VHADDPD XMM0, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_same_dest_src1() {
    let mut emu = emu64();
    // VHADDPD XMM1, XMM1, XMM2
    let code = [
        0xc5, 0xf3, 0x7c, 0xca, // VHADDPD XMM1, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_all_regs() {
    let mut emu = emu64();
    // VHADDPD XMM3, XMM4, XMM5
    let code = [
        0xc5, 0xdb, 0x7c, 0xdd, // VHADDPD XMM3, XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_high_regs() {
    let mut emu = emu64();
    // VHADDPD XMM6, XMM7, XMM2
    let code = [
        0xc5, 0xc3, 0x7c, 0xf2, // VHADDPD XMM6, XMM7, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_extended_dest() {
    let mut emu = emu64();
    // VHADDPD XMM8, XMM1, XMM2
    let code = [
        0xc4, 0xc1, 0x73, 0x7c, 0xc2, // VHADDPD XMM8, XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_extended_src1() {
    let mut emu = emu64();
    // VHADDPD XMM1, XMM9, XMM2
    let code = [
        0xc4, 0xc1, 0x33, 0x7c, 0xca, // VHADDPD XMM1, XMM9, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_extended_src2() {
    let mut emu = emu64();
    // VHADDPD XMM1, XMM2, XMM10
    let code = [
        0xc4, 0xc1, 0x6b, 0x7c, 0xca, // VHADDPD XMM1, XMM2, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_all_extended() {
    let mut emu = emu64();
    // VHADDPD XMM11, XMM12, XMM13
    let code = [
        0xc4, 0xc1, 0x1b, 0x7c, 0xdd, // VHADDPD XMM11, XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_r14_r15_r8() {
    let mut emu = emu64();
    // VHADDPD XMM14, XMM15, XMM8
    let code = [
        0xc4, 0xc1, 0x03, 0x7c, 0xf0, // VHADDPD XMM14, XMM15, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_xmm_mem() {
    let mut emu = emu64();
    // VHADDPD XMM1, XMM0, [mem]
    let code = [
        0xc5, 0xfb, 0x7c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHADDPD XMM1, XMM0, [rip+0x4000]
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
fn test_vhaddpd_xmm_mem_extended() {
    let mut emu = emu64();
    // VHADDPD XMM10, XMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x23, 0x7c, 0x15, 0x00, 0x40, 0x00, 0x00, // VHADDPD XMM10, XMM11, [rip+0x4000]
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
fn test_vhaddpd_xmm_self() {
    let mut emu = emu64();
    // VHADDPD XMM2, XMM2, XMM2
    let code = [
        0xc5, 0xeb, 0x7c, 0xd2, // VHADDPD XMM2, XMM2, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VHADDPD Tests - 256-bit (4x float64)
// ============================================================================

#[test]
fn test_vhaddpd_ymm_basic() {
    let mut emu = emu64();
    // VHADDPD YMM0, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7c, 0xc2, // VHADDPD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_same_dest_src1() {
    let mut emu = emu64();
    // VHADDPD YMM1, YMM1, YMM2
    let code = [
        0xc5, 0xf7, 0x7c, 0xca, // VHADDPD YMM1, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_all_regs() {
    let mut emu = emu64();
    // VHADDPD YMM3, YMM4, YMM5
    let code = [
        0xc5, 0xdf, 0x7c, 0xdd, // VHADDPD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_high_regs() {
    let mut emu = emu64();
    // VHADDPD YMM6, YMM7, YMM2
    let code = [
        0xc5, 0xc7, 0x7c, 0xf2, // VHADDPD YMM6, YMM7, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_extended_dest() {
    let mut emu = emu64();
    // VHADDPD YMM8, YMM1, YMM2
    let code = [
        0xc4, 0xc1, 0x77, 0x7c, 0xc2, // VHADDPD YMM8, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_extended_src1() {
    let mut emu = emu64();
    // VHADDPD YMM1, YMM9, YMM2
    let code = [
        0xc4, 0xc1, 0x37, 0x7c, 0xca, // VHADDPD YMM1, YMM9, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_extended_src2() {
    let mut emu = emu64();
    // VHADDPD YMM1, YMM2, YMM10
    let code = [
        0xc4, 0xc1, 0x6f, 0x7c, 0xca, // VHADDPD YMM1, YMM2, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_all_extended() {
    let mut emu = emu64();
    // VHADDPD YMM11, YMM12, YMM13
    let code = [
        0xc4, 0xc1, 0x1f, 0x7c, 0xdd, // VHADDPD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_r14_r15_r8() {
    let mut emu = emu64();
    // VHADDPD YMM14, YMM15, YMM8
    let code = [
        0xc4, 0xc1, 0x07, 0x7c, 0xf0, // VHADDPD YMM14, YMM15, YMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_mem() {
    let mut emu = emu64();
    // VHADDPD YMM1, YMM0, [mem]
    let code = [
        0xc5, 0xff, 0x7c, 0x0d, 0x00, 0x40, 0x00, 0x00, // VHADDPD YMM1, YMM0, [rip+0x4000]
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
fn test_vhaddpd_ymm_mem_extended() {
    let mut emu = emu64();
    // VHADDPD YMM10, YMM11, [mem]
    let code = [
        0xc4, 0xc1, 0x27, 0x7c, 0x15, 0x00, 0x40, 0x00, 0x00, // VHADDPD YMM10, YMM11, [rip+0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vhaddpd_ymm_self() {
    let mut emu = emu64();
    // VHADDPD YMM7, YMM7, YMM7
    let code = [
        0xc5, 0xc7, 0x7c, 0xff, // VHADDPD YMM7, YMM7, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
