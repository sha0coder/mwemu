use crate::*;

// VPMOVSXWD/VPMOVSXWQ/VPMOVSXDQ - Sign Extend Packed Words/Dwords (AVX2)
//
// Sign extends packed word/dword integers to larger element sizes.
// The lower elements from the source are sign-extended to fill the destination.
//
// VPMOVSXWD: Sign extend 8 packed words to 8 dwords (16->32 bit)
// VPMOVSXWQ: Sign extend 4 packed words to 4 qwords (16->64 bit)
// VPMOVSXDQ: Sign extend 4 packed dwords to 4 qwords (32->64 bit)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 23 /r       VPMOVSXWD ymm1, xmm2/m128
// VEX.256.66.0F38.WIG 24 /r       VPMOVSXWQ ymm1, xmm2/m64
// VEX.256.66.0F38.WIG 25 /r       VPMOVSXDQ ymm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPMOVSXWD Tests - 8x Word to Dword Sign Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovsxwd_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVSXWD YMM0, XMM1 - sign extend 8 words to 8 dwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x23, 0xc1, // VPMOVSXWD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x23, 0xdc, // VPMOVSXWD YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x23, 0xf7, // VPMOVSXWD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x23, 0xca, // VPMOVSXWD YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x23, 0xe5, // VPMOVSXWD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x23, 0xf8, // VPMOVSXWD YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVSXWD YMM0, [memory] - load and sign extend from memory
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x23, 0x00, // VPMOVSXWD YMM0, [RAX]
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
fn test_vpmovsxwd_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x23, 0x28, // VPMOVSXWD YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // All -1 in signed 16-bit
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x23, 0x18, // VPMOVSXWD YMM11, [RAX]
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
// VPMOVSXWQ Tests - 4x Word to Qword Sign Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovsxwq_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVSXWQ YMM0, XMM1 - sign extend 4 words to 4 qwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x24, 0xc1, // VPMOVSXWQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x24, 0xdc, // VPMOVSXWQ YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x24, 0xf7, // VPMOVSXWQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x24, 0xca, // VPMOVSXWQ YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x24, 0xe5, // VPMOVSXWQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x24, 0xf8, // VPMOVSXWQ YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVSXWQ YMM0, [memory] - load 8 bytes and sign extend to 4 qwords
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x24, 0x00, // VPMOVSXWQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x24, 0x28, // VPMOVSXWQ YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFE, 0xFF, 0xFD, 0xFF, 0xFC, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x24, 0x18, // VPMOVSXWQ YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x80, 0x01, 0x80, 0x02, 0x80, 0x03, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVSXDQ Tests - 4x Dword to Qword Sign Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovsxdq_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVSXDQ YMM0, XMM1 - sign extend 4 dwords to 4 qwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x25, 0xc1, // VPMOVSXDQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x25, 0xdc, // VPMOVSXDQ YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x25, 0xf7, // VPMOVSXDQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x25, 0xca, // VPMOVSXDQ YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x25, 0xe5, // VPMOVSXDQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x25, 0xf8, // VPMOVSXDQ YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVSXDQ YMM0, [memory] - load 16 bytes and sign extend to 4 qwords
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x25, 0x00, // VPMOVSXDQ YMM0, [RAX]
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
fn test_vpmovsxdq_ymm5_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x25, 0x28, // VPMOVSXDQ YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // All -1 in signed 32-bit
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x25, 0x18, // VPMOVSXDQ YMM11, [RAX]
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
fn test_vpmovsxwd_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x23, 0xc1, // VPMOVSXWD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x23, 0xd0, // VPMOVSXWD YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x24, 0xc1, // VPMOVSXWQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x24, 0xd0, // VPMOVSXWQ YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x25, 0xc1, // VPMOVSXDQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x25, 0xd0, // VPMOVSXDQ YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsx_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x23, 0xc1, // VPMOVSXWD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x24, 0xd2, // VPMOVSXWQ YMM2, XMM2
        0xc4, 0xe2, 0x7d, 0x25, 0xe3, // VPMOVSXDQ YMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x23, 0xc1, // VPMOVSXWD YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x23, 0xd5, // VPMOVSXWD YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x23, 0xff, // VPMOVSXWD YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x24, 0xc1, // VPMOVSXWQ YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x24, 0xd5, // VPMOVSXWQ YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x24, 0xff, // VPMOVSXWQ YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x25, 0xc1, // VPMOVSXDQ YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x25, 0xd5, // VPMOVSXDQ YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x25, 0xff, // VPMOVSXDQ YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x23, 0xc0, // VPMOVSXWD YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x23, 0xce, // VPMOVSXWD YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x23, 0xd7, // VPMOVSXWD YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x23, 0xf8, // VPMOVSXWD YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x24, 0xc0, // VPMOVSXWQ YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x24, 0xce, // VPMOVSXWQ YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x24, 0xd7, // VPMOVSXWQ YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x24, 0xf8, // VPMOVSXWQ YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x25, 0xc0, // VPMOVSXDQ YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x25, 0xce, // VPMOVSXDQ YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x25, 0xd7, // VPMOVSXDQ YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x25, 0xf8, // VPMOVSXDQ YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_mem_zero_words() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x23, 0x00, // VPMOVSXWD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_mem_zero_words() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x24, 0x00, // VPMOVSXWQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_mem_zero_dwords() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x25, 0x00, // VPMOVSXDQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwd_positive_negative_mix() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x23, 0x00, // VPMOVSXWD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xFF, 0x7F, // 32767 (positive)
        0x00, 0x80, // -32768 (negative)
        0x01, 0x00, // 1 (positive)
        0xFF, 0xFF, // -1 (negative)
        0x00, 0x00, // 0
        0x00, 0x40, // 16384 (positive)
        0x00, 0xC0, // -16384 (negative)
        0x55, 0x55, // 21845 (positive)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxwq_positive_negative_mix() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x24, 0x00, // VPMOVSXWQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xFF, 0x7F, // 32767 (positive)
        0x00, 0x80, // -32768 (negative)
        0x01, 0x00, // 1 (positive)
        0xFF, 0xFF, // -1 (negative)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxdq_positive_negative_mix() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x25, 0x00, // VPMOVSXDQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xFF, 0xFF, 0xFF, 0x7F, // 2147483647 (positive)
        0x00, 0x00, 0x00, 0x80, // -2147483648 (negative)
        0x01, 0x00, 0x00, 0x00, // 1 (positive)
        0xFF, 0xFF, 0xFF, 0xFF, // -1 (negative)
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
