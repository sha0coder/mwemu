use crate::*;

// VPSRAVD - Variable Bit Shift Right Arithmetic (AVX2)
//
// Performs SIMD arithmetic right shift with independent shift counts per element.
// Each element in the destination is shifted right by the corresponding count
// from the second source operand. The sign bit is propagated (arithmetic shift).
//
// VPSRAVD: Shift 8 packed dword integers (32-bit each) right arithmetic with variable counts
//
// Note: There is no VPSRAVQ in AVX2 (only in AVX-512)
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.W0 46 /r       VPSRAVD ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPSRAVD Tests - 8x Dword Variable Shift Right Arithmetic (256-bit)
// ============================================================================

#[test]
fn test_vpsravd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPSRAVD YMM0, YMM1, YMM2 - variable shift right arithmetic
    let code = [
        0xc4, 0xe2, 0x75, 0x46, 0xc2, // VPSRAVD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x46, 0xdd, // VPSRAVD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x45, 0x46, 0xf0, // VPSRAVD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x46, 0xcb, // VPSRAVD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x46, 0xe6, // VPSRAVD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x46, 0xf9, // VPSRAVD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSRAVD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
        0x06, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm5_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x4d, 0x46, 0x28, // VPSRAVD YMM5, YMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_ymm11_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x1d, 0x46, 0x18, // VPSRAVD YMM11, YMM12, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vpsravd_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x46, 0xc2, // VPSRAVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x46, 0xd3, // VPSRAVD YMM2, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x3d, 0x46, 0xc1, // VPSRAVD YMM8, YMM8, YMM9
        0xc4, 0x42, 0x15, 0x46, 0xd5, // VPSRAVD YMM10, YMM13, YMM13
        0xc4, 0x42, 0x05, 0x46, 0xff, // VPSRAVD YMM15, YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x46, 0xc2, // VPSRAVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x5d, 0x46, 0xdc, // VPSRAVD YMM3, YMM4, YMM4
        0xc4, 0xe2, 0x4d, 0x46, 0xf7, // VPSRAVD YMM6, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_same_src_dst() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x46, 0xc1, // VPSRAVD YMM0, YMM0, YMM1
        0xc4, 0xe2, 0x75, 0x46, 0xd0, // VPSRAVD YMM2, YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_zero_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // All zero shift counts
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_max_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_overflow_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x20, 0x00, 0x00, 0x00, 0x21, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00,
        0xFF, 0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_mixed_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_positive_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_negative_values() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_sign_extension() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_shift_by_one() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_shift_byte_boundaries() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
        0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_alternate_signs() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00,
        0x07, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_powers_of_two() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
        0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_consecutive_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x46, 0xc2, // VPSRAVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x46, 0xcb, // VPSRAVD YMM1, YMM2, YMM3
        0xc4, 0xe2, 0x65, 0x46, 0xd4, // VPSRAVD YMM2, YMM3, YMM4
        0xc4, 0xe2, 0x5d, 0x46, 0xdd, // VPSRAVD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_self_shift() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x46, 0xc0, // VPSRAVD YMM0, YMM0, YMM0
        0xc4, 0xe2, 0x6d, 0x46, 0xca, // VPSRAVD YMM1, YMM2, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_max_negative() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0F, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsravd_max_positive() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x46, 0x00, // VPSRAVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x01, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x1F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0F, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
