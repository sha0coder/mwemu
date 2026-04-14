use crate::*;

// PSRLW/PSRLD/PSRLQ - Shift Packed Data Right Logical (SSE2)
//
// Performs logical right shift on packed integers in XMM registers.
// Empty high-order bits are filled with zeros (no sign extension).
// If shift count > element size in bits, result is all zeros.
//
// PSRLW: Shift 8 packed word integers (16-bit each) right
// PSRLD: Shift 4 packed doubleword integers (32-bit each) right
// PSRLQ: Shift 2 packed quadword integers (64-bit each) right
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F D1 /r      PSRLW xmm1, xmm2/m128   - Shift words right by count in xmm2/m128
// 66 0F 71 /2 ib   PSRLW xmm1, imm8        - Shift words right by immediate
// 66 0F D2 /r      PSRLD xmm1, xmm2/m128   - Shift dwords right by count in xmm2/m128
// 66 0F 72 /2 ib   PSRLD xmm1, imm8        - Shift dwords right by immediate
// 66 0F D3 /r      PSRLQ xmm1, xmm2/m128   - Shift qwords right by count in xmm2/m128
// 66 0F 73 /2 ib   PSRLQ xmm1, imm8        - Shift qwords right by immediate

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PSRLW Tests - Shift 8x Word Right Logical
// ============================================================================

#[test]
fn test_psrlw_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,       // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x71, 0xd0, 0x00, // PSRLW XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_imm8_one_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0x01, // PSRLW XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x02, 0x00, 0x04, 0x00, 0x06, 0x00, 0x08, 0x00,
                0x0A, 0x00, 0x0C, 0x00, 0x0E, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_imm8_seven_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0x07, // PSRLW XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x00, 0x00, 0x01, 0x80, 0x01, 0x00, 0x02,
                0x80, 0x02, 0x00, 0x03, 0x80, 0x03, 0x00, 0x04];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_imm8_eight_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0x08, // PSRLW XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0xFE, 0x00, 0xFD, 0x00, 0xFC,
                0x00, 0xFB, 0x00, 0xFA, 0x00, 0xF9, 0x00, 0xF8];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_imm8_fifteen_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0x0F, // PSRLW XMM0, 15
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80,
                0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_imm8_sixteen_bits_zero_out() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0x10, // PSRLW XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_imm8_overflow() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0xFF, // PSRLW XMM0, 255
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_no_sign_extension() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xd0, 0x01, // PSRLW XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,       // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b,       // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xd1, 0xc1,       // PSRLW XMM0, XMM1
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
fn test_psrlw_xmm_count_four() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xd1, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xF0, 0x00, 0xE0, 0x00, 0xD0, 0x00, 0xC0, 0x00,
                0xB0, 0x00, 0xA0, 0x00, 0x90, 0x00, 0x80, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlw_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xd1, 0x03,       // PSRLW XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
                0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// PSRLD Tests - Shift 4x Dword Right Logical
// ============================================================================

#[test]
fn test_psrld_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x00, // PSRLD XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_one_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x01, // PSRLD XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
                0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_seven_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x07, // PSRLD XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
                0x80, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_eight_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x08, // PSRLD XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00,
                0x00, 0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_sixteen_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x10, // PSRLD XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xEE, 0xEE,
                0x00, 0x00, 0xDD, 0xDD, 0x00, 0x00, 0xCC, 0xCC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_thirtyone_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x1F, // PSRLD XMM0, 31
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_thirtytwo_bits_zero_out() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x20, // PSRLD XMM0, 32
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_imm8_overflow() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0xFF, // PSRLD XMM0, 255
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_no_sign_extension() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xd0, 0x01, // PSRLD XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xd2, 0xc1,       // PSRLD XMM0, XMM1
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
fn test_psrld_xmm_count_eight() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xd2, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0xEE, 0x00, 0x00,
                0x00, 0xDD, 0x00, 0x00, 0x00, 0xCC, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xd2, 0x03,       // PSRLD XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF,
                0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// PSRLQ Tests - Shift 2x Qword Right Logical
// ============================================================================

#[test]
fn test_psrlq_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x00, // PSRLQ XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_one_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x01, // PSRLQ XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_seven_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x07, // PSRLQ XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_eight_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x08, // PSRLQ XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_sixteen_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x10, // PSRLQ XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0xEE, 0xEE, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_thirtytwo_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x20, // PSRLQ XMM0, 32
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0xEE, 0xEE, 0xEE, 0xEE];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_sixtythree_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x3F, // PSRLQ XMM0, 63
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_sixtyfour_bits_zero_out() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x40, // PSRLQ XMM0, 64
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_imm8_overflow() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0xFF, // PSRLQ XMM0, 255
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_no_sign_extension() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xd0, 0x01, // PSRLQ XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xd3, 0xc1,       // PSRLQ XMM0, XMM1
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
fn test_psrlq_xmm_count_sixteen() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xd3, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0xEE, 0xEE, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xd3, 0x03,       // PSRLQ XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// Register Variant Tests
// ============================================================================

#[test]
fn test_psrlw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10,       // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x71, 0xd2, 0x04, // PSRLW XMM2, 4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xF0, 0x00, 0xE0, 0x00, 0xD0, 0x00, 0xC0, 0x00,
                0xB0, 0x00, 0xA0, 0x00, 0x90, 0x00, 0x80, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20,       // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x72, 0xd4, 0x08, // PSRLD XMM4, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0xEE, 0x00, 0x00,
                0x00, 0xDD, 0x00, 0x00, 0x00, 0xCC, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30,       // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x73, 0xd6, 0x10, // PSRLQ XMM6, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0xEE, 0xEE, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_psrlw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00,       // MOVDQA XMM8, [RAX]
        0x66, 0x41, 0x0f, 0x71, 0xd0, 0x01, // PSRLW XMM8, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x02, 0x00, 0x04, 0x00, 0x06, 0x00, 0x08, 0x00,
                0x0A, 0x00, 0x0C, 0x00, 0x0E, 0x00, 0x10, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrld_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10,       // MOVDQA XMM10, [RAX]
        0x66, 0x41, 0x0f, 0x72, 0xd2, 0x04, // PSRLD XMM10, 4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xF0, 0x00, 0x00, 0x00, 0xE0, 0x00, 0x00, 0x00,
                0xD0, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psrlq_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20,       // MOVDQA XMM12, [RAX]
        0x66, 0x41, 0x0f, 0x73, 0xd4, 0x08, // PSRLQ XMM12, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
