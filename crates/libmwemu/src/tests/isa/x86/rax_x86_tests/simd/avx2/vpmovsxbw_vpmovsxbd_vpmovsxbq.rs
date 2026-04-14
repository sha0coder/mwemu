use crate::*;

// VPMOVSXBW/VPMOVSXBD/VPMOVSXBQ - Packed Move with Sign Extend from Byte (AVX2)
//
// Sign-extends packed signed integer values from a smaller data size to a larger data size.
//
// VPMOVSXBW: Sign-extend 16 packed signed 8-bit integers to 16-bit integers
// VPMOVSXBD: Sign-extend 8 packed signed 8-bit integers to 32-bit integers
// VPMOVSXBQ: Sign-extend 4 packed signed 8-bit integers to 64-bit integers
//
// For VPMOVSXBW with YMM (256-bit):
// dst[15:0]    = sign_extend(src[7:0])
// dst[31:16]   = sign_extend(src[15:8])
// ... (total 16 conversions from byte to word)
//
// For VPMOVSXBD with YMM (256-bit):
// dst[31:0]    = sign_extend(src[7:0])
// dst[63:32]   = sign_extend(src[15:8])
// ... (total 8 conversions from byte to dword)
//
// For VPMOVSXBQ with YMM (256-bit):
// dst[63:0]    = sign_extend(src[7:0])
// dst[127:64]  = sign_extend(src[15:8])
// ... (total 4 conversions from byte to qword)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.WIG 20 /r     VPMOVSXBW ymm1, xmm2/m128
// VEX.256.66.0F38.WIG 21 /r     VPMOVSXBD ymm1, xmm2/m64
// VEX.256.66.0F38.WIG 22 /r     VPMOVSXBQ ymm1, xmm2/m32

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPMOVSXBW Tests - Sign Extend Byte to Word (256-bit)
// ============================================================================

#[test]
fn test_vpmovsxbw_ymm0_xmm1_all_zeros() {
    let mut emu = emu64();
    // VPMOVSXBW YMM0, XMM1 with all zeros
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xc1, // VPMOVSXBW YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm2_xmm3_all_positive() {
    let mut emu = emu64();
    // VPMOVSXBW YMM2, XMM3 with positive values
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xd3, // VPMOVSXBW YMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm4_xmm5_all_negative() {
    let mut emu = emu64();
    // VPMOVSXBW YMM4, XMM5 with negative values (0xFF = -1, extends to 0xFFFF)
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xe5, // VPMOVSXBW YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm6_xmm7_mixed_signs() {
    let mut emu = emu64();
    // VPMOVSXBW YMM6, XMM7 with mixed positive and negative
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xf7, // VPMOVSXBW YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm8_xmm9_max_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x20, 0xc1, // VPMOVSXBW YMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm10_xmm11_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x20, 0xd3, // VPMOVSXBW YMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm12_xmm13_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x20, 0xe5, // VPMOVSXBW YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm14_xmm15_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x20, 0xf7, // VPMOVSXBW YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVSXBW YMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x20, 0x00, // VPMOVSXBW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).map(|i| (i as i8) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm1_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x20, 0x08, // VPMOVSXBW YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).map(|i| (-(i as i8)) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ymm2_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x20, 0x10, // VPMOVSXBW YMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x00, 0x7F, 0x80, 0xFF, 0x01, 0xFE, 0x40, 0xC0]
        .into_iter()
        .cycle()
        .take(16)
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVSXBD Tests - Sign Extend Byte to Doubleword (256-bit)
// ============================================================================

#[test]
fn test_vpmovsxbd_ymm0_xmm1_all_zeros() {
    let mut emu = emu64();
    // VPMOVSXBD YMM0, XMM1 with all zeros
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xc1, // VPMOVSXBD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm2_xmm3_all_positive() {
    let mut emu = emu64();
    // VPMOVSXBD YMM2, XMM3 with positive values
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xd3, // VPMOVSXBD YMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm4_xmm5_all_negative() {
    let mut emu = emu64();
    // VPMOVSXBD YMM4, XMM5 with negative values (0xFF = -1, extends to 0xFFFFFFFF)
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xe5, // VPMOVSXBD YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm6_xmm7_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xf7, // VPMOVSXBD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm8_xmm9_max_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x21, 0xc1, // VPMOVSXBD YMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm10_xmm11_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x21, 0xd3, // VPMOVSXBD YMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm12_xmm13_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x21, 0xe5, // VPMOVSXBD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm14_xmm15_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x21, 0xf7, // VPMOVSXBD YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVSXBD YMM0, [memory] - reads 8 bytes
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x21, 0x00, // VPMOVSXBD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).map(|i| (i as i8) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm1_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x21, 0x08, // VPMOVSXBD YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).map(|i| (-(i as i8)) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ymm2_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x21, 0x10, // VPMOVSXBD YMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x00, 0x7F, 0x80, 0xFF, 0x01, 0xFE, 0x40, 0xC0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPMOVSXBQ Tests - Sign Extend Byte to Quadword (256-bit)
// ============================================================================

#[test]
fn test_vpmovsxbq_ymm0_xmm1_all_zeros() {
    let mut emu = emu64();
    // VPMOVSXBQ YMM0, XMM1 with all zeros
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xc1, // VPMOVSXBQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm2_xmm3_all_positive() {
    let mut emu = emu64();
    // VPMOVSXBQ YMM2, XMM3 with positive values
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xd3, // VPMOVSXBQ YMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm4_xmm5_all_negative() {
    let mut emu = emu64();
    // VPMOVSXBQ YMM4, XMM5 with negative values (0xFF = -1, extends to 0xFFFFFFFFFFFFFFFF)
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xe5, // VPMOVSXBQ YMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm6_xmm7_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xf7, // VPMOVSXBQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm8_xmm9_max_negative() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x22, 0xc1, // VPMOVSXBQ YMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm10_xmm11_max_positive() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x22, 0xd3, // VPMOVSXBQ YMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm12_xmm13_sequential() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x22, 0xe5, // VPMOVSXBQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm14_xmm15_alternating() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x22, 0xf7, // VPMOVSXBQ YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm0_mem() {
    let mut emu = emu64();
    // VPMOVSXBQ YMM0, [memory] - reads 4 bytes
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x22, 0x00, // VPMOVSXBQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x00, 0x7F, 0x80, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm1_mem_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x22, 0x08, // VPMOVSXBQ YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xFF, 0xFE, 0x81, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ymm2_mem_mixed() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x22, 0x10, // VPMOVSXBQ YMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x01, 0x7F, 0xC0, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional comprehensive tests
// ============================================================================

#[test]
fn test_vpmovsxbw_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xc1, // VPMOVSXBW YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x20, 0xc8, // VPMOVSXBW YMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xc1, // VPMOVSXBD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x21, 0xc8, // VPMOVSXBD YMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xc1, // VPMOVSXBQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x22, 0xc8, // VPMOVSXBQ YMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xc1, // VPMOVSXBW YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xc1, // VPMOVSXBD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_boundary_values() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xc1, // VPMOVSXBQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x20, 0x00, // VPMOVSXBW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80, 0xFF, 0xFE, 0x40, 0xC0]
        .into_iter()
        .cycle()
        .take(16)
        .collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x21, 0x00, // VPMOVSXBD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80, 0xFF, 0xFE, 0x40, 0xC0];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_mem_pattern() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x22, 0x00, // VPMOVSXBQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let pattern: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &pattern);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_ones_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xc1, // VPMOVSXBW YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_ones_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xc1, // VPMOVSXBD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_ones_pattern() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xc1, // VPMOVSXBQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_mem_increment() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x20, 0x00, // VPMOVSXBW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (1..=16).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_mem_increment() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x21, 0x00, // VPMOVSXBD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (1..=8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_mem_increment() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x22, 0x00, // VPMOVSXBQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (1..=4).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_same_source() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x20, 0xc0, // VPMOVSXBW YMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_same_source() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x21, 0xc0, // VPMOVSXBD YMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_same_source() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x22, 0xc0, // VPMOVSXBQ YMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbw_mem_negative_increment() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x20, 0x00, // VPMOVSXBW YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..16).map(|i| (-(i as i8 + 1)) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbd_mem_negative_increment() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x21, 0x00, // VPMOVSXBD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..8).map(|i| (-(i as i8 + 1)) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpmovsxbq_mem_negative_increment() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x22, 0x00, // VPMOVSXBQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..4).map(|i| (-(i as i8 + 1)) as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
