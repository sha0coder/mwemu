use crate::*;

// VPMOVZXWD/VPMOVZXWQ/VPMOVZXDQ - Zero Extend Packed Words/Dwords (AVX2)
//
// Zero extends packed word/dword integers to larger element sizes.
// The lower elements from the source are zero-extended to fill the destination.
//
// VPMOVZXWD: Zero extend 8 packed words to 8 dwords (16->32 bit)
// VPMOVZXWQ: Zero extend 4 packed words to 4 qwords (16->64 bit)
// VPMOVZXDQ: Zero extend 4 packed dwords to 4 qwords (32->64 bit)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 33 /r       VPMOVZXWD ymm1, xmm2/m128
// VEX.256.66.0F38.WIG 34 /r       VPMOVZXWQ ymm1, xmm2/m64
// VEX.256.66.0F38.WIG 35 /r       VPMOVZXDQ ymm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPMOVZXWD Tests - 8x Word to Dword Zero Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovzxwd_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVZXWD YMM0, XMM1 - zero extend 8 words to 8 dwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x33, 0xc1, // VPMOVZXWD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x33, 0xdc, // VPMOVZXWD YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x33, 0xf7, // VPMOVZXWD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x33, 0xca, // VPMOVZXWD YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x33, 0xe5, // VPMOVZXWD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x33, 0xf8, // VPMOVZXWD YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVZXWD YMM0, [memory] - load and zero extend from memory
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x33, 0x00, // VPMOVZXWD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00,
        0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x33, 0x28, // VPMOVZXWD YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x33, 0x18, // VPMOVZXWD YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x80, 0x01, 0x80, 0x02, 0x80, 0x03, 0x80,
        0x04, 0x80, 0x05, 0x80, 0x06, 0x80, 0x07, 0x80,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVZXWQ Tests - 4x Word to Qword Zero Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovzxwq_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVZXWQ YMM0, XMM1 - zero extend 4 words to 4 qwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x34, 0xc1, // VPMOVZXWQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x34, 0xdc, // VPMOVZXWQ YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x34, 0xf7, // VPMOVZXWQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x34, 0xca, // VPMOVZXWQ YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x34, 0xe5, // VPMOVZXWQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x34, 0xf8, // VPMOVZXWQ YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVZXWQ YMM0, [memory] - load 8 bytes and zero extend to 4 qwords
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x34, 0x00, // VPMOVZXWQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x34, 0x28, // VPMOVZXWQ YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x34, 0x18, // VPMOVZXWQ YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x80, 0x01, 0x80, 0x02, 0x80, 0x03, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVZXDQ Tests - 4x Dword to Qword Zero Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovzxdq_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVZXDQ YMM0, XMM1 - zero extend 4 dwords to 4 qwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x35, 0xc1, // VPMOVZXDQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x35, 0xdc, // VPMOVZXDQ YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x35, 0xf7, // VPMOVZXDQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x35, 0xca, // VPMOVZXDQ YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x35, 0xe5, // VPMOVZXDQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x35, 0xf8, // VPMOVZXDQ YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVZXDQ YMM0, [memory] - load 16 bytes and zero extend to 4 qwords
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x35, 0x00, // VPMOVZXDQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x35, 0x28, // VPMOVZXDQ YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x35, 0x18, // VPMOVZXDQ YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80,
        0x02, 0x00, 0x00, 0x80, 0x03, 0x00, 0x00, 0x80,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vpmovzxwd_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x33, 0xc1, // VPMOVZXWD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x33, 0xd0, // VPMOVZXWD YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x34, 0xc1, // VPMOVZXWQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x34, 0xd0, // VPMOVZXWQ YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x35, 0xc1, // VPMOVZXDQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x35, 0xd0, // VPMOVZXDQ YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzx_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x33, 0xc1, // VPMOVZXWD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x34, 0xd2, // VPMOVZXWQ YMM2, XMM2
        0xc4, 0xe2, 0x7d, 0x35, 0xe3, // VPMOVZXDQ YMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x33, 0xc1, // VPMOVZXWD YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x33, 0xd5, // VPMOVZXWD YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x33, 0xff, // VPMOVZXWD YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x34, 0xc1, // VPMOVZXWQ YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x34, 0xd5, // VPMOVZXWQ YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x34, 0xff, // VPMOVZXWQ YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x35, 0xc1, // VPMOVZXDQ YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x35, 0xd5, // VPMOVZXDQ YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x35, 0xff, // VPMOVZXDQ YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x33, 0xc0, // VPMOVZXWD YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x33, 0xce, // VPMOVZXWD YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x33, 0xd7, // VPMOVZXWD YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x33, 0xf8, // VPMOVZXWD YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x34, 0xc0, // VPMOVZXWQ YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x34, 0xce, // VPMOVZXWQ YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x34, 0xd7, // VPMOVZXWQ YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x34, 0xf8, // VPMOVZXWQ YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x35, 0xc0, // VPMOVZXDQ YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x35, 0xce, // VPMOVZXDQ YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x35, 0xd7, // VPMOVZXDQ YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x35, 0xf8, // VPMOVZXDQ YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_mem_zero_words() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x33, 0x00, // VPMOVZXWD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_mem_zero_words() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x34, 0x00, // VPMOVZXWQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_mem_zero_dwords() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x35, 0x00, // VPMOVZXDQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwd_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x33, 0x00, // VPMOVZXWD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxwq_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x34, 0x00, // VPMOVZXWQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxdq_mem_max_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x35, 0x00, // VPMOVZXDQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
