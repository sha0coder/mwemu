use crate::*;

// VPSLLVD/VPSLLVQ - Variable Bit Shift Left Logical (AVX2)
//
// Performs SIMD logical left shift with independent shift counts per element.
// Each element in the destination is shifted left by the corresponding count
// from the second source operand.
//
// VPSLLVD: Shift 8 packed dword integers (32-bit each) left with variable counts
// VPSLLVQ: Shift 4 packed qword integers (64-bit each) left with variable counts
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.W0 47 /r       VPSLLVD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.W1 47 /r       VPSLLVQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPSLLVD Tests - 8x Dword Variable Shift Left (256-bit)
// ============================================================================

#[test]
fn test_vpsllvd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPSLLVD YMM0, YMM1, YMM2 - variable shift left
    let code = [
        0xc4, 0xe2, 0x75, 0x47, 0xc2, // VPSLLVD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x47, 0xdd, // VPSLLVD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x45, 0x47, 0xf0, // VPSLLVD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x47, 0xcb, // VPSLLVD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x47, 0xe6, // VPSLLVD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x47, 0xf9, // VPSLLVD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSLLVD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x47, 0x00, // VPSLLVD YMM0, YMM1, [RAX]
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
fn test_vpsllvd_ymm5_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x4d, 0x47, 0x28, // VPSLLVD YMM5, YMM6, [RAX]
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
fn test_vpsllvd_ymm11_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x1d, 0x47, 0x18, // VPSLLVD YMM11, YMM12, [RAX]
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
// VPSLLVQ Tests - 4x Qword Variable Shift Left (256-bit)
// ============================================================================

#[test]
fn test_vpsllvq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPSLLVQ YMM0, YMM1, YMM2 - variable shift left
    let code = [
        0xc4, 0xe2, 0xf5, 0x47, 0xc2, // VPSLLVQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xdd, 0x47, 0xdd, // VPSLLVQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc5, 0x47, 0xf0, // VPSLLVQ YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xad, 0x47, 0xcb, // VPSLLVQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x95, 0x47, 0xe6, // VPSLLVQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xfd, 0x47, 0xf9, // VPSLLVQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSLLVQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x47, 0x00, // VPSLLVQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm5_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xcd, 0x47, 0x28, // VPSLLVQ YMM5, YMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_ymm11_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x9d, 0x47, 0x18, // VPSLLVQ YMM11, YMM12, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Comprehensive tests
// ============================================================================

#[test]
fn test_vpsllvd_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x47, 0xc2, // VPSLLVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x47, 0xd3, // VPSLLVD YMM2, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x47, 0xc2, // VPSLLVQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0xed, 0x47, 0xd3, // VPSLLVQ YMM2, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x3d, 0x47, 0xc1, // VPSLLVD YMM8, YMM8, YMM9
        0xc4, 0x42, 0x15, 0x47, 0xd5, // VPSLLVD YMM10, YMM13, YMM13
        0xc4, 0x42, 0x05, 0x47, 0xff, // VPSLLVD YMM15, YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xbd, 0x47, 0xc1, // VPSLLVQ YMM8, YMM8, YMM9
        0xc4, 0x42, 0x95, 0x47, 0xd5, // VPSLLVQ YMM10, YMM13, YMM13
        0xc4, 0x42, 0x85, 0x47, 0xff, // VPSLLVQ YMM15, YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x47, 0xc2, // VPSLLVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x5d, 0x47, 0xdc, // VPSLLVD YMM3, YMM4, YMM4
        0xc4, 0xe2, 0x4d, 0x47, 0xf7, // VPSLLVD YMM6, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x47, 0xc2, // VPSLLVQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0xdd, 0x47, 0xdc, // VPSLLVQ YMM3, YMM4, YMM4
        0xc4, 0xe2, 0xcd, 0x47, 0xf7, // VPSLLVQ YMM6, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_same_src_dst() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x47, 0xc1, // VPSLLVD YMM0, YMM0, YMM1
        0xc4, 0xe2, 0x75, 0x47, 0xd0, // VPSLLVD YMM2, YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_same_src_dst() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xfd, 0x47, 0xc1, // VPSLLVQ YMM0, YMM0, YMM1
        0xc4, 0xe2, 0xf5, 0x47, 0xd0, // VPSLLVQ YMM2, YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_zero_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x47, 0x00, // VPSLLVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // All zero shift counts
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvq_zero_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x47, 0x00, // VPSLLVQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // All zero shift counts
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_max_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x47, 0x00, // VPSLLVD YMM0, YMM1, [RAX]
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
fn test_vpsllvq_max_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x47, 0x00, // VPSLLVQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_overflow_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x47, 0x00, // VPSLLVD YMM0, YMM1, [RAX]
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
fn test_vpsllvq_overflow_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x47, 0x00, // VPSLLVQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsllvd_mixed_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x47, 0x00, // VPSLLVD YMM0, YMM1, [RAX]
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
fn test_vpsllvq_mixed_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x47, 0x00, // VPSLLVQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
