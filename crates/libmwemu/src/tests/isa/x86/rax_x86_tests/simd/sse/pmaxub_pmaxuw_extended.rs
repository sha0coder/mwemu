use crate::*;

// PMAXUB/PMAXUW - Extended Maximum of Packed Unsigned Integers
//
// Performs SIMD compare of packed unsigned integers and returns maximum values.
// These comprehensive tests cover edge cases and special value combinations.
//
// PMAXUB: Maximum of 16 packed unsigned byte integers (SSE2)
// PMAXUW: Maximum of 8 packed unsigned word integers (SSE4.1)
//
// Opcodes:
// 66 0F DE /r         PMAXUB xmm1, xmm2/m128   - Max of packed unsigned bytes
// 66 0F 38 3E /r      PMAXUW xmm1, xmm2/m128   - Max of packed unsigned words

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// Extended PMAXUB Tests - 16x Unsigned Byte Maximum
// ============================================================================

#[test]
fn test_pmaxub_first_larger_all() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
                 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F];
    let data2 = [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
                 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_second_larger_all() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    let data2 = [0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
                 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_alternating_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
                 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00];
    let data2 = [0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
                 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_sequential_increasing() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70,
                 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0];
    let data2 = [0x08, 0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78,
                 0x88, 0x98, 0xA8, 0xB8, 0xC8, 0xD8, 0xE8, 0xF8];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_identity_operation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0xde, 0xc0, // PMAXUB XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x42, 0x13, 0xAB, 0xCD, 0xEF, 0x55, 0x99, 0x77,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_power_of_two_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80,
                 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];
    let data2 = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01,
                 0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxub_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xde, 0xc1, // PMAXUB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01]);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended PMAXUW Tests - 8x Unsigned Word Maximum
// ============================================================================

#[test]
fn test_pmaxuw_first_larger_all() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x80, 0x01, 0x80, 0x02, 0x80, 0x03, 0x80,
                 0x04, 0x80, 0x05, 0x80, 0x06, 0x80, 0x07, 0x80];
    let data2 = [0x00, 0x10, 0x01, 0x10, 0x02, 0x10, 0x03, 0x10,
                 0x04, 0x10, 0x05, 0x10, 0x06, 0x10, 0x07, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_second_larger_all() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2 = [0x20, 0x00, 0x21, 0x00, 0x22, 0x00, 0x23, 0x00,
                 0x24, 0x00, 0x25, 0x00, 0x26, 0x00, 0x27, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
                 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00];
    let data2 = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF,
                 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_identity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x3e, 0xc0, // PMAXUW XMM0, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE,
                0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_high_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let data2 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_mixed_high_low() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF,
                 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x12, 0x34];
    let data2 = [0xFF, 0xFF, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80,
                 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_sequential_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0x10, 0x00, 0x20, 0x00, 0x30, 0x00,
                 0x40, 0x00, 0x50, 0x00, 0x60, 0x00, 0x70, 0x00];
    let data2 = [0x08, 0x00, 0x18, 0x00, 0x28, 0x00, 0x38, 0x00,
                 0x48, 0x00, 0x58, 0x00, 0x68, 0x00, 0x78, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_boundary_zero_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_power_of_two_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00,
                 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80, 0x00];
    let data2 = [0x00, 0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08,
                 0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmaxuw_inverse_patterns() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x38, 0x3e, 0xc1, // PMAXUW XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x80, 0x01, 0x81, 0x02, 0x82, 0x03, 0x83,
                 0x04, 0x84, 0x05, 0x85, 0x06, 0x86, 0x07, 0x87];
    let data2 = [0xFF, 0x7F, 0xFE, 0x7E, 0xFD, 0x7D, 0xFC, 0x7C,
                 0xFB, 0x7B, 0xFA, 0x7A, 0xF9, 0x79, 0xF8, 0x78];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}
