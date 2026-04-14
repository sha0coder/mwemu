use crate::*;

// PMOVZXWD/PMOVZXWQ/PMOVZXDQ - Zero Extend Packed Integer Types
//
// These instructions zero-extend packed unsigned integers to larger integer types.
//
// PMOVZXWD: Zero extend 4 packed unsigned words to 4 packed unsigned dwords
// PMOVZXWQ: Zero extend 2 packed unsigned words to 2 packed unsigned qwords
// PMOVZXDQ: Zero extend 2 packed unsigned dwords to 2 packed unsigned qwords
//
// Opcodes:
// 66 0F 38 33 /r      PMOVZXWD xmm1, xmm2/m64   - Zero extend 4 words to 4 dwords
// 66 0F 38 34 /r      PMOVZXWQ xmm1, xmm2/m32   - Zero extend 2 words to 2 qwords
// 66 0F 38 35 /r      PMOVZXDQ xmm1, xmm2/m64   - Zero extend 2 dwords to 2 qwords

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// ============================================================================
// PMOVZXWD Tests - 4 Words to 4 Dwords
// ============================================================================

#[test]
fn test_pmovzxwd_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_small_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x33, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0xFF, 0x7F,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_large_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x33, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x80, 0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x33, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x7F, 0x00, 0x80, 0x01, 0x00, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x33, 0x00, // PMOVZXWD XMM0, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x11, 0x11, 0x22, 0x22, 0x33, 0x33, 0x44, 0x44,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_xmm2_xmm3() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x10, // MOVDQA XMM2, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xda, // PMOVZXWD XMM3, XMM2
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x33, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x33, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PMOVZXWQ Tests - 2 Words to 2 Qwords
// ============================================================================

#[test]
fn test_pmovzxwq_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8, // PMOVZXWQ XMM1, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_small_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_large_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x80, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x7F, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x34, 0x00, // PMOVZXWQ XMM0, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x11, 0x11, 0x22, 0x22, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_xmm4_xmm5() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x20, // MOVDQA XMM4, [RAX]
        0x66, 0x0f, 0x38, 0x34, 0xec, // PMOVZXWQ XMM5, XMM4
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// PMOVZXDQ Tests - 2 Dwords to 2 Qwords
// ============================================================================

#[test]
fn test_pmovzxdq_all_zeros() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8, // PMOVZXDQ XMM1, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_small_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x7F,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_large_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x80, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_mixed_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x35, 0x00, // PMOVZXDQ XMM0, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x11, 0x11, 0x11, 0x11, 0x22, 0x22, 0x22, 0x22,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_xmm6_xmm7() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x30, // MOVDQA XMM6, [RAX]
        0x66, 0x0f, 0x38, 0x35, 0xfe, // PMOVZXDQ XMM7, XMM6
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_all_ones() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_sequential() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Extended Register Tests (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pmovzxwd_xmm8_xmm9() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM9, XMM8
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_xmm10_xmm11() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x10, // MOVDQA XMM10, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x34, 0xda, // PMOVZXWQ XMM11, XMM10
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_xmm12_xmm13() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x20, // MOVDQA XMM12, [RAX]
        0x66, 0x45, 0x0f, 0x38, 0x35, 0xec, // PMOVZXDQ XMM13, XMM12
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_xmm14_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x38, 0x33, 0x30, // PMOVZXWD XMM14, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x7F, 0x00, 0x80, 0x01, 0x00, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_xmm15_from_memory() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x38, 0x34, 0x38, // PMOVZXWQ XMM15, [RAX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0xFF, 0x7F, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Combined/Sequence Tests
// ============================================================================

#[test]
fn test_pmovzxwd_pmovzxwq_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0x66, 0x0f, 0x38, 0x34, 0xd0, // PMOVZXWQ XMM2, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_all_pmovzxw_sequence() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0x66, 0x0f, 0x38, 0x34, 0xd0, // PMOVZXWQ XMM2, XMM0
        0x66, 0x0f, 0x38, 0x35, 0xd8, // PMOVZXDQ XMM3, XMM0
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_chain() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0xc8, // PMOVZXWD XMM1, XMM0
        0x66, 0x0f, 0x38, 0x35, 0xd1, // PMOVZXDQ XMM2, XMM1
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x01, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0xFE, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_double_memory_load() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x33, 0x00, // PMOVZXWD XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x33, 0x0b, // PMOVZXWD XMM1, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_double_memory_load() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x34, 0x00, // PMOVZXWQ XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x34, 0x0b, // PMOVZXWQ XMM1, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFF, 0xFE, 0xFF, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_double_memory_load() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[0x48, 0xbb]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x38, 0x35, 0x00, // PMOVZXDQ XMM0, [RAX]
        0x66, 0x0f, 0x38, 0x35, 0x0b, // PMOVZXDQ XMM1, [RBX]
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data1 = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let data2 = [0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &data2);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwd_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x33, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxwq_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x34, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_pmovzxdq_alternating() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00,
        0x66, 0x0f, 0x38, 0x35, 0xc8,
        0xf4,
    ]);

    emu.load_code_bytes(&full_code);
    let data = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
