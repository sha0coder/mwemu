use crate::*;

// VEXTRACTI128 - Extract 128-bit Integer Lane (AVX2)
//
// Extracts a 128-bit lane from a 256-bit YMM register and stores it to
// a 128-bit XMM register or memory location.
//
// The immediate byte selects which 128-bit lane:
// - imm8[0] = 0: Extract bits [127:0] (lower lane)
// - imm8[0] = 1: Extract bits [255:128] (upper lane)
//
// Only bit 0 of the immediate is used; other bits are ignored.
//
// Opcode: VEX.256.66.0F3A.W0 39 /r ib    VEXTRACTI128 xmm1/m128, ymm2, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests extracting lower lane (imm8 = 0)
// ============================================================================

#[test]
fn test_vextracti128_xmm0_ymm1_lower() {
    let mut emu = emu64();
    // VEXTRACTI128 XMM0, YMM1, 0 (extract lower 128 bits)
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0x00, // VEXTRACTI128 XMM0, YMM1, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm2_ymm3_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xda, 0x00, // VEXTRACTI128 XMM2, YMM3, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm4_ymm5_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xec, 0x00, // VEXTRACTI128 XMM4, YMM5, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm6_ymm7_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xfe, 0x00, // VEXTRACTI128 XMM6, YMM7, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests extracting upper lane (imm8 = 1)
// ============================================================================

#[test]
fn test_vextracti128_xmm0_ymm1_upper() {
    let mut emu = emu64();
    // VEXTRACTI128 XMM0, YMM1, 1 (extract upper 128 bits)
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0x01, // VEXTRACTI128 XMM0, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm2_ymm3_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xda, 0x01, // VEXTRACTI128 XMM2, YMM3, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm4_ymm5_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xec, 0x01, // VEXTRACTI128 XMM4, YMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm6_ymm7_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xfe, 0x01, // VEXTRACTI128 XMM6, YMM7, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers (XMM8-XMM15, YMM8-YMM15)
// ============================================================================

#[test]
fn test_vextracti128_xmm8_ymm9_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xc8, 0x00, // VEXTRACTI128 XMM8, YMM9, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm10_ymm11_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xda, 0x01, // VEXTRACTI128 XMM10, YMM11, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm12_ymm13_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xec, 0x00, // VEXTRACTI128 XMM12, YMM13, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm14_ymm15_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xfe, 0x01, // VEXTRACTI128 XMM14, YMM15, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm0_ymm8_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc3, 0x7d, 0x39, 0xc0, 0x00, // VEXTRACTI128 XMM0, YMM8, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm0_ymm15_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc3, 0x7d, 0x39, 0xf8, 0x01, // VEXTRACTI128 XMM0, YMM15, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm15_ymm0_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0x7d, 0x39, 0xc7, 0x00, // VEXTRACTI128 XMM15, YMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm15_ymm8_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xc7, 0x01, // VEXTRACTI128 XMM15, YMM8, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory destination - lower lane
// ============================================================================

#[test]
fn test_vextracti128_mem_ymm1_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x08, 0x00, // VEXTRACTI128 [RAX], YMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_mem_ymm2_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x10, 0x00, // VEXTRACTI128 [RAX], YMM2, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_mem_ymm3_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x18, 0x00, // VEXTRACTI128 [RAX], YMM3, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with memory destination - upper lane
// ============================================================================

#[test]
fn test_vextracti128_mem_ymm1_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x08, 0x01, // VEXTRACTI128 [RAX], YMM1, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_mem_ymm4_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x20, 0x01, // VEXTRACTI128 [RAX], YMM4, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_mem_ymm7_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x38, 0x01, // VEXTRACTI128 [RAX], YMM7, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with extended registers and memory
// ============================================================================

#[test]
fn test_vextracti128_mem_ymm8_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0x7d, 0x39, 0x00, 0x00, // VEXTRACTI128 [RAX], YMM8, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_mem_ymm15_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0x7d, 0x39, 0x38, 0x01, // VEXTRACTI128 [RAX], YMM15, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high immediate bits ignored
// ============================================================================

#[test]
fn test_vextracti128_xmm0_ymm1_imm_0x02() {
    let mut emu = emu64();
    // imm8 = 0x02 (bit 0 = 0, so lower lane)
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0x02, // VEXTRACTI128 XMM0, YMM1, 0x02
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm0_ymm1_imm_0x03() {
    let mut emu = emu64();
    // imm8 = 0x03 (bit 0 = 1, so upper lane)
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0x03, // VEXTRACTI128 XMM0, YMM1, 0x03
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm0_ymm1_imm_0xFF() {
    let mut emu = emu64();
    // imm8 = 0xFF (bit 0 = 1, so upper lane)
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0xff, // VEXTRACTI128 XMM0, YMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm0_ymm1_imm_0xFE() {
    let mut emu = emu64();
    // imm8 = 0xFE (bit 0 = 0, so lower lane)
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0xfe, // VEXTRACTI128 XMM0, YMM1, 0xFE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Chained operations
// ============================================================================

#[test]
fn test_vextracti128_chain_both_lanes() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0x00, // VEXTRACTI128 XMM0, YMM1, 0
        0xc4, 0xe3, 0x7d, 0x39, 0xd1, 0x01, // VEXTRACTI128 XMM2, YMM1, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_sequential_extracts() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc8, 0x00, // VEXTRACTI128 XMM0, YMM1, 0
        0xc4, 0xe3, 0x7d, 0x39, 0xda, 0x00, // VEXTRACTI128 XMM2, YMM3, 0
        0xc4, 0xe3, 0x7d, 0x39, 0xec, 0x01, // VEXTRACTI128 XMM4, YMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Same register operations
// ============================================================================

#[test]
fn test_vextracti128_xmm0_ymm0_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xc0, 0x00, // VEXTRACTI128 XMM0, YMM0, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm5_ymm5_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0x7d, 0x39, 0xed, 0x01, // VEXTRACTI128 XMM5, YMM5, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm15_ymm15_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xff, 0x00, // VEXTRACTI128 XMM15, YMM15, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operations with offsets
// ============================================================================

#[test]
fn test_vextracti128_mem_unaligned_lower() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x08, 0x00, // VEXTRACTI128 [RAX], YMM1, 0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_mem_unaligned_upper() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0x7d, 0x39, 0x10, 0x01, // VEXTRACTI128 [RAX], YMM2, 1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

// ============================================================================
// All combinations of extended registers
// ============================================================================

#[test]
fn test_vextracti128_xmm9_ymm10_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xd1, 0x00, // VEXTRACTI128 XMM9, YMM10, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm11_ymm12_upper() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xe3, 0x01, // VEXTRACTI128 XMM11, YMM12, 1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vextracti128_xmm13_ymm14_lower() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0x7d, 0x39, 0xf5, 0x00, // VEXTRACTI128 XMM13, YMM14, 0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
