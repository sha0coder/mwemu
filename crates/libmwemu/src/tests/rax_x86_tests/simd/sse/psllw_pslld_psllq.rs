use crate::*;

// PSLLW/PSLLD/PSLLQ - Shift Packed Data Left Logical (SSE2)
//
// Performs logical left shift on packed integers in XMM registers.
// Empty low-order bits are filled with zeros.
// If shift count > element size in bits, result is all zeros.
//
// PSLLW: Shift 8 packed word integers (16-bit each) left
// PSLLD: Shift 4 packed doubleword integers (32-bit each) left
// PSLLQ: Shift 2 packed quadword integers (64-bit each) left
//
// Opcodes (SSE2 - 128-bit XMM):
// 66 0F F1 /r      PSLLW xmm1, xmm2/m128   - Shift words left by count in xmm2/m128
// 66 0F 71 /6 ib   PSLLW xmm1, imm8        - Shift words left by immediate
// 66 0F F2 /r      PSLLD xmm1, xmm2/m128   - Shift dwords left by count in xmm2/m128
// 66 0F 72 /6 ib   PSLLD xmm1, imm8        - Shift dwords left by immediate
// 66 0F F3 /r      PSLLQ xmm1, xmm2/m128   - Shift qwords left by count in xmm2/m128
// 66 0F 73 /6 ib   PSLLQ xmm1, imm8        - Shift qwords left by immediate

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PSLLW Tests - Shift 8x Word Left
// ============================================================================

#[test]
fn test_psllw_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,       // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x71, 0xf0, 0x00, // PSLLW XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_imm8_one_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xf0, 0x01, // PSLLW XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_imm8_seven_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xf0, 0x07, // PSLLW XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_imm8_eight_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xf0, 0x08, // PSLLW XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x00, 0xFE, 0x00, 0xFD, 0x00, 0xFC, 0x00,
                0xFB, 0x00, 0xFA, 0x00, 0xF9, 0x00, 0xF8, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_imm8_fifteen_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xf0, 0x0F, // PSLLW XMM0, 15
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00,
                0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_imm8_sixteen_bits_zero_out() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xf0, 0x10, // PSLLW XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_imm8_overflow() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x71, 0xf0, 0xFF, // PSLLW XMM0, 255
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,       // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x6f, 0x0b,       // MOVDQA XMM1, [RBX]
        0x66, 0x0f, 0xf1, 0xc1,       // PSLLW XMM0, XMM1
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
fn test_psllw_xmm_count_one() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xf1, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psllw_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xf1, 0x03,       // PSLLW XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00,
                0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// PSLLD Tests - Shift 4x Dword Left
// ============================================================================

#[test]
fn test_pslld_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x00, // PSLLD XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_one_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x01, // PSLLD XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_seven_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x07, // PSLLD XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_eight_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x08, // PSLLD XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x00, 0x00, 0x00, 0xFE, 0x00, 0x00, 0x00,
                0xFD, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_sixteen_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x10, // PSLLD XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0x00, 0x00, 0xEE, 0xEE, 0x00, 0x00,
                0xDD, 0xDD, 0x00, 0x00, 0xCC, 0xCC, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_thirtyone_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x1F, // PSLLD XMM0, 31
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_thirtytwo_bits_zero_out() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0x20, // PSLLD XMM0, 32
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_imm8_overflow() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x72, 0xf0, 0xFF, // PSLLD XMM0, 255
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xf2, 0xc1,       // PSLLD XMM0, XMM1
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
fn test_pslld_xmm_count_eight() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xf2, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x00, 0x00, 0x00, 0xEE, 0x00, 0x00, 0x00,
                0xDD, 0x00, 0x00, 0x00, 0xCC, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xf2, 0x03,       // PSLLD XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
                0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

// ============================================================================
// PSLLQ Tests - Shift 2x Qword Left
// ============================================================================

#[test]
fn test_psllq_imm8_zero_shift() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x00, // PSLLQ XMM0, 0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_one_bit() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x01, // PSLLQ XMM0, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_seven_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x07, // PSLLQ XMM0, 7
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_eight_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x08, // PSLLQ XMM0, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_sixteen_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x10, // PSLLQ XMM0, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xEE, 0xEE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_thirtytwo_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x20, // PSLLQ XMM0, 32
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0xEE, 0xEE, 0xEE, 0xEE, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_sixtythree_bits() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x3F, // PSLLQ XMM0, 63
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_sixtyfour_bits_zero_out() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0x40, // PSLLQ XMM0, 64
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_imm8_overflow() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x73, 0xf0, 0xFF, // PSLLQ XMM0, 255
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_xmm_count_zero() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xf3, 0xc1,       // PSLLQ XMM0, XMM1
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
fn test_psllq_xmm_count_sixteen() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x6f, 0x0b,
        0x66, 0x0f, 0xf3, 0xc1,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xEE, 0xEE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    let count = [0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &count);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_xmm_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0xf3, 0x03,       // PSLLQ XMM0, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00];
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
fn test_psllw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10,       // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x71, 0xf2, 0x04, // PSLLW XMM2, 4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x01, 0x02, 0x02, 0x03, 0x03, 0x04, 0x04,
                0x05, 0x05, 0x06, 0x06, 0x07, 0x07, 0x08, 0x08];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20,       // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x72, 0xf4, 0x08, // PSLLD XMM4, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02,
                0x03, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30,       // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x73, 0xf6, 0x10, // PSLLQ XMM6, 16
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x02, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_psllw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00,       // MOVDQA XMM8, [RAX]
        0x66, 0x41, 0x0f, 0x71, 0xf0, 0x01, // PSLLW XMM8, 1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pslld_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10,       // MOVDQA XMM10, [RAX]
        0x66, 0x41, 0x0f, 0x72, 0xf2, 0x04, // PSLLD XMM10, 4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_psllq_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20,       // MOVDQA XMM12, [RAX]
        0x66, 0x41, 0x0f, 0x73, 0xf4, 0x08, // PSLLQ XMM12, 8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
