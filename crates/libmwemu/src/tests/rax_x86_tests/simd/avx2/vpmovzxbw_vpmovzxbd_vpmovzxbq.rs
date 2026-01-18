use crate::*;

// VPMOVZXBW/VPMOVZXBD/VPMOVZXBQ - Zero Extend Packed Bytes (AVX2)
//
// Zero extends packed byte integers to larger element sizes.
// The lower bytes from the source are zero-extended to fill the destination.
//
// VPMOVZXBW: Zero extend 16 packed bytes to 16 words (8->16 bit)
// VPMOVZXBD: Zero extend 8 packed bytes to 8 dwords (8->32 bit)
// VPMOVZXBQ: Zero extend 4 packed bytes to 4 qwords (8->64 bit)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 30 /r       VPMOVZXBW ymm1, xmm2/m128
// VEX.256.66.0F38.WIG 31 /r       VPMOVZXBD ymm1, xmm2/m64
// VEX.256.66.0F38.WIG 32 /r       VPMOVZXBQ ymm1, xmm2/m32

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPMOVZXBW Tests - 16x Byte to Word Zero Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovzxbw_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVZXBW YMM0, XMM1 - zero extend 16 bytes to 16 words
    let code = [
        0xc4, 0xe2, 0x7d, 0x30, 0xc1, // VPMOVZXBW YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x30, 0xdc, // VPMOVZXBW YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x30, 0xf7, // VPMOVZXBW YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x30, 0xca, // VPMOVZXBW YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x30, 0xe5, // VPMOVZXBW YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x30, 0xf8, // VPMOVZXBW YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVZXBW YMM0, [memory] - load and zero extend from memory
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x30, 0x00, // VPMOVZXBW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x30, 0x28, // VPMOVZXBW YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x30, 0x18, // VPMOVZXBW YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
                    0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVZXBD Tests - 8x Byte to Dword Zero Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovzxbd_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVZXBD YMM0, XMM1 - zero extend 8 bytes to 8 dwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x31, 0xc1, // VPMOVZXBD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x31, 0xdc, // VPMOVZXBD YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x31, 0xf7, // VPMOVZXBD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x31, 0xca, // VPMOVZXBD YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x31, 0xe5, // VPMOVZXBD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x31, 0xf8, // VPMOVZXBD YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVZXBD YMM0, [memory] - load 8 bytes and zero extend to 8 dwords
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x31, 0x00, // VPMOVZXBD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x31, 0x28, // VPMOVZXBD YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x31, 0x18, // VPMOVZXBD YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVZXBQ Tests - 4x Byte to Qword Zero Extension (256-bit)
// ============================================================================

#[test]
fn test_vpmovzxbq_ymm0_xmm1() {
    let mut emu = emu64();
    // VPMOVZXBQ YMM0, XMM1 - zero extend 4 bytes to 4 qwords
    let code = [
        0xc4, 0xe2, 0x7d, 0x32, 0xc1, // VPMOVZXBQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x32, 0xdc, // VPMOVZXBQ YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x32, 0xf7, // VPMOVZXBQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x32, 0xca, // VPMOVZXBQ YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x32, 0xe5, // VPMOVZXBQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x32, 0xf8, // VPMOVZXBQ YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVZXBQ YMM0, [memory] - load 4 bytes and zero extend to 4 qwords
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x32, 0x00, // VPMOVZXBQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x01, 0x02, 0x03, 0x04];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x32, 0x28, // VPMOVZXBQ YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFE, 0xFD, 0xFC];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_ymm11_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x32, 0x18, // VPMOVZXBQ YMM11, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x80, 0x81, 0x82, 0x83];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vpmovzxbw_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x30, 0xc1, // VPMOVZXBW YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x30, 0xd0, // VPMOVZXBW YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x31, 0xc1, // VPMOVZXBD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x31, 0xd0, // VPMOVZXBD YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x32, 0xc1, // VPMOVZXBQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x32, 0xd0, // VPMOVZXBQ YMM2, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzx_mixed_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x30, 0xc1, // VPMOVZXBW YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x31, 0xd2, // VPMOVZXBD YMM2, XMM2
        0xc4, 0xe2, 0x7d, 0x32, 0xe3, // VPMOVZXBQ YMM4, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x30, 0xc1, // VPMOVZXBW YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x30, 0xd5, // VPMOVZXBW YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x30, 0xff, // VPMOVZXBW YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x31, 0xc1, // VPMOVZXBD YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x31, 0xd5, // VPMOVZXBD YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x31, 0xff, // VPMOVZXBD YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x32, 0xc1, // VPMOVZXBQ YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x32, 0xd5, // VPMOVZXBQ YMM10, XMM13
        0xc4, 0x42, 0x7d, 0x32, 0xff, // VPMOVZXBQ YMM15, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x30, 0xc0, // VPMOVZXBW YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x30, 0xce, // VPMOVZXBW YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x30, 0xd7, // VPMOVZXBW YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x30, 0xf8, // VPMOVZXBW YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x31, 0xc0, // VPMOVZXBD YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x31, 0xce, // VPMOVZXBD YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x31, 0xd7, // VPMOVZXBD YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x31, 0xf8, // VPMOVZXBD YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x32, 0xc0, // VPMOVZXBQ YMM0, XMM0
        0xc4, 0xe2, 0x7d, 0x32, 0xce, // VPMOVZXBQ YMM1, XMM6
        0xc4, 0xe2, 0x7d, 0x32, 0xd7, // VPMOVZXBQ YMM2, XMM7
        0xc4, 0xe2, 0x7d, 0x32, 0xf8, // VPMOVZXBQ YMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbw_mem_zero_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x30, 0x00, // VPMOVZXBW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbd_mem_zero_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x31, 0x00, // VPMOVZXBD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovzxbq_mem_zero_bytes() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x32, 0x00, // VPMOVZXBQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
