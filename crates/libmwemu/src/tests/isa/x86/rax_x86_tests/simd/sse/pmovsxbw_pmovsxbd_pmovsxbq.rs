use crate::*;

// PMOVSXBW/PMOVSXBD/PMOVSXBQ - Sign Extend Packed Byte Integers
//
// These instructions sign-extend packed signed byte integers to larger integer types.
//
// PMOVSXBW: Sign extend 8 packed signed bytes to 8 packed signed words
// PMOVSXBD: Sign extend 4 packed signed bytes to 4 packed signed dwords
// PMOVSXBQ: Sign extend 2 packed signed bytes to 2 packed signed qwords
//
// Opcodes:
// 66 0F 38 20 /r      PMOVSXBW xmm1, xmm2/m64   - Sign extend 8 bytes to 8 words
// 66 0F 38 21 /r      PMOVSXBD xmm1, xmm2/m32   - Sign extend 4 bytes to 4 dwords
// 66 0F 38 22 /r      PMOVSXBQ xmm1, xmm2/m16   - Sign extend 2 bytes to 2 qwords

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PMOVSXBW Tests - 8 Bytes to 8 Words
// ============================================================================

#[test]
fn test_pmovsxbw_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x20, 0xc8, // PMOVSXBW XMM1, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x7F,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF0,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x01, 0xFF, 0x40, 0xC0, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x20, 0x00, // PMOVSXBW XMM0, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x20, 0xda, // PMOVSXBW XMM3, XMM2
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x03, 0xFD, 0x04, 0xFC,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PMOVSXBD Tests - 4 Bytes to 4 Dwords
// ============================================================================

#[test]
fn test_pmovsxbd_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8, // PMOVSXBD XMM1, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x7F, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0xFF, 0xFE, 0xFD, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x01, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x21, 0x00, // PMOVSXBD XMM0, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x11, 0x22, 0x33, 0x44, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x38, 0x21, 0xec, // PMOVSXBD XMM5, XMM4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x7F, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PMOVSXBQ Tests - 2 Bytes to 2 Qwords
// ============================================================================

#[test]
fn test_pmovsxbq_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8, // PMOVSXBQ XMM1, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x22, 0x00, // PMOVSXBQ XMM0, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x11, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x38, 0x22, 0xfe, // PMOVSXBQ XMM7, XMM6
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_boundary_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pmovsxbw_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x20, 0xc8, // PMOVSXBW XMM9, XMM8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x03, 0xFD, 0x04, 0xFC,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x21, 0xda, // PMOVSXBD XMM11, XMM10
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x22, 0xec, // PMOVSXBQ XMM13, XMM12
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_xmm14_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x38, 0x20, 0x30, // PMOVSXBW XMM14, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x01, 0xFF, 0x40, 0xC0, 0x20, 0xE0,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_xmm15_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x38, 0x21, 0x38, // PMOVSXBD XMM15, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x01, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined/Sequence Tests
// ============================================================================

#[test]
fn test_pmovsxbw_pmovsxbd_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x20, 0xc8, // PMOVSXBW XMM1, XMM0
        0x66, 0x0f, 0x38, 0x21, 0xd0, // PMOVSXBD XMM2, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x03, 0xFD, 0x04, 0xFC,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_all_pmovsxb_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x20, 0xc8, // PMOVSXBW XMM1, XMM0
        0x66, 0x0f, 0x38, 0x21, 0xd0, // PMOVSXBD XMM2, XMM0
        0x66, 0x0f, 0x38, 0x22, 0xd8, // PMOVSXBQ XMM3, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x03, 0xFD, 0x04, 0xFC,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_chain() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x20, 0xc8, // PMOVSXBW XMM1, XMM0
        0x66, 0x0f, 0x38, 0x20, 0xd1, // PMOVSXBW XMM2, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0xFF, 0x02, 0xFE, 0x03, 0xFD, 0x04, 0xFC,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_all_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_all_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x81, 0x82, 0x83, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_all_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x80, 0x81, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_alternating_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x20, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80, 0x7F, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_alternating_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x21, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x7F, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_alternating_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x22, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x7F, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbw_double_memory_load() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x20, 0x00, // PMOVSXBW XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x20, 0x0b, // PMOVSXBW XMM1, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbd_double_memory_load() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x21, 0x00, // PMOVSXBD XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x21, 0x0b, // PMOVSXBD XMM1, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x02, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFE, 0xFD, 0xFC, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovsxbq_double_memory_load() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x22, 0x00, // PMOVSXBQ XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x22, 0x0b, // PMOVSXBQ XMM1, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}
