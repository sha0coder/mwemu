use crate::*;

// VEXTRACTF128 - Extract 128-bit Floating-Point Value
// VINSERTF128 - Insert 128-bit Floating-Point Value
//
// VEXTRACTF128 extracts a 128-bit floating-point value from a 256-bit source
// and stores it to a 128-bit destination (XMM register or memory).
// VINSERTF128 inserts a 128-bit floating-point value into a 256-bit destination.
// The imm8 parameter specifies which 128-bit lane to extract/insert.
//
// Opcodes:
// VEX.256.66 0F 3A 19 /r ib    VEXTRACTF128 xmm1/m128, ymm2, imm8   - Extract 128-bit float
// VEX.256.66 0F 3A 18 /r ib    VINSERTF128 ymm1, ymm2, xmm3/m128, imm8 - Insert 128-bit float

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VEXTRACTF128 Tests - Extract 128-bit Float (YMM -> XMM/Memory)
// ============================================================================

#[test]
fn test_vextractf128_ymm0_xmm1_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM1, YMM0, 0 (extract lower 128 bits)
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc1, 0x00, // VEXTRACTF128 XMM1, YMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm0_xmm1_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM1, YMM0, 1 (extract upper 128 bits)
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc1, 0x01, // VEXTRACTF128 XMM1, YMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm1_xmm2_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM2, YMM1, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xca, 0x00, // VEXTRACTF128 XMM2, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm1_xmm2_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM2, YMM1, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xca, 0x01, // VEXTRACTF128 XMM2, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm2_xmm3_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM3, YMM2, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xd3, 0x00, // VEXTRACTF128 XMM3, YMM2, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm3_xmm4_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM4, YMM3, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xdc, 0x01, // VEXTRACTF128 XMM4, YMM3, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm4_xmm5_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM5, YMM4, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xe5, 0x00, // VEXTRACTF128 XMM5, YMM4, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm5_xmm6_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM6, YMM5, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xee, 0x01, // VEXTRACTF128 XMM6, YMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm6_xmm7_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM7, YMM6, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xf7, 0x00, // VEXTRACTF128 XMM7, YMM6, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm7_xmm0_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM0, YMM7, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xf8, 0x01, // VEXTRACTF128 XMM0, YMM7, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm8_xmm9_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM9, YMM8, 0
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xc1, 0x00, // VEXTRACTF128 XMM9, YMM8, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm12_xmm13_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM13, YMM12, 1
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xe5, 0x01, // VEXTRACTF128 XMM13, YMM12, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VEXTRACTF128 Memory Tests
// ============================================================================

#[test]
fn test_vextractf128_ymm0_mem_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM0, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0x05, 0x00, 0x40, 0x00, 0x00, 0x00, // VEXTRACTF128 [rip + 0x4000], YMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm1_mem_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM1, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0x0d, 0x00, 0x40, 0x00, 0x00, 0x01, // VEXTRACTF128 [rip + 0x4000], YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm8_mem_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM8, 0
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0x05, 0x00, 0x40, 0x00, 0x00, 0x00, // VEXTRACTF128 [rip + 0x4000], YMM8, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm15_mem_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM15, 1
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0x3d, 0x00, 0x40, 0x00, 0x00, 0x01, // VEXTRACTF128 [rip + 0x4000], YMM15, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

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
fn test_vinsertf128_ymm2_ymm3_mem_lane1() {
    let mut emu = emu64();
    // VINSERTF128 YMM2, YMM3, [mem128], 1
    let code = [
        0xc4, 0xe3, 0x65, 0x18, 0x15, 0x00, 0x40, 0x00, 0x00, 0x01, // VINSERTF128 YMM2, YMM3, [rip + 0x4000], 1
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
fn test_vinsertf128_ymm8_ymm9_mem_lane0() {
    let mut emu = emu64();
    // VINSERTF128 YMM8, YMM9, [mem128], 0
    let code = [
        0xc4, 0xc3, 0x35, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, 0x00, // VINSERTF128 YMM8, YMM9, [rip + 0x4000], 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 16] = [0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa];
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

    let test_data: [u8; 16] = [0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb, 0xbb];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// Combined Extract-Insert Tests
// ============================================================================

#[test]
fn test_vextractf128_then_insert() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc1, 0x00, // VEXTRACTF128 XMM1, YMM0, 0
        0xc4, 0xe3, 0x75, 0x18, 0xc1, 0x00, // VINSERTF128 YMM0, YMM1, XMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_lane_swap() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc1, 0x01, // VEXTRACTF128 XMM1, YMM0, 1 (extract upper)
        0xc4, 0xe3, 0x75, 0x18, 0xc1, 0x00, // VINSERTF128 YMM0, YMM1, XMM1, 0 (insert lower)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinsertf128_sequence() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x18, 0xc1, 0x00, // VINSERTF128 YMM0, YMM1, XMM1, 0 (lower)
        0xc4, 0xe3, 0x75, 0x18, 0xc2, 0x01, // VINSERTF128 YMM0, YMM1, XMM2, 1 (upper)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
