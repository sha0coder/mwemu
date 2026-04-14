use crate::*;

// VEXTRACTF128 - Extract 128-bit Floating-Point Value
//
// VEXTRACTF128 extracts a 128-bit floating-point value from a 256-bit source
// and stores it to a 128-bit destination (XMM register or memory).
// The imm8 parameter specifies which 128-bit lane to extract (0 = low, 1 = high).
//
// Opcodes:
// VEX.256.66 0F 3A 19 /r ib    VEXTRACTF128 xmm1/m128, ymm2, imm8   - Extract 128-bit float

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
fn test_vextractf128_ymm2_xmm3_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM3, YMM2, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xd3, 0x01, // VEXTRACTF128 XMM3, YMM2, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm3_xmm4_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM4, YMM3, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xdc, 0x00, // VEXTRACTF128 XMM4, YMM3, 0
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
fn test_vextractf128_ymm4_xmm5_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM5, YMM4, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xe5, 0x01, // VEXTRACTF128 XMM5, YMM4, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm5_xmm6_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM6, YMM5, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xee, 0x00, // VEXTRACTF128 XMM6, YMM5, 0
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
fn test_vextractf128_ymm6_xmm7_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM7, YMM6, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xf7, 0x01, // VEXTRACTF128 XMM7, YMM6, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm7_xmm0_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM0, YMM7, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xf8, 0x00, // VEXTRACTF128 XMM0, YMM7, 0
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

// ============================================================================
// VEXTRACTF128 Extended Register Tests
// ============================================================================

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
fn test_vextractf128_ymm8_xmm9_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM9, YMM8, 1
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xc1, 0x01, // VEXTRACTF128 XMM9, YMM8, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm9_xmm10_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM10, YMM9, 0
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xca, 0x00, // VEXTRACTF128 XMM10, YMM9, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm10_xmm11_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM11, YMM10, 1
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xd3, 0x01, // VEXTRACTF128 XMM11, YMM10, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm11_xmm12_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM12, YMM11, 0
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xdc, 0x00, // VEXTRACTF128 XMM12, YMM11, 0
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

#[test]
fn test_vextractf128_ymm13_xmm14_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM14, YMM13, 0
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xee, 0x00, // VEXTRACTF128 XMM14, YMM13, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm14_xmm15_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM15, YMM14, 1
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0xf7, 0x01, // VEXTRACTF128 XMM15, YMM14, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm15_xmm0_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM0, YMM15, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xf8, 0x00, // VEXTRACTF128 XMM0, YMM15, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm15_xmm1_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM1, YMM15, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xf9, 0x01, // VEXTRACTF128 XMM1, YMM15, 1
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
fn test_vextractf128_ymm0_mem_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM0, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0x05, 0x00, 0x40, 0x00, 0x00, 0x01, // VEXTRACTF128 [rip + 0x4000], YMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm1_mem_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM1, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0x0d, 0x00, 0x40, 0x00, 0x00, 0x00, // VEXTRACTF128 [rip + 0x4000], YMM1, 0
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
fn test_vextractf128_ymm7_mem_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM7, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0x3d, 0x00, 0x40, 0x00, 0x00, 0x00, // VEXTRACTF128 [rip + 0x4000], YMM7, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm7_mem_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM7, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0x3d, 0x00, 0x40, 0x00, 0x00, 0x01, // VEXTRACTF128 [rip + 0x4000], YMM7, 1
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
fn test_vextractf128_ymm8_mem_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM8, 1
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0x05, 0x00, 0x40, 0x00, 0x00, 0x01, // VEXTRACTF128 [rip + 0x4000], YMM8, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm15_mem_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 [mem128], YMM15, 0
    let code = [
        0xc4, 0xc3, 0x7d, 0x19, 0x3d, 0x00, 0x40, 0x00, 0x00, 0x00, // VEXTRACTF128 [rip + 0x4000], YMM15, 0
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
// VEXTRACTF128 Self-Extract Tests
// ============================================================================

#[test]
fn test_vextractf128_ymm0_xmm0_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM0, YMM0, 0 - extract to self (low lane)
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc0, 0x00, // VEXTRACTF128 XMM0, YMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm0_xmm0_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM0, YMM0, 1 - extract to self (high lane)
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xc0, 0x01, // VEXTRACTF128 XMM0, YMM0, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm5_xmm5_lane0() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM5, YMM5, 0
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xed, 0x00, // VEXTRACTF128 XMM5, YMM5, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextractf128_ymm5_xmm5_lane1() {
    let mut emu = emu64();
    // VEXTRACTF128 XMM5, YMM5, 1
    let code = [
        0xc4, 0xe3, 0x7d, 0x19, 0xed, 0x01, // VEXTRACTF128 XMM5, YMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
