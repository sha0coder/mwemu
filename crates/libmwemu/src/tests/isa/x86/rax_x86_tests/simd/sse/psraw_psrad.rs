use crate::*;

// PSRAW/PSRAD - Shift Packed Data Right Arithmetic (SSE2)
//
// Performs arithmetic right shift on packed integers in XMM registers.
// Empty high-order bits are filled with the sign bit (sign extension).
// If shift count > element size in bits, result is all sign bits.
//
// PSRAW: Shift 8 packed word integers (16-bit each) right with sign extension
// PSRAD: Shift 4 packed doubleword integers (32-bit each) right with sign extension
//
// Note: PSRAQ (quadword arithmetic shift) was added in AVX512, not available in SSE2
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F E1 /r      PSRAW xmm1, xmm2/m128   - Shift words right by count in xmm2/m128
// 66 0F 71 /4 ib   PSRAW xmm1, imm8        - Shift words right by immediate
// 66 0F E2 /r      PSRAD xmm1, xmm2/m128   - Shift dwords right by count in xmm2/m128
// 66 0F 72 /4 ib   PSRAD xmm1, imm8        - Shift dwords right by immediate

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PSRAW Tests - Shift 8x Word Right Arithmetic
// ============================================================================

#[test]
fn test_psraw_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,       // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x71, 0xe0, 0x00, // PSRAW XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_one_bit_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x02, 0x00, 0x04, 0x00, 0x06, 0x00, 0x08, 0x00,
                0x0A, 0x00, 0x0C, 0x00, 0x0E, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_one_bit_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFE, 0xFF, 0xFC, 0xFF, 0xFA, 0xFF, 0xF8, 0xFF,
                0xF6, 0xFF, 0xF4, 0xFF, 0xF2, 0xFF, 0xF0, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_seven_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x07, // PSRAW XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x00, 0x00, 0x01, 0x80, 0x01, 0x00, 0x02,
                0x80, 0x02, 0x00, 0x03, 0x80, 0x03, 0x00, 0x04];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_seven_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x07, // PSRAW XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0xFF, 0x00, 0xFF, 0x80, 0xFE, 0x00, 0xFE,
                0x80, 0xFD, 0x00, 0xFD, 0x80, 0xFC, 0x00, 0xFC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_eight_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x08, // PSRAW XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04,
                0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_eight_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x08, // PSRAW XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0xFE, 0x00, 0xFD, 0x00, 0xFC,
                0x00, 0xFB, 0x00, 0xFA, 0x00, 0xF9, 0x00, 0xF8];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_fifteen_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x0F, // PSRAW XMM0, 15
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F,
                0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_fifteen_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x0F, // PSRAW XMM0, 15
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x80, 0x01, 0x80, 0x00, 0x80, 0x01, 0x80,
                0x00, 0x80, 0x01, 0x80, 0x00, 0x80, 0x01, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_sixteen_bits_sign_fill_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x10, // PSRAW XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x7F, 0xFE, 0x7F, 0xFD, 0x7F, 0xFC, 0x7F,
                0xFB, 0x7F, 0xFA, 0x7F, 0xF9, 0x7F, 0xF8, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_sixteen_bits_sign_fill_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x10, // PSRAW XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x80, 0x01, 0x80, 0x02, 0x80, 0x03, 0x80,
                0x04, 0x80, 0x05, 0x80, 0x06, 0x80, 0x07, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_imm8_mixed_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x04, 0x00, 0xFC, 0xFF, 0x08, 0x00, 0xF8, 0xFF,
                0x0C, 0x00, 0xF4, 0xFF, 0x10, 0x00, 0xF0, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,       // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b,       // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xe1, 0xc1,       // PSRAW XMM0, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_xmm_count_four_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xe1, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xF0, 0xFF, 0xE0, 0xFF, 0xD0, 0xFF, 0xC0, 0xFF,
                0xB0, 0xFF, 0xA0, 0xFF, 0x90, 0xFF, 0x80, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psraw_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xe1, 0x03,       // PSRAW XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x0F, 0x00, 0xF0, 0x00, 0x0F, 0x00, 0xF0,
                0x00, 0x0F, 0x00, 0xF0, 0x00, 0x0F, 0x00, 0xF0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// PSRAD Tests - Shift 4x Dword Right Arithmetic
// ============================================================================

#[test]
fn test_psrad_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x00, // PSRAD XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_one_bit_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x01, // PSRAD XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
                0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_one_bit_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x01, // PSRAD XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFE, 0xFF, 0xFF, 0xFF, 0xFC, 0xFF, 0xFF, 0xFF,
                0xFA, 0xFF, 0xFF, 0xFF, 0xF8, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_seven_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x07, // PSRAD XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                0x80, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_seven_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x07, // PSRAD XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF,
                0x80, 0xFE, 0xFF, 0xFF, 0x00, 0xFE, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_eight_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x08, // PSRAD XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00,
                0x00, 0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_eight_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x08, // PSRAD XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xFE, 0xFF, 0xFF,
                0x00, 0xFD, 0xFF, 0xFF, 0x00, 0xFC, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_sixteen_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x10, // PSRAD XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00,
                0x00, 0x00, 0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_sixteen_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x10, // PSRAD XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFE, 0xFF,
                0x00, 0x00, 0xFD, 0xFF, 0x00, 0x00, 0xFC, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_thirtyone_bits_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x1F, // PSRAD XMM0, 31
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F,
                0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_thirtyone_bits_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x1F, // PSRAD XMM0, 31
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80,
                0x02, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_thirtytwo_bits_sign_fill_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x20, // PSRAD XMM0, 32
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0x7F, 0xFE, 0xFF, 0xFF, 0x7F,
                0xFD, 0xFF, 0xFF, 0x7F, 0xFC, 0xFF, 0xFF, 0x7F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_thirtytwo_bits_sign_fill_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x20, // PSRAD XMM0, 32
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80,
                0x02, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_imm8_mixed_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xe0, 0x01, // PSRAD XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x08, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF,
                0x10, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xe2, 0xc1,       // PSRAD XMM0, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_xmm_count_eight_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xe2, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0xFF, 0xFF, 0x00, 0xEE, 0xFF, 0xFF,
                0x00, 0xDD, 0xFF, 0xFF, 0x00, 0xCC, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xe2, 0x03,       // PSRAD XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0xF0, 0xFF,
                0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0xF0, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// Register Variant Tests
// ============================================================================

#[test]
fn test_psraw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10,       // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x71, 0xe2, 0x04, // PSRAW XMM2, 4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xF0, 0x00, 0xF0, 0xFF, 0xE0, 0x00, 0xE0, 0xFF,
                0xD0, 0x00, 0xD0, 0xFF, 0xC0, 0x00, 0xC0, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20,       // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x72, 0xe4, 0x08, // PSRAD XMM4, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
                0x00, 0xEE, 0x00, 0x00, 0x00, 0xEE, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_psraw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00,       // MOVDQA XMM8, [RAX]
        0x66, 0x41, 0x0f, 0x71, 0xe0, 0x01, // PSRAW XMM8, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x04, 0x00, 0xFC, 0xFF, 0x08, 0x00, 0xF8, 0xFF,
                0x0C, 0x00, 0xF4, 0xFF, 0x10, 0x00, 0xF0, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrad_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10,       // MOVDQA XMM10, [RAX]
        0x66, 0x41, 0x0f, 0x72, 0xe2, 0x04, // PSRAD XMM10, 4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xF0, 0x00, 0x00, 0x00, 0xF0, 0xFF, 0xFF, 0xFF,
                0xE0, 0x00, 0x00, 0x00, 0xE0, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
