use crate::*;

// VPBROADCASTD/VPBROADCASTQ - Broadcast Dword/Qword (AVX2)
//
// Broadcasts a single dword or qword value from source to all elements of destination.
// Can broadcast from XMM register or memory.
//
// VPBROADCASTD: Broadcast a single dword to all 8 dwords in YMM
// VPBROADCASTQ: Broadcast a single qword to all 4 qwords in YMM
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.W0 58 /r       VPBROADCASTD ymm1, xmm2/m32
// VEX.256.66.0F38.W0 59 /r       VPBROADCASTQ ymm1, xmm2/m64

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPBROADCASTD Tests - Broadcast Dword to 8 Dwords (256-bit)
// ============================================================================

#[test]
fn test_vpbroadcastd_ymm0_xmm1() {
    let mut emu = emu64();
    // VPBROADCASTD YMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x58, 0xc1, // VPBROADCASTD YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x58, 0xdc, // VPBROADCASTD YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x58, 0xf7, // VPBROADCASTD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x58, 0xca, // VPBROADCASTD YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x58, 0xe5, // VPBROADCASTD YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x58, 0xf8, // VPBROADCASTD YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm0_mem() {
    let mut emu = emu64();
    // VPBROADCASTD YMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x00, // VPBROADCASTD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x18, // VPBROADCASTD YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAA, 0xAA, 0xAA, 0xAA, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x30, // VPBROADCASTD YMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x55, 0x55, 0x55, 0x55, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm9_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x58, 0x08, // VPBROADCASTD YMM9, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x12, 0x34, 0x56, 0x78, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x58, 0x20, // VPBROADCASTD YMM12, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x7F, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPBROADCASTQ Tests - Broadcast Qword to 4 Qwords (256-bit)
// ============================================================================

#[test]
fn test_vpbroadcastq_ymm0_xmm1() {
    let mut emu = emu64();
    // VPBROADCASTQ YMM0, XMM1
    let code = [
        0xc4, 0xe2, 0x7d, 0x59, 0xc1, // VPBROADCASTQ YMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm3_xmm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x59, 0xdc, // VPBROADCASTQ YMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm6_xmm7() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x59, 0xf7, // VPBROADCASTQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm9_xmm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x59, 0xca, // VPBROADCASTQ YMM9, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm12_xmm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x59, 0xe5, // VPBROADCASTQ YMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm15_xmm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x59, 0xf8, // VPBROADCASTQ YMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm0_mem() {
    let mut emu = emu64();
    // VPBROADCASTQ YMM0, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x00, // VPBROADCASTQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x18, // VPBROADCASTQ YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x30, // VPBROADCASTQ YMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm9_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x59, 0x08, // VPBROADCASTQ YMM9, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x7d, 0x59, 0x20, // VPBROADCASTQ YMM12, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vpbroadcastd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x58, 0xc1, // VPBROADCASTD YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x58, 0xd3, // VPBROADCASTD YMM2, XMM3
        0xc4, 0xe2, 0x7d, 0x58, 0xe5, // VPBROADCASTD YMM4, XMM5
        0xc4, 0xe2, 0x7d, 0x58, 0xf7, // VPBROADCASTD YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x59, 0xc1, // VPBROADCASTQ YMM0, XMM1
        0xc4, 0xe2, 0x7d, 0x59, 0xd3, // VPBROADCASTQ YMM2, XMM3
        0xc4, 0xe2, 0x7d, 0x59, 0xe5, // VPBROADCASTQ YMM4, XMM5
        0xc4, 0xe2, 0x7d, 0x59, 0xf7, // VPBROADCASTQ YMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x58, 0xc1, // VPBROADCASTD YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x58, 0xd3, // VPBROADCASTD YMM10, XMM11
        0xc4, 0x42, 0x7d, 0x58, 0xe5, // VPBROADCASTD YMM12, XMM13
        0xc4, 0x42, 0x7d, 0x58, 0xf7, // VPBROADCASTD YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x7d, 0x59, 0xc1, // VPBROADCASTQ YMM8, XMM9
        0xc4, 0x42, 0x7d, 0x59, 0xd3, // VPBROADCASTQ YMM10, XMM11
        0xc4, 0x42, 0x7d, 0x59, 0xe5, // VPBROADCASTQ YMM12, XMM13
        0xc4, 0x42, 0x7d, 0x59, 0xf7, // VPBROADCASTQ YMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_zero_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x00, // VPBROADCASTD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_zero_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x00, // VPBROADCASTQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_max_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x00, // VPBROADCASTD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_max_value() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x00, // VPBROADCASTQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_chain() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x00, // VPBROADCASTD YMM0, [RAX]
        0xc4, 0xe2, 0x7d, 0x58, 0x10, // VPBROADCASTD YMM2, [RAX]
        0xc4, 0xe2, 0x7d, 0x58, 0x20, // VPBROADCASTD YMM4, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0xAB, 0xCD, 0xEF, 0x01, 0x00, 0x00, 0x00, 0x00];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_chain() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x00, // VPBROADCASTQ YMM0, [RAX]
        0xc4, 0xe2, 0x7d, 0x59, 0x10, // VPBROADCASTQ YMM2, [RAX]
        0xc4, 0xe2, 0x7d, 0x59, 0x20, // VPBROADCASTQ YMM4, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_signed_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x00, // VPBROADCASTD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00]; // -2147483648
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_signed_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x00, // VPBROADCASTQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, // -9223372036854775808
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_mem_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x40, 0x10, // VPBROADCASTD YMM0, [RAX+16]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    data[16] = 0xBE;
    data[17] = 0xBA;
    data[18] = 0xFE;
    data[19] = 0xCA;
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_mem_offset() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x40, 0x10, // VPBROADCASTQ YMM0, [RAX+16]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    data[16] = 0xBE;
    data[17] = 0xBA;
    data[18] = 0xFE;
    data[19] = 0xCA;
    data[20] = 0xDE;
    data[21] = 0xAD;
    data[22] = 0xBE;
    data[23] = 0xEF;
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastd_power_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x58, 0x00, // VPBROADCASTD YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00]; // 268435456
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpbroadcastq_power_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x7d, 0x59, 0x00, // VPBROADCASTQ YMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // 4611686018427387904
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
