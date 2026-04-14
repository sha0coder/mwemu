use crate::*;

// VINSERTF128 - Insert 128-bit Floating-Point Value
//
// VINSERTF128 inserts a 128-bit floating-point value into a 256-bit destination.
// The imm8 parameter specifies which 128-bit lane to insert into (0 = low, 1 = high).
// The other lane from the first source operand is preserved.
//
// Opcodes:
// VEX.256.66 0F 3A 18 /r ib    VINSERTF128 ymm1, ymm2, xmm3/m128, imm8 - Insert 128-bit float

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VINSERTF128 Tests - Insert 128-bit Float (YMM, XMM -> YMM)
// ============================================================================

#[test]
fn test_vinsertf128_ymm0_ymm1_xmm2_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM0, YMM1, XMM2, 0 (insert into lower 128 bits)
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0xc2, 0x00, // VINSERTF128 YMM0, YMM1, XMM2, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm0_ymm1_xmm2_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM0, YMM1, XMM2, 1 (insert into upper 128 bits)
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0xc2, 0x01, // VINSERTF128 YMM0, YMM1, XMM2, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm1_ymm2_xmm3_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM1, YMM2, XMM3, 0
    let code = [
        0xc4, 0xe3, 0x6d, 0x18, 0xcb, 0x00, // VINSERTF128 YMM1, YMM2, XMM3, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm1_ymm2_xmm3_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM1, YMM2, XMM3, 1
    let code = [
        0xc4, 0xe3, 0x6d, 0x18, 0xcb, 0x01, // VINSERTF128 YMM1, YMM2, XMM3, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm2_ymm3_xmm4_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM2, YMM3, XMM4, 0
    let code = [
        0xc4, 0xe3, 0x65, 0x18, 0xd4, 0x00, // VINSERTF128 YMM2, YMM3, XMM4, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm2_ymm3_xmm4_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM2, YMM3, XMM4, 1
    let code = [
        0xc4, 0xe3, 0x65, 0x18, 0xd4, 0x01, // VINSERTF128 YMM2, YMM3, XMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm3_ymm4_xmm5_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM3, YMM4, XMM5, 0
    let code = [
        0xc4, 0xe3, 0x5d, 0x18, 0xdd, 0x00, // VINSERTF128 YMM3, YMM4, XMM5, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm3_ymm4_xmm5_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM3, YMM4, XMM5, 1
    let code = [
        0xc4, 0xe3, 0x5d, 0x18, 0xdd, 0x01, // VINSERTF128 YMM3, YMM4, XMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm4_ymm5_xmm6_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM4, YMM5, XMM6, 0
    let code = [
        0xc4, 0xe3, 0x55, 0x18, 0xe6, 0x00, // VINSERTF128 YMM4, YMM5, XMM6, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm4_ymm5_xmm6_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM4, YMM5, XMM6, 1
    let code = [
        0xc4, 0xe3, 0x55, 0x18, 0xe6, 0x01, // VINSERTF128 YMM4, YMM5, XMM6, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm5_ymm6_xmm7_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM5, YMM6, XMM7, 0
    let code = [
        0xc4, 0xe3, 0x4d, 0x18, 0xef, 0x00, // VINSERTF128 YMM5, YMM6, XMM7, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm5_ymm6_xmm7_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM5, YMM6, XMM7, 1
    let code = [
        0xc4, 0xe3, 0x4d, 0x18, 0xef, 0x01, // VINSERTF128 YMM5, YMM6, XMM7, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm6_ymm7_xmm0_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM6, YMM7, XMM0, 0
    let code = [
        0xc4, 0xe3, 0x45, 0x18, 0xf8, 0x00, // VINSERTF128 YMM6, YMM7, XMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm6_ymm7_xmm0_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM6, YMM7, XMM0, 1
    let code = [
        0xc4, 0xe3, 0x45, 0x18, 0xf8, 0x01, // VINSERTF128 YMM6, YMM7, XMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm7_ymm0_xmm1_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM7, YMM0, XMM1, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xf9, 0x00, // VINSERTF128 YMM7, YMM0, XMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm7_ymm0_xmm1_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM7, YMM0, XMM1, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xf9, 0x01, // VINSERTF128 YMM7, YMM0, XMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VINSERTF128 Extended Register Tests
// ============================================================================

#[test]
fn test_vinsertf128_ymm8_ymm9_xmm10_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM8, YMM9, XMM10, 0
    let code = [
        0xc4, 0xc3, 0x35, 0x18, 0xc2, 0x00, // VINSERTF128 YMM8, YMM9, XMM10, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm8_ymm9_xmm10_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM8, YMM9, XMM10, 1
    let code = [
        0xc4, 0xc3, 0x35, 0x18, 0xc2, 0x01, // VINSERTF128 YMM8, YMM9, XMM10, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm9_ymm10_xmm11_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM9, YMM10, XMM11, 0
    let code = [
        0xc4, 0xc3, 0x2d, 0x18, 0xcb, 0x00, // VINSERTF128 YMM9, YMM10, XMM11, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm10_ymm11_xmm12_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM10, YMM11, XMM12, 1
    let code = [
        0xc4, 0xc3, 0x25, 0x18, 0xd4, 0x01, // VINSERTF128 YMM10, YMM11, XMM12, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm11_ymm12_xmm13_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM11, YMM12, XMM13, 0
    let code = [
        0xc4, 0xc3, 0x1d, 0x18, 0xdd, 0x00, // VINSERTF128 YMM11, YMM12, XMM13, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm12_ymm13_xmm14_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM12, YMM13, XMM14, 1
    let code = [
        0xc4, 0xc3, 0x15, 0x18, 0xe6, 0x01, // VINSERTF128 YMM12, YMM13, XMM14, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm13_ymm14_xmm15_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM13, YMM14, XMM15, 0
    let code = [
        0xc4, 0xc3, 0x0d, 0x18, 0xef, 0x00, // VINSERTF128 YMM13, YMM14, XMM15, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm14_ymm15_xmm0_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM14, YMM15, XMM0, 1
    let code = [
        0xc4, 0xe3, 0x05, 0x18, 0xf0, 0x01, // VINSERTF128 YMM14, YMM15, XMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm15_ymm0_xmm1_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM15, YMM0, XMM1, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xf9, 0x00, // VINSERTF128 YMM15, YMM0, XMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm15_ymm1_xmm2_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM15, YMM1, XMM2, 1
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0xfa, 0x01, // VINSERTF128 YMM15, YMM1, XMM2, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VINSERTF128 Memory Tests
// ============================================================================

#[test]
fn test_vinsertf128_ymm0_ymm1_mem_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM0, YMM1, [mem128], 0
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, 0x00, // VINSERTF128 YMM0, YMM1, [rip + 0x4000], 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0x80, 0x3f,
        0x00, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x40, 0x40,
        0x00, 0x00, 0x80, 0x40,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm0_ymm1_mem_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM0, YMM1, [mem128], 1
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, 0x01, // VINSERTF128 YMM0, YMM1, [rip + 0x4000], 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [
        0x00, 0x00, 0xa0, 0x40,
        0x00, 0x00, 0xc0, 0x40,
        0x00, 0x00, 0xe0, 0x40,
        0x00, 0x00, 0x00, 0x41,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm2_ymm3_mem_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM2, YMM3, [mem128], 0
    let code = [
        0xc4, 0xe3, 0x65, 0x18, 0x15, 0x00, 0x40, 0x00, 0x00, 0x00, // VINSERTF128 YMM2, YMM3, [rip + 0x4000], 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm2_ymm3_mem_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM2, YMM3, [mem128], 1
    let code = [
        0xc4, 0xe3, 0x65, 0x18, 0x15, 0x00, 0x40, 0x00, 0x00, 0x01, // VINSERTF128 YMM2, YMM3, [rip + 0x4000], 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm7_ymm6_mem_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM7, YMM6, [mem128], 0
    let code = [
        0xc4, 0xe3, 0x4d, 0x18, 0x3d, 0x00, 0x40, 0x00, 0x00, 0x00, // VINSERTF128 YMM7, YMM6, [rip + 0x4000], 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm7_ymm6_mem_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM7, YMM6, [mem128], 1
    let code = [
        0xc4, 0xe3, 0x4d, 0x18, 0x3d, 0x00, 0x40, 0x00, 0x00, 0x01, // VINSERTF128 YMM7, YMM6, [rip + 0x4000], 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm8_ymm9_mem_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM8, YMM9, [mem128], 0
    let code = [
        0xc4, 0xc3, 0x35, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, 0x00, // VINSERTF128 YMM8, YMM9, [rip + 0x4000], 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm14_ymm15_mem_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM14, YMM15, [mem128], 1
    let code = [
        0xc4, 0xc3, 0x05, 0x18, 0x35, 0x00, 0x40, 0x00, 0x00, 0x01, // VINSERTF128 YMM14, YMM15, [rip + 0x4000], 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VINSERTF128 Self-Insert Tests
// ============================================================================

#[test]
fn test_vinsertf128_ymm0_ymm0_xmm0_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM0, YMM0, XMM0, 0 - insert low lane from self
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xc0, 0x00, // VINSERTF128 YMM0, YMM0, XMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm0_ymm0_xmm0_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM0, YMM0, XMM0, 1 - insert high lane from self
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xc0, 0x01, // VINSERTF128 YMM0, YMM0, XMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm5_ymm5_xmm5_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM5, YMM5, XMM5, 0
    let code = [
        0xc4, 0xe3, 0x55, 0x18, 0xed, 0x00, // VINSERTF128 YMM5, YMM5, XMM5, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_ymm5_ymm5_xmm5_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM5, YMM5, XMM5, 1
    let code = [
        0xc4, 0xe3, 0x55, 0x18, 0xed, 0x01, // VINSERTF128 YMM5, YMM5, XMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VINSERTF128 Combined Extract-Insert Pattern Tests
// ============================================================================

#[test]
fn test_vinsertf128_sequence_both_lanes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0xc1, 0x00, // VINSERTF128 YMM0, YMM1, XMM1, 0 (lower)
        0xc4, 0xe3, 0x75, 0x18, 0xc2, 0x01, // VINSERTF128 YMM0, YMM1, XMM2, 1 (upper)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_lane_swap() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x18, 0xc0, 0x01, // VINSERTF128 YMM0, YMM0, XMM0, 1 - insert lower to upper
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
