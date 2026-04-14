use crate::*;

// VDPPS - Dot Product of Packed Single-Precision Floating-Point Values
//
// VDPPS computes the dot product of packed single-precision floating-point values
// from the source operands and stores the result in the destination operand.
//
// An 8-bit immediate operand controls which elements participate in the dot product
// calculation and which elements of the result are written:
//
// - Bits 7:4 control which source elements are multiplied and summed
//   - Bit 7: multiply src1[3] * src2[3]
//   - Bit 6: multiply src1[2] * src2[2]
//   - Bit 5: multiply src1[1] * src2[1]
//   - Bit 4: multiply src1[0] * src2[0]
//
// - Bits 3:0 control which destination elements receive the result
//   - Bit 3: write result to dst[3]
//   - Bit 2: write result to dst[2]
//   - Bit 1: write result to dst[1]
//   - Bit 0: write result to dst[0]
//
// Example: imm8 = 0xFF means all 4 elements participate, result goes to all 4 elements
//          imm8 = 0xF1 means all 4 elements participate, result goes to dst[0] only
//          imm8 = 0x71 means elements 0,1,2 participate, result goes to dst[0] only
//
// For 256-bit (YMM), the operation is performed independently on the lower and upper 128-bit lanes.
//
// Opcodes:
// VEX.128.66.0F3A.WIG 40 /r ib    VDPPS xmm1, xmm2, xmm3/m128, imm8
// VEX.256.66.0F3A.WIG 40 /r ib    VDPPS ymm1, ymm2, ymm3/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VDPPS Tests - 128-bit XMM registers, full dot product (imm8 = 0xFF)
// ============================================================================

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_ff() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0xFF (all elements participate, all receive result)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xff, // VDPPS XMM0, XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm1_xmm2_xmm3_ff() {
    let mut emu = emu64();
    // VDPPS XMM1, XMM2, XMM3, 0xFF
    let code = [
        0xc4, 0xe3, 0x69, 0x40, 0xcb, 0xff, // VDPPS XMM1, XMM2, XMM3, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm2_xmm3_xmm4_ff() {
    let mut emu = emu64();
    // VDPPS XMM2, XMM3, XMM4, 0xFF
    let code = [
        0xc4, 0xe3, 0x61, 0x40, 0xd4, 0xff, // VDPPS XMM2, XMM3, XMM4, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm3_xmm4_xmm5_ff() {
    let mut emu = emu64();
    // VDPPS XMM3, XMM4, XMM5, 0xFF
    let code = [
        0xc4, 0xe3, 0x59, 0x40, 0xdd, 0xff, // VDPPS XMM3, XMM4, XMM5, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm4_xmm5_xmm6_ff() {
    let mut emu = emu64();
    // VDPPS XMM4, XMM5, XMM6, 0xFF
    let code = [
        0xc4, 0xe3, 0x51, 0x40, 0xe6, 0xff, // VDPPS XMM4, XMM5, XMM6, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm5_xmm6_xmm7_ff() {
    let mut emu = emu64();
    // VDPPS XMM5, XMM6, XMM7, 0xFF
    let code = [
        0xc4, 0xe3, 0x49, 0x40, 0xef, 0xff, // VDPPS XMM5, XMM6, XMM7, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm6_xmm7_xmm0_ff() {
    let mut emu = emu64();
    // VDPPS XMM6, XMM7, XMM0, 0xFF
    let code = [
        0xc4, 0xe3, 0x41, 0x40, 0xf0, 0xff, // VDPPS XMM6, XMM7, XMM0, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm7_xmm0_xmm1_ff() {
    let mut emu = emu64();
    // VDPPS XMM7, XMM0, XMM1, 0xFF
    let code = [
        0xc4, 0xe3, 0x79, 0x40, 0xf9, 0xff, // VDPPS XMM7, XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - Extended XMM registers
// ============================================================================

#[test]
fn test_vdpps_xmm8_xmm9_xmm10_ff() {
    let mut emu = emu64();
    // VDPPS XMM8, XMM9, XMM10, 0xFF
    let code = [
        0xc4, 0x43, 0x31, 0x40, 0xc2, 0xff, // VDPPS XMM8, XMM9, XMM10, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm9_xmm10_xmm11_ff() {
    let mut emu = emu64();
    // VDPPS XMM9, XMM10, XMM11, 0xFF
    let code = [
        0xc4, 0x43, 0x29, 0x40, 0xcb, 0xff, // VDPPS XMM9, XMM10, XMM11, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm10_xmm11_xmm12_ff() {
    let mut emu = emu64();
    // VDPPS XMM10, XMM11, XMM12, 0xFF
    let code = [
        0xc4, 0x43, 0x21, 0x40, 0xd4, 0xff, // VDPPS XMM10, XMM11, XMM12, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm15_xmm14_xmm13_ff() {
    let mut emu = emu64();
    // VDPPS XMM15, XMM14, XMM13, 0xFF
    let code = [
        0xc4, 0x43, 0x09, 0x40, 0xfd, 0xff, // VDPPS XMM15, XMM14, XMM13, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - Different immediate masks
// ============================================================================

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_f1() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0xF1 (all multiply, result to element 0 only)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xf1, // VDPPS XMM0, XMM1, XMM2, 0xF1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_7f() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x7F (elements 0-2 multiply, result to all)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x7f, // VDPPS XMM0, XMM1, XMM2, 0x7F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_71() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x71 (elements 0-2 multiply, result to element 0)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x71, // VDPPS XMM0, XMM1, XMM2, 0x71
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_3f() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x3F (elements 0-1 multiply, result to all)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x3f, // VDPPS XMM0, XMM1, XMM2, 0x3F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_31() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x31 (elements 0-1 multiply, result to element 0)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x31, // VDPPS XMM0, XMM1, XMM2, 0x31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_11() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x11 (element 0 only, result to element 0)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x11, // VDPPS XMM0, XMM1, XMM2, 0x11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_f0() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0xF0 (all multiply, zero all result elements)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xf0, // VDPPS XMM0, XMM1, XMM2, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_0f() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x0F (no multiply, result to all - should be 0)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x0f, // VDPPS XMM0, XMM1, XMM2, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_88() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0x88 (element 3 only, result to element 3)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x88, // VDPPS XMM0, XMM1, XMM2, 0x88
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_cc() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0xCC (elements 2-3 multiply, result to elements 2-3)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xcc, // VDPPS XMM0, XMM1, XMM2, 0xCC
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm0_xmm1_xmm2_aa() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, XMM2, 0xAA (elements 1,3 multiply, result to elements 1,3)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xaa, // VDPPS XMM0, XMM1, XMM2, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - Memory operands (128-bit)
// ============================================================================

#[test]
fn test_vdpps_xmm0_xmm1_mem_ff() {
    let mut emu = emu64();
    // VDPPS XMM0, XMM1, [mem], 0xFF
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0x05, 0x00, 0x40, 0x00, 0x00, 0xff, // VDPPS XMM0, XMM1, [rip+0x4000], 0xFF
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
fn test_vdpps_xmm1_xmm2_mem_f1() {
    let mut emu = emu64();
    // VDPPS XMM1, XMM2, [mem], 0xF1
    let code = [
        0xc4, 0xe3, 0x69, 0x40, 0x0d, 0x00, 0x40, 0x00, 0x00, 0xf1, // VDPPS XMM1, XMM2, [rip+0x4000], 0xF1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x00, 0x3f, // 0.5
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_xmm8_xmm9_mem_ff() {
    let mut emu = emu64();
    // VDPPS XMM8, XMM9, [mem], 0xFF
    let code = [
        0xc4, 0x63, 0x31, 0x40, 0x05, 0x00, 0x40, 0x00, 0x00, 0xff, // VDPPS XMM8, XMM9, [rip+0x4000], 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x80, 0x3f,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - 256-bit YMM registers
// ============================================================================

#[test]
fn test_vdpps_ymm0_ymm1_ymm2_ff() {
    let mut emu = emu64();
    // VDPPS YMM0, YMM1, YMM2, 0xFF (operates on both 128-bit lanes independently)
    let code = [
        0xc4, 0xe3, 0x75, 0x40, 0xc2, 0xff, // VDPPS YMM0, YMM1, YMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm1_ymm2_ymm3_ff() {
    let mut emu = emu64();
    // VDPPS YMM1, YMM2, YMM3, 0xFF
    let code = [
        0xc4, 0xe3, 0x6d, 0x40, 0xcb, 0xff, // VDPPS YMM1, YMM2, YMM3, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm2_ymm3_ymm4_ff() {
    let mut emu = emu64();
    // VDPPS YMM2, YMM3, YMM4, 0xFF
    let code = [
        0xc4, 0xe3, 0x65, 0x40, 0xd4, 0xff, // VDPPS YMM2, YMM3, YMM4, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm3_ymm4_ymm5_ff() {
    let mut emu = emu64();
    // VDPPS YMM3, YMM4, YMM5, 0xFF
    let code = [
        0xc4, 0xe3, 0x5d, 0x40, 0xdd, 0xff, // VDPPS YMM3, YMM4, YMM5, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm4_ymm5_ymm6_ff() {
    let mut emu = emu64();
    // VDPPS YMM4, YMM5, YMM6, 0xFF
    let code = [
        0xc4, 0xe3, 0x55, 0x40, 0xe6, 0xff, // VDPPS YMM4, YMM5, YMM6, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm8_ymm9_ymm10_ff() {
    let mut emu = emu64();
    // VDPPS YMM8, YMM9, YMM10, 0xFF
    let code = [
        0xc4, 0x43, 0x35, 0x40, 0xc2, 0xff, // VDPPS YMM8, YMM9, YMM10, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm15_ymm14_ymm13_ff() {
    let mut emu = emu64();
    // VDPPS YMM15, YMM14, YMM13, 0xFF
    let code = [
        0xc4, 0x43, 0x0d, 0x40, 0xfd, 0xff, // VDPPS YMM15, YMM14, YMM13, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - YMM with different masks
// ============================================================================

#[test]
fn test_vdpps_ymm0_ymm1_ymm2_f1() {
    let mut emu = emu64();
    // VDPPS YMM0, YMM1, YMM2, 0xF1
    let code = [
        0xc4, 0xe3, 0x75, 0x40, 0xc2, 0xf1, // VDPPS YMM0, YMM1, YMM2, 0xF1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm0_ymm1_ymm2_7f() {
    let mut emu = emu64();
    // VDPPS YMM0, YMM1, YMM2, 0x7F
    let code = [
        0xc4, 0xe3, 0x75, 0x40, 0xc2, 0x7f, // VDPPS YMM0, YMM1, YMM2, 0x7F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm0_ymm1_ymm2_31() {
    let mut emu = emu64();
    // VDPPS YMM0, YMM1, YMM2, 0x31
    let code = [
        0xc4, 0xe3, 0x75, 0x40, 0xc2, 0x31, // VDPPS YMM0, YMM1, YMM2, 0x31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - Memory operands (256-bit)
// ============================================================================

#[test]
fn test_vdpps_ymm0_ymm1_mem_ff() {
    let mut emu = emu64();
    // VDPPS YMM0, YMM1, [mem], 0xFF
    let code = [
        0xc4, 0xe3, 0x75, 0x40, 0x05, 0x00, 0x40, 0x00, 0x00, 0xff, // VDPPS YMM0, YMM1, [rip+0x4000], 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
        0x00, 0x00, 0x80, 0x3f, // 1.0
        0x00, 0x00, 0x00, 0x40, // 2.0
        0x00, 0x00, 0x40, 0x40, // 3.0
        0x00, 0x00, 0x80, 0x40, // 4.0
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_ymm8_ymm9_mem_f1() {
    let mut emu = emu64();
    // VDPPS YMM8, YMM9, [mem], 0xF1
    let code = [
        0xc4, 0x63, 0x35, 0x40, 0x05, 0x00, 0x40, 0x00, 0x00, 0xf1, // VDPPS YMM8, YMM9, [rip+0x4000], 0xF1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 32] = [
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x80, 0x3f,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VDPPS Tests - Special patterns and use cases
// ============================================================================

#[test]
fn test_vdpps_3d_dot_product() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x71, // VDPPS XMM0, XMM1, XMM2, 0x71
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_2d_dot_product() {
    let mut emu = emu64();
    // 2D dot product pattern (0x31)
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x31, // VDPPS XMM0, XMM1, XMM2, 0x31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_broadcast_result() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xff, // VDPPS XMM0, XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_multiple_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xff, // VDPPS XMM0, XMM1, XMM2, 0xFF
        0xc4, 0xe3, 0x69, 0x40, 0xcc, 0xff, // VDPPS XMM1, XMM2, XMM4, 0xFF
        0xc4, 0xe3, 0x61, 0x40, 0xd5, 0xff, // VDPPS XMM2, XMM3, XMM5, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_partial_elements() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x11, // VDPPS XMM0, XMM1, XMM2, 0x11 (elem 0 only)
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x22, // VDPPS XMM0, XMM1, XMM2, 0x22 (elem 1 only)
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x44, // VDPPS XMM0, XMM1, XMM2, 0x44 (elem 2 only)
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x88, // VDPPS XMM0, XMM1, XMM2, 0x88 (elem 3 only)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vdpps_alternating_elements() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0x55, // VDPPS XMM0, XMM1, XMM2, 0x55 (0 and 2)
        0xc4, 0xe3, 0x71, 0x40, 0xc2, 0xaa, // VDPPS XMM0, XMM1, XMM2, 0xAA (1 and 3)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
