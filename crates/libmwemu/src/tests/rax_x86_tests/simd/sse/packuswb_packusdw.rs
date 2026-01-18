use crate::*;

// PACKUSWB/PACKUSDW - Pack with Unsigned Saturation (SSE2/SSE4.1)
//
// Converts packed signed integers from source and destination operands
// into packed unsigned integers of smaller data type using unsigned saturation.
// Negative values saturate to 0, values beyond max saturate to max.
//
// PACKUSWB: Converts 8 signed word integers (16-bit) from dest and 8 from src
//           into 16 unsigned byte integers (8-bit) with unsigned saturation
//           Range: 0 to 255 (0xFF)
//           Negative values -> 0, values > 255 -> 255
//
// PACKUSDW: Converts 4 signed dword integers (32-bit) from dest and 4 from src
//           into 8 unsigned word integers (16-bit) with unsigned saturation
//           Range: 0 to 65535 (0xFFFF)
//           Negative values -> 0, values > 65535 -> 65535
//
// Opcodes:
// 66 0F 67 /r          PACKUSWB xmm1, xmm2/m128   - Pack words to unsigned bytes (SSE2)
// 66 0F 38 2B /r       PACKUSDW xmm1, xmm2/m128   - Pack dwords to unsigned words (SSE4.1)

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PACKUSWB Tests - Pack Words to Unsigned Bytes
// ============================================================================

#[test]
fn test_packuswb_all_zeros() {
    let mut emu = emu64();
    // PACKUSWB XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x67, 0xc1, // PACKUSWB XMM0, XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // 8 words: 1, 2, 3, 4, 5, 6, 7, 8
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    // 8 words: 9, 10, 11, 12, 13, 14, 15, 16
    let data2 = [0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00,
                 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_negative_saturate_to_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // 8 words: -1, -2, -3, -4, -5, -6, -7, -8
    let data1 = [0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF,
                 0xFB, 0xFF, 0xFA, 0xFF, 0xF9, 0xFF, 0xF8, 0xFF];
    // 8 words: -9, -10, -100, -1000, -10000, -20000, -30000, -32768
    let data2 = [0xF7, 0xFF, 0xF6, 0xFF, 0x9C, 0xFF, 0x18, 0xFC,
                 0xF0, 0xD8, 0xE0, 0xB1, 0xD0, 0x8A, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_saturate_positive_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x01, 0x2C, 0x01, 0xE8, 0x03, 0xFF, 0x7F,
                 0x00, 0x10, 0xFF, 0x0F, 0x00, 0x20, 0x00, 0x40];
    let data2 = [0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F,
                 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00,
                 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00];
    let data2 = [0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00,
                 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_mixed_saturation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x0A, 0x00, 0xC8, 0x00, 0x32, 0x00, 0x2C, 0x01,
                 0x64, 0x00, 0x90, 0x01, 0x96, 0x00, 0xF4, 0x01];
    let data2 = [0xF6, 0xFF, 0xFF, 0x00, 0xCE, 0xFF, 0x00, 0x01,
                 0x00, 0x00, 0xE8, 0x03, 0xFF, 0xFF, 0x18, 0xFC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x6f, 0x1b, // MOVDQA XMM3, [RBX]
        0x66, 0x0f, 0x67, 0xd3, // PACKUSWB XMM2, XMM3
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x10, 0x00, 0x20, 0x00, 0x30, 0x00, 0x40, 0x00,
                 0x50, 0x00, 0x60, 0x00, 0x70, 0x00, 0x80, 0x00];
    let data2 = [0x90, 0x00, 0xA0, 0x00, 0xB0, 0x00, 0xC0, 0x00,
                 0xD0, 0x00, 0xE0, 0x00, 0xF0, 0x00, 0xFF, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_from_memory() {
    let mut emu = emu64();
    // PACKUSWB XMM0, [mem]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x67, 0x03, // PACKUSWB XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2 = [0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00,
                 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_xmm7_xmm6() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x38, // MOVDQA XMM7, [RAX]
        0x66, 0x0f, 0x6f, 0x33, // MOVDQA XMM6, [RBX]
        0x66, 0x0f, 0x67, 0xfe, // PACKUSWB XMM7, XMM6
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00,
                 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00];
    let data2 = [0x08, 0x00, 0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00,
                 0x0C, 0x00, 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// PACKUSDW Tests - Pack Dwords to Unsigned Words
// ============================================================================

#[test]
fn test_packusdw_all_zeros() {
    let mut emu = emu64();
    // PACKUSDW XMM0, XMM1 with all zeros
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // 4 dwords: 1, 2, 3, 4
    let data1 = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    // 4 dwords: 5, 6, 7, 8
    let data2 = [0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
                 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_negative_saturate_to_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    // 4 dwords: -1, -2, -3, -4
    let data1 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF,
                 0xFD, 0xFF, 0xFF, 0xFF, 0xFC, 0xFF, 0xFF, 0xFF];
    // 4 dwords: -100, -1000, -100000, -2147483648
    let data2 = [0x9C, 0xFF, 0xFF, 0xFF, 0x18, 0xFC, 0xFF, 0xFF,
                 0x60, 0x79, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_saturate_positive_max() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0x01, 0x00, 0x70, 0x11, 0x01, 0x00,
                 0xA0, 0x86, 0x01, 0x00, 0xFF, 0xFF, 0xFF, 0x7F];
    let data2 = [0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F,
                 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00];
    let data2 = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_mixed_saturation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x64, 0x00, 0x00, 0x00, 0x70, 0x11, 0x01, 0x00,
                 0x88, 0x13, 0x00, 0x00, 0xA0, 0x86, 0x01, 0x00];
    let data2 = [0x9C, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                 0x78, 0xEC, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x6f, 0x2b, // MOVDQA XMM5, [RBX]
        0x66, 0x0f, 0x38, 0x2b, 0xe5, // PACKUSDW XMM4, XMM5
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
                 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00];
    let data2 = [0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00,
                 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_from_memory() {
    let mut emu = emu64();
    // PACKUSDW XMM0, [mem]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x2b, 0x03, // PACKUSDW XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x0A, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00,
                 0x1E, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00];
    let data2 = [0x32, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x00, 0x00,
                 0x46, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm1_xmm2() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0x66, 0x0f, 0x6f, 0x13, // MOVDQA XMM2, [RBX]
        0x66, 0x0f, 0x38, 0x2b, 0xca, // PACKUSDW XMM1, XMM2
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00,
                 0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00];
    let data2 = [0xFB, 0x00, 0x00, 0x00, 0xFA, 0x00, 0x00, 0x00,
                 0xF9, 0x00, 0x00, 0x00, 0xF8, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_packuswb_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x0b, // MOVDQA XMM9, [RBX]
        0x66, 0x45, 0x0f, 0x67, 0xc1, // PACKUSWB XMM8, XMM9
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2 = [0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00,
                 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x1b, // MOVDQA XMM11, [RBX]
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xd3, // PACKUSDW XMM10, XMM11
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
                 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00];
    let data2 = [0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00,
                 0x00, 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x2b, // MOVDQA XMM13, [RBX]
        0x66, 0x45, 0x0f, 0x67, 0xe5, // PACKUSWB XMM12, XMM13
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x00, 0x00, 0xFF, 0x00, 0x80, 0x00, 0x00, 0x01,
                 0x7F, 0x00, 0x01, 0x00, 0xFE, 0xFF, 0x00, 0x00];
    let data2 = [0x50, 0x00, 0xA0, 0x00, 0x64, 0x00, 0xC8, 0x00,
                 0x32, 0x00, 0x96, 0x00, 0x14, 0x00, 0xFA, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_xmm14_xmm15() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x30, // MOVDQA XMM14, [RAX]
        0x66, 0x44, 0x0f, 0x6f, 0x3b, // MOVDQA XMM15, [RBX]
        0x66, 0x45, 0x0f, 0x38, 0x2b, 0xf7, // PACKUSDW XMM14, XMM15
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x01, 0x00, 0x9C, 0xFF, 0xFF, 0xFF];
    let data2 = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x7F,
                 0x10, 0x27, 0x00, 0x00, 0xF0, 0xD8, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequence and Combination Tests
// ============================================================================

#[test]
fn test_packuswb_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x67, 0xc1, // PACKUSWB XMM0, XMM1
        0x66, 0x0f, 0x67, 0xd1, // PACKUSWB XMM2, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    let data2 = [0x09, 0x00, 0x0A, 0x00, 0x0B, 0x00, 0x0C, 0x00,
                 0x0D, 0x00, 0x0E, 0x00, 0x0F, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd1, // PACKUSDW XMM2, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    let data2 = [0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
                 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_then_packuswb() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b, // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x2b, 0xc1, // PACKUSDW XMM0, XMM1
        0x66, 0x0f, 0x38, 0x2b, 0xd1, // PACKUSDW XMM2, XMM1
        0x66, 0x0f, 0x67, 0xc2, // PACKUSWB XMM0, XMM2
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    let data2 = [0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
                 0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_all_saturate_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03,
                0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03, 0xE8, 0x03];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_all_saturate_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC,
                0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC, 0x18, 0xFC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_all_saturate_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xA0, 0x86, 0x01, 0x00, 0xA0, 0x86, 0x01, 0x00,
                0xA0, 0x86, 0x01, 0x00, 0xA0, 0x86, 0x01, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_all_saturate_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x60, 0x79, 0xFE, 0xFF, 0x60, 0x79, 0xFE, 0xFF,
                0x60, 0x79, 0xFE, 0xFF, 0x60, 0x79, 0xFE, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_alternating_saturation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x0A, 0x00, 0x2C, 0x01, 0x14, 0x00, 0x90, 0x01,
                 0x1E, 0x00, 0xF4, 0x01, 0x28, 0x00, 0x58, 0x02];
    let data2 = [0xF6, 0xFF, 0xFF, 0x00, 0xEC, 0xFF, 0x00, 0x01,
                 0xE2, 0xFF, 0x00, 0x00, 0xD8, 0xFF, 0x01, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_alternating_saturation() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x64, 0x00, 0x00, 0x00, 0x70, 0x11, 0x01, 0x00,
                 0xE8, 0x03, 0x00, 0x00, 0xA0, 0x86, 0x01, 0x00];
    let data2 = [0x9C, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00,
                 0x18, 0xFC, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packuswb_edge_cases() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x67, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFE, 0x00, 0xFF, 0x00, 0x00, 0x01, 0x01, 0x01,
                 0xFF, 0xFF, 0xFE, 0xFF, 0x00, 0x00, 0x01, 0x00];
    let data2 = [0x7F, 0x00, 0x80, 0x00, 0x81, 0x00, 0x00, 0x80,
                 0xFF, 0x7F, 0x01, 0x80, 0x00, 0x00, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_packusdw_edge_cases() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0x38, 0x2b, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0xFE, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
                 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00];
    let data2 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF,
                 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}
