use crate::*;

// PCMPEQB/PCMPEQW/PCMPEQD - Compare Packed Data for Equal (SSE2)
//
// Performs SIMD compare for equality of packed integers in destination and source operands.
// If a pair of data elements is equal, the corresponding data element in destination is set
// to all 1s; otherwise, it is set to all 0s.
//
// PCMPEQB: Compare 16 packed bytes for equality (sets 0xFF or 0x00 for each byte)
// PCMPEQW: Compare 8 packed words for equality (sets 0xFFFF or 0x0000 for each word)
// PCMPEQD: Compare 4 packed dwords for equality (sets 0xFFFFFFFF or 0x00000000 for each dword)
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F 74 /r      PCMPEQB xmm1, xmm2/m128   - Compare packed bytes for equality
// 66 0F 75 /r      PCMPEQW xmm1, xmm2/m128   - Compare packed words for equality
// 66 0F 76 /r      PCMPEQD xmm1, xmm2/m128   - Compare packed dwords for equality

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PCMPEQB Tests - 16x Byte Comparison
// ============================================================================

#[test]
fn test_pcmpeqb_all_equal() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x74, 0xc1, // PCMPEQB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_all_different() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x74, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x74, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    let data2 = [0x01, 0xFF, 0x03, 0xFF, 0x05, 0xFF, 0x07, 0xFF,
                 0x09, 0xFF, 0x0B, 0xFF, 0x0D, 0xFF, 0x0F, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x74, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x74, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x1b, // MOVDQA XMM3, [RBX]
        0x66, 0x0f, 0x74, 0xd3, // PCMPEQB XMM2, XMM3
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x74, 0x03, // PCMPEQB XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x74, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
                 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99];
    let data2 = [0xAA, 0x00, 0xCC, 0x00, 0xEE, 0x00, 0x00, 0x00,
                 0x22, 0x00, 0x44, 0x00, 0x66, 0x00, 0x88, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// PCMPEQW Tests - 8x Word Comparison
// ============================================================================

#[test]
fn test_pcmpeqw_all_equal() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x75, 0xc1, // PCMPEQW XMM0, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12,
                0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_all_different() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x75, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2 = [0x11, 0x00, 0x12, 0x00, 0x13, 0x00, 0x14, 0x00,
                 0x15, 0x00, 0x16, 0x00, 0x17, 0x00, 0x18, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x75, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2 = [0x01, 0x00, 0xFF, 0xFF, 0x03, 0x00, 0xFF, 0xFF,
                 0x05, 0x00, 0xFF, 0xFF, 0x07, 0x00, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x75, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x75, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x2b, // MOVDQA XMM5, [RBX]
        0x66, 0x0f, 0x75, 0xe5, // PCMPEQW XMM4, XMM5
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x75, 0x03, // PCMPEQW XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xAB, 0xCD, 0xAB, 0xCD, 0xAB, 0xCD, 0xAB, 0xCD,
                0xAB, 0xCD, 0xAB, 0xCD, 0xAB, 0xCD, 0xAB, 0xCD];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PCMPEQD Tests - 4x Dword Comparison
// ============================================================================

#[test]
fn test_pcmpeqd_all_equal() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x76, 0xc1, // PCMPEQD XMM0, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12,
                0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_all_different() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x76, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    let data2 = [0x11, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00,
                 0x13, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x76, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    let data2 = [0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                 0x03, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x76, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x76, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x6f, 0x13, // MOVDQA XMM2, [RBX]
        0x66, 0x0f, 0x76, 0xca, // PCMPEQD XMM1, XMM2
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x76, 0x03, // PCMPEQD XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xEF, 0xCD, 0xAB, 0x90, 0xEF, 0xCD, 0xAB, 0x90,
                0xEF, 0xCD, 0xAB, 0x90, 0xEF, 0xCD, 0xAB, 0x90];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x76, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x80, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x80, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pcmpeqb_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x0b, // MOVDQA XMM9, [RBX]
        0x66, 0x45, 0x0f, 0x74, 0xc1, // PCMPEQB XMM8, XMM9
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x1b, // MOVDQA XMM11, [RBX]
        0x66, 0x45, 0x0f, 0x75, 0xd3, // PCMPEQW XMM10, XMM11
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34,
                0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x2b, // MOVDQA XMM13, [RBX]
        0x66, 0x45, 0x0f, 0x76, 0xe5, // PCMPEQD XMM12, XMM13
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12,
                0x78, 0x56, 0x34, 0x12, 0x78, 0x56, 0x34, 0x12];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequence Tests
// ============================================================================

#[test]
fn test_pcmpeq_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x74, 0xc1, // PCMPEQB XMM0, XMM1
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x75, 0xc1, // PCMPEQW XMM0, XMM1
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x76, 0xc1, // PCMPEQD XMM0, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42]);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqb_self_comparison() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x74, 0xc0, // PCMPEQB XMM0, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqw_self_comparison() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x75, 0xc0, // PCMPEQW XMM0, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pcmpeqd_self_comparison() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x76, 0xc0, // PCMPEQD XMM0, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
