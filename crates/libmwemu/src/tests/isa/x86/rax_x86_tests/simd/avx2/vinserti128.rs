use crate::*;

// VINSERTI128 - Insert 128-bit Integer Lane (AVX2)
//
// Inserts a 128-bit value from an XMM register or memory location into
// a 256-bit YMM register at a specified position.
//
// The immediate byte selects which 128-bit lane to replace:
// - imm8[0] = 0: Insert into bits [127:0] (lower lane)
// - imm8[0] = 1: Insert into bits [255:128] (upper lane)
//
// The other lane is copied from the first source operand.
// Only bit 0 of the immediate is used; other bits are ignored.
//
// Opcode: VEX.256.66.0F3A.W0 38 /r ib    VINSERTI128 ymm1, ymm2, xmm3/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests inserting into lower lane (imm8 = 0)
// ============================================================================

#[test]
fn test_vinserti128_ymm0_ymm1_xmm2_lower() {
    let mut emu = emu64();
    // VINSERTI128 YMM0, YMM1, XMM2, 0 (insert into lower 128 bits)
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0x00, // VINSERTI128 YMM0, YMM1, XMM2, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm3_ymm4_xmm5_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x5d, 0x38, 0xdd, 0x00, // VINSERTI128 YMM3, YMM4, XMM5, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm6_ymm7_xmm0_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x45, 0x38, 0xf0, 0x00, // VINSERTI128 YMM6, YMM7, XMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm2_ymm3_xmm4_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x65, 0x38, 0xd4, 0x00, // VINSERTI128 YMM2, YMM3, XMM4, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests inserting into upper lane (imm8 = 1)
// ============================================================================

#[test]
fn test_vinserti128_ymm0_ymm1_xmm2_upper() {
    let mut emu = emu64();
    // VINSERTI128 YMM0, YMM1, XMM2, 1 (insert into upper 128 bits)
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0x01, // VINSERTI128 YMM0, YMM1, XMM2, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm3_ymm4_xmm5_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x5d, 0x38, 0xdd, 0x01, // VINSERTI128 YMM3, YMM4, XMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm6_ymm7_xmm1_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x45, 0x38, 0xf1, 0x01, // VINSERTI128 YMM6, YMM7, XMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm5_ymm2_xmm7_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x6d, 0x38, 0xef, 0x01, // VINSERTI128 YMM5, YMM2, XMM7, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers (XMM8-XMM15, YMM8-YMM15)
// ============================================================================

#[test]
fn test_vinserti128_ymm8_ymm9_xmm10_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x35, 0x38, 0xc2, 0x00, // VINSERTI128 YMM8, YMM9, XMM10, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm11_ymm12_xmm13_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x1d, 0x38, 0xdd, 0x01, // VINSERTI128 YMM11, YMM12, XMM13, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm14_ymm15_xmm0_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x05, 0x38, 0xf0, 0x00, // VINSERTI128 YMM14, YMM15, XMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm0_ymm1_xmm15_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc3, 0x75, 0x38, 0xc7, 0x01, // VINSERTI128 YMM0, YMM1, XMM15, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm15_ymm8_xmm9_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x3d, 0x38, 0xf9, 0x00, // VINSERTI128 YMM15, YMM8, XMM9, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm10_ymm11_xmm8_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x25, 0x38, 0xd0, 0x01, // VINSERTI128 YMM10, YMM11, XMM8, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm9_ymm0_xmm14_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc3, 0x7d, 0x38, 0xce, 0x00, // VINSERTI128 YMM9, YMM0, XMM14, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm12_ymm13_xmm15_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x15, 0x38, 0xe7, 0x01, // VINSERTI128 YMM12, YMM13, XMM15, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory source - lower lane
// ============================================================================

#[test]
fn test_vinserti128_ymm0_ymm1_mem_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x38, 0x00, 0x00, // VINSERTI128 YMM0, YMM1, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm2_ymm3_mem_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x65, 0x38, 0x10, 0x00, // VINSERTI128 YMM2, YMM3, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm4_ymm5_mem_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x55, 0x38, 0x20, 0x00, // VINSERTI128 YMM4, YMM5, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory source - upper lane
// ============================================================================

#[test]
fn test_vinserti128_ymm0_ymm1_mem_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x38, 0x00, 0x01, // VINSERTI128 YMM0, YMM1, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).map(|i| i * 2).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm3_ymm4_mem_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x5d, 0x38, 0x18, 0x01, // VINSERTI128 YMM3, YMM4, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm6_ymm7_mem_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x45, 0x38, 0x30, 0x01, // VINSERTI128 YMM6, YMM7, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers and memory
// ============================================================================

#[test]
fn test_vinserti128_ymm8_ymm9_mem_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0x35, 0x38, 0x00, 0x00, // VINSERTI128 YMM8, YMM9, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm15_ymm0_mem_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0x7d, 0x38, 0x38, 0x01, // VINSERTI128 YMM15, YMM0, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high immediate bits ignored
// ============================================================================

#[test]
fn test_vinserti128_ymm0_ymm1_xmm2_imm_0x02() {
    let mut emu = emu64();
    // imm8 = 0x02 (bit 0 = 0, so lower lane)
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0x02, // VINSERTI128 YMM0, YMM1, XMM2, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm0_ymm1_xmm2_imm_0x03() {
    let mut emu = emu64();
    // imm8 = 0x03 (bit 0 = 1, so upper lane)
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0x03, // VINSERTI128 YMM0, YMM1, XMM2, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm0_ymm1_xmm2_imm_0xFF() {
    let mut emu = emu64();
    // imm8 = 0xFF (bit 0 = 1, so upper lane)
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0xff, // VINSERTI128 YMM0, YMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm0_ymm1_xmm2_imm_0xFE() {
    let mut emu = emu64();
    // imm8 = 0xFE (bit 0 = 0, so lower lane)
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0xfe, // VINSERTI128 YMM0, YMM1, XMM2, 0xFE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Chained operations
// ============================================================================

#[test]
fn test_vinserti128_chain_both_lanes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0x00, // VINSERTI128 YMM0, YMM1, XMM2, 0
        0xc4, 0xe3, 0x7d, 0x38, 0xc3, 0x01, // VINSERTI128 YMM0, YMM0, XMM3, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_sequential_inserts() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x75, 0x38, 0xc2, 0x00, // VINSERTI128 YMM0, YMM1, XMM2, 0
        0xc4, 0xe3, 0x65, 0x38, 0xdc, 0x01, // VINSERTI128 YMM3, YMM3, XMM4, 1
        0xc4, 0xe3, 0x55, 0x38, 0xee, 0x00, // VINSERTI128 YMM5, YMM5, XMM6, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Same register operations
// ============================================================================

#[test]
fn test_vinserti128_ymm0_ymm0_xmm0_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x38, 0xc0, 0x00, // VINSERTI128 YMM0, YMM0, XMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm5_ymm5_xmm5_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x55, 0x38, 0xed, 0x01, // VINSERTI128 YMM5, YMM5, XMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm15_ymm15_xmm15_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x05, 0x38, 0xff, 0x00, // VINSERTI128 YMM15, YMM15, XMM15, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operations with offsets
// ============================================================================

#[test]
fn test_vinserti128_mem_unaligned_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x38, 0x00, 0x00, // VINSERTI128 YMM0, YMM1, [RAX], 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_mem_unaligned_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x75, 0x38, 0x00, 0x01, // VINSERTI128 YMM0, YMM1, [RAX], 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vinserti128_all_combinations_1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x6d, 0x38, 0xd1, 0x00, // VINSERTI128 YMM2, YMM2, XMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_all_combinations_2() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x4d, 0x38, 0xe3, 0x01, // VINSERTI128 YMM4, YMM6, XMM3, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_build_ymm_from_xmm() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x38, 0xc1, 0x00, // VINSERTI128 YMM0, YMM0, XMM1, 0
        0xc4, 0xe3, 0x7d, 0x38, 0xc2, 0x01, // VINSERTI128 YMM0, YMM0, XMM2, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm9_ymm10_xmm11_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x2d, 0x38, 0xcb, 0x00, // VINSERTI128 YMM9, YMM10, XMM11, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vinserti128_ymm13_ymm14_xmm12_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x0d, 0x38, 0xec, 0x01, // VINSERTI128 YMM13, YMM14, XMM12, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
