use crate::*;

// VBROADCASTSS - Broadcast Single-Precision Floating-Point Value
// VBROADCASTSD - Broadcast Double-Precision Floating-Point Value
//
// VBROADCASTSS loads a single-precision floating-point value from a 32-bit source
// and replicates it to all 8 elements in the 256-bit destination (YMM register).
// VBROADCASTSD loads a double-precision floating-point value from a 64-bit source
// and replicates it to all 4 elements in the 256-bit destination (YMM register).
//
// Opcodes:
// VEX.256.66 0F 38 18 /r    VBROADCASTSS ymm1, xmm2/m32   - Broadcast 32-bit float
// VEX.256.66 0F 38 19 /r    VBROADCASTSD ymm1, xmm2/m64   - Broadcast 64-bit double

const ALIGNED_ADDR: u64 = 0x3000; // 32-byte aligned address for testing

// ============================================================================
// VBROADCASTSS Tests - Broadcast Single Precision (32-bit -> 256-bit)
// ============================================================================

#[test]
fn test_vbroadcastss_xmm0_to_ymm1() {
    let mut emu = emu64();
    // VBROADCASTSS YMM1, XMM0
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xc8, // VBROADCASTSS YMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm1_to_ymm2() {
    let mut emu = emu64();
    // VBROADCASTSS YMM2, XMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xd1, // VBROADCASTSS YMM2, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm2_to_ymm3() {
    let mut emu = emu64();
    // VBROADCASTSS YMM3, XMM2
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xda, // VBROADCASTSS YMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm3_to_ymm4() {
    let mut emu = emu64();
    // VBROADCASTSS YMM4, XMM3
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xe3, // VBROADCASTSS YMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm4_to_ymm5() {
    let mut emu = emu64();
    // VBROADCASTSS YMM5, XMM4
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xec, // VBROADCASTSS YMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm5_to_ymm6() {
    let mut emu = emu64();
    // VBROADCASTSS YMM6, XMM5
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xf5, // VBROADCASTSS YMM6, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm6_to_ymm7() {
    let mut emu = emu64();
    // VBROADCASTSS YMM7, XMM6
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xfe, // VBROADCASTSS YMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm7_to_ymm0() {
    let mut emu = emu64();
    // VBROADCASTSS YMM0, XMM7
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0xc7, // VBROADCASTSS YMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm8_to_ymm9() {
    let mut emu = emu64();
    // VBROADCASTSS YMM9, XMM8
    let code = [
        0xc4, 0xc2, 0x7d, 0x18, 0xc8, // VBROADCASTSS YMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm12_to_ymm13() {
    let mut emu = emu64();
    // VBROADCASTSS YMM13, XMM12
    let code = [
        0xc4, 0xc2, 0x7d, 0x18, 0xec, // VBROADCASTSS YMM13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_xmm14_to_ymm15() {
    let mut emu = emu64();
    // VBROADCASTSS YMM15, XMM14
    let code = [
        0xc4, 0xc2, 0x7d, 0x18, 0xfe, // VBROADCASTSS YMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VBROADCASTSS Memory to Register Tests
// ============================================================================

#[test]
fn test_vbroadcastss_mem_to_ymm0() {
    let mut emu = emu64();
    // VBROADCASTSS YMM0, [mem32]
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSS YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [0x00, 0x00, 0xc0, 0x3f]; // 1.5 as float32
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_mem_to_ymm1() {
    let mut emu = emu64();
    // VBROADCASTSS YMM1, [mem32]
    let code = [
        0xc4, 0xe2, 0x7d, 0x18, 0x0d, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSS YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [0x00, 0x00, 0x00, 0x40]; // 2.0 as float32
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_mem_to_ymm8() {
    let mut emu = emu64();
    // VBROADCASTSS YMM8, [mem32]
    let code = [
        0xc4, 0xc2, 0x7d, 0x18, 0x05, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSS YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [0x00, 0x00, 0x80, 0x3f]; // 1.0 as float32
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastss_mem_to_ymm15() {
    let mut emu = emu64();
    // VBROADCASTSS YMM15, [mem32]
    let code = [
        0xc4, 0xc2, 0x7d, 0x18, 0x3d, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSS YMM15, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 4] = [0x00, 0x00, 0x00, 0xc0]; // -2.0 as float32
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

// ============================================================================
// VBROADCASTSD Tests - Broadcast Double Precision (64-bit -> 256-bit)
// ============================================================================

#[test]
fn test_vbroadcastsd_xmm0_to_ymm1() {
    let mut emu = emu64();
    // VBROADCASTSD YMM1, XMM0
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xc8, // VBROADCASTSD YMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm1_to_ymm2() {
    let mut emu = emu64();
    // VBROADCASTSD YMM2, XMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xd1, // VBROADCASTSD YMM2, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm2_to_ymm3() {
    let mut emu = emu64();
    // VBROADCASTSD YMM3, XMM2
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xda, // VBROADCASTSD YMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm3_to_ymm4() {
    let mut emu = emu64();
    // VBROADCASTSD YMM4, XMM3
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xe3, // VBROADCASTSD YMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm4_to_ymm5() {
    let mut emu = emu64();
    // VBROADCASTSD YMM5, XMM4
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xec, // VBROADCASTSD YMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm5_to_ymm6() {
    let mut emu = emu64();
    // VBROADCASTSD YMM6, XMM5
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xf5, // VBROADCASTSD YMM6, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm6_to_ymm7() {
    let mut emu = emu64();
    // VBROADCASTSD YMM7, XMM6
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xfe, // VBROADCASTSD YMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm7_to_ymm0() {
    let mut emu = emu64();
    // VBROADCASTSD YMM0, XMM7
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0xc7, // VBROADCASTSD YMM0, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm8_to_ymm9() {
    let mut emu = emu64();
    // VBROADCASTSD YMM9, XMM8
    let code = [
        0xc4, 0xc2, 0x7d, 0x19, 0xc8, // VBROADCASTSD YMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm12_to_ymm13() {
    let mut emu = emu64();
    // VBROADCASTSD YMM13, XMM12
    let code = [
        0xc4, 0xc2, 0x7d, 0x19, 0xec, // VBROADCASTSD YMM13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_xmm14_to_ymm15() {
    let mut emu = emu64();
    // VBROADCASTSD YMM15, XMM14
    let code = [
        0xc4, 0xc2, 0x7d, 0x19, 0xfe, // VBROADCASTSD YMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VBROADCASTSD Memory to Register Tests
// ============================================================================

#[test]
fn test_vbroadcastsd_mem_to_ymm0() {
    let mut emu = emu64();
    // VBROADCASTSD YMM0, [mem64]
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0x05, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSD YMM0, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    let test_data: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x3f];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_mem_to_ymm1() {
    let mut emu = emu64();
    // VBROADCASTSD YMM1, [mem64]
    let code = [
        0xc4, 0xe2, 0x7d, 0x19, 0x0d, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSD YMM1, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // 2.0 as float64
    let test_data: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_mem_to_ymm8() {
    let mut emu = emu64();
    // VBROADCASTSD YMM8, [mem64]
    let code = [
        0xc4, 0xc2, 0x7d, 0x19, 0x05, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSD YMM8, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // 1.0 as float64
    let test_data: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}

#[test]
fn test_vbroadcastsd_mem_to_ymm15() {
    let mut emu = emu64();
    // VBROADCASTSD YMM15, [mem64]
    let code = [
        0xc4, 0xc2, 0x7d, 0x19, 0x3d, 0x00, 0x40, 0x00, 0x00, // VBROADCASTSD YMM15, [rip + 0x4000]
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);

    // 3.0 as float64
    let test_data: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &test_data);

    emu.run(None).unwrap();
}
