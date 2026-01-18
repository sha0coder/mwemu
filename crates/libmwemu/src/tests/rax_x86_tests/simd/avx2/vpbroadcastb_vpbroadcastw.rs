use crate::*;

// VPBROADCASTB/VPBROADCASTW - Broadcast Byte/Word (AVX2)
//
// Broadcasts a single byte or word value from source to all elements of destination.
// Can broadcast from XMM register or memory.
//
// VPBROADCASTB: Broadcast a single byte to all 32 bytes in YMM
// VPBROADCASTW: Broadcast a single word to all 16 words in YMM
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.W0 78 /r       VPBROADCASTB ymm1, xmm2/m8
// VEX.256.66.0F38.W0 79 /r       VPBROADCASTW ymm1, xmm2/m16

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPBROADCASTB Tests - Broadcast Byte to 32 Bytes (256-bit)
// ============================================================================

#[test]
fn test_vpbroadcastb_ymm0_xmm1() {
    let mut emu = emu64();
    // VPBROADCASTB YMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x78, 0xc1, // VPBROADCASTB YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x78, 0xdc, // VPBROADCASTB YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x78, 0xf7, // VPBROADCASTB YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x78, 0xca, // VPBROADCASTB YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x78, 0xe5, // VPBROADCASTB YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x78, 0xf8, // VPBROADCASTB YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm0_mem() {
    let mut emu = emu64();
    // VPBROADCASTB YMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x00, // VPBROADCASTB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x18, // VPBROADCASTB YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAA, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x30, // VPBROADCASTB YMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x55, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm9_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x78, 0x08, // VPBROADCASTB YMM9, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x12, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x78, 0x20, // VPBROADCASTB YMM12, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x7F, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPBROADCASTW Tests - Broadcast Word to 16 Words (256-bit)
// ============================================================================

#[test]
fn test_vpbroadcastw_ymm0_xmm1() {
    let mut emu = emu64();
    // VPBROADCASTW YMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x79, 0xc1, // VPBROADCASTW YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x79, 0xdc, // VPBROADCASTW YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x79, 0xf7, // VPBROADCASTW YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x79, 0xca, // VPBROADCASTW YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x79, 0xe5, // VPBROADCASTW YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x79, 0xf8, // VPBROADCASTW YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm0_mem() {
    let mut emu = emu64();
    // VPBROADCASTW YMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x00, // VPBROADCASTW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x18, // VPBROADCASTW YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAA, 0xAA, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x30, // VPBROADCASTW YMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x55, 0x55, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm9_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x79, 0x08, // VPBROADCASTW YMM9, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x12, 0x34, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x79, 0x20, // VPBROADCASTW YMM12, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x7F, 0xFF, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vpbroadcastb_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x78, 0xc1, // VPBROADCASTB YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x78, 0xd3, // VPBROADCASTB YMM2, XMM3
        0xc4, 0xe2, 0x7d, 0x78, 0xe5, // VPBROADCASTB YMM4, XMM5
        0xc4, 0xe2, 0x7d, 0x78, 0xf7, // VPBROADCASTB YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x79, 0xc1, // VPBROADCASTW YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x79, 0xd3, // VPBROADCASTW YMM2, XMM3
        0xc4, 0xe2, 0x7d, 0x79, 0xe5, // VPBROADCASTW YMM4, XMM5
        0xc4, 0xe2, 0x7d, 0x79, 0xf7, // VPBROADCASTW YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x78, 0xc1, // VPBROADCASTB YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x78, 0xd3, // VPBROADCASTB YMM10, XMM11
        0xc4, 0x42, 0x7d, 0x78, 0xe5, // VPBROADCASTB YMM12, XMM13
        0xc4, 0x42, 0x7d, 0x78, 0xf7, // VPBROADCASTB YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x79, 0xc1, // VPBROADCASTW YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x79, 0xd3, // VPBROADCASTW YMM10, XMM11
        0xc4, 0x42, 0x7d, 0x79, 0xe5, // VPBROADCASTW YMM12, XMM13
        0xc4, 0x42, 0x7d, 0x79, 0xf7, // VPBROADCASTW YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_zero_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x00, // VPBROADCASTB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_zero_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x00, // VPBROADCASTW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_max_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x00, // VPBROADCASTB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_max_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x00, // VPBROADCASTW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_chain() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x00, // VPBROADCASTB YMM0, [RAX]
        0xc4, 0xe2, 0x7d, 0x78, 0x10, // VPBROADCASTB YMM2, [RAX]
        0xc4, 0xe2, 0x7d, 0x78, 0x20, // VPBROADCASTB YMM4, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAB, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_chain() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x00, // VPBROADCASTW YMM0, [RAX]
        0xc4, 0xe2, 0x7d, 0x79, 0x10, // VPBROADCASTW YMM2, [RAX]
        0xc4, 0xe2, 0x7d, 0x79, 0x20, // VPBROADCASTW YMM4, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAB, 0xCD, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_signed_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x00, // VPBROADCASTB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x80, 0x00, 0x00, 0x00]; // -128 as signed byte
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_signed_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x00, // VPBROADCASTW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x80, 0x00, 0x00]; // -32768 as signed word
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_mem_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x40, 0x10, // VPBROADCASTB YMM0, [RAX+16]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    data[16] = 0xBE;
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_mem_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x40, 0x10, // VPBROADCASTW YMM0, [RAX+16]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    data[16] = 0xBE;
    data[17] = 0xEF;
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastb_various_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x78, 0x00, // VPBROADCASTB YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x5A, 0x00, 0x00, 0x00]; // 01011010 pattern
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastw_various_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x79, 0x00, // VPBROADCASTW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x5A, 0xA5, 0x00, 0x00]; // 0101101010100101 pattern
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
