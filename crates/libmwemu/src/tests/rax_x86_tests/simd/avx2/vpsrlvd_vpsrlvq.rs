use crate::*;

// VPSRLVD/VPSRLVQ - Variable Bit Shift Right Logical (AVX2)
//
// Performs SIMD logical right shift with independent shift counts per element.
// Each element in the destination is shifted right by the corresponding count
// from the second source operand. Zeros are shifted in from the left.
//
// VPSRLVD: Shift 8 packed dword integers (32-bit each) right with variable counts
// VPSRLVQ: Shift 4 packed qword integers (64-bit each) right with variable counts
//
// Opcodes (AVX2 - 256-bit YMM):
// VEX.256.66.0F38.W0 45 /r       VPSRLVD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F38.W1 45 /r       VPSRLVQ ymm1, ymm2, ymm3/m256

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPSRLVD Tests - 8x Dword Variable Shift Right Logical (256-bit)
// ============================================================================

#[test]
fn test_vpsrlvd_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPSRLVD YMM0, YMM1, YMM2 - variable shift right logical
    let code = [
        0xc4, 0xe2, 0x75, 0x45, 0xc2, // VPSRLVD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x45, 0xdd, // VPSRLVD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x45, 0x45, 0xf0, // VPSRLVD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x45, 0xcb, // VPSRLVD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x45, 0xe6, // VPSRLVD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x7d, 0x45, 0xf9, // VPSRLVD YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSRLVD YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x45, 0x00, // VPSRLVD YMM0, YMM1, [RAX]
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
fn test_vpsrlvd_ymm5_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x4d, 0x45, 0x28, // VPSRLVD YMM5, YMM6, [RAX]
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
fn test_vpsrlvd_ymm11_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x1d, 0x45, 0x18, // VPSRLVD YMM11, YMM12, [RAX]
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
// VPSRLVQ Tests - 4x Qword Variable Shift Right Logical (256-bit)
// ============================================================================

#[test]
fn test_vpsrlvq_ymm0_ymm1_ymm2() {
    let mut emu = emu64();
    // VPSRLVQ YMM0, YMM1, YMM2 - variable shift right logical
    let code = [
        0xc4, 0xe2, 0xf5, 0x45, 0xc2, // VPSRLVQ YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xdd, 0x45, 0xdd, // VPSRLVQ YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xc5, 0x45, 0xf0, // VPSRLVQ YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xad, 0x45, 0xcb, // VPSRLVQ YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x95, 0x45, 0xe6, // VPSRLVQ YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_ymm15_ymm0_ymm1() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0xfd, 0x45, 0xf9, // VPSRLVQ YMM15, YMM0, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_ymm0_ymm1_mem() {
    let mut emu = emu64();
    // VPSRLVQ YMM0, YMM1, [memory]
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x45, 0x00, // VPSRLVQ YMM0, YMM1, [RAX]
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
fn test_vpsrlvq_ymm5_ymm6_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xcd, 0x45, 0x28, // VPSRLVQ YMM5, YMM6, [RAX]
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
fn test_vpsrlvq_ymm11_ymm12_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x9d, 0x45, 0x18, // VPSRLVQ YMM11, YMM12, [RAX]
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
fn test_vpsrlvd_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x45, 0xc2, // VPSRLVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x6d, 0x45, 0xd3, // VPSRLVD YMM2, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_chain() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x45, 0xc2, // VPSRLVQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0xed, 0x45, 0xd3, // VPSRLVQ YMM2, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x3d, 0x45, 0xc1, // VPSRLVD YMM8, YMM8, YMM9
        0xc4, 0x42, 0x15, 0x45, 0xd5, // VPSRLVD YMM10, YMM13, YMM13
        0xc4, 0x42, 0x05, 0x45, 0xff, // VPSRLVD YMM15, YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0xbd, 0x45, 0xc1, // VPSRLVQ YMM8, YMM8, YMM9
        0xc4, 0x42, 0x95, 0x45, 0xd5, // VPSRLVQ YMM10, YMM13, YMM13
        0xc4, 0x42, 0x85, 0x45, 0xff, // VPSRLVQ YMM15, YMM15, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x45, 0xc2, // VPSRLVD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x5d, 0x45, 0xdc, // VPSRLVD YMM3, YMM4, YMM4
        0xc4, 0xe2, 0x4d, 0x45, 0xf7, // VPSRLVD YMM6, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_all_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf5, 0x45, 0xc2, // VPSRLVQ YMM0, YMM1, YMM2
        0xc4, 0xe2, 0xdd, 0x45, 0xdc, // VPSRLVQ YMM3, YMM4, YMM4
        0xc4, 0xe2, 0xcd, 0x45, 0xf7, // VPSRLVQ YMM6, YMM6, YMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_same_src_dst() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x7d, 0x45, 0xc1, // VPSRLVD YMM0, YMM0, YMM1
        0xc4, 0xe2, 0x75, 0x45, 0xd0, // VPSRLVD YMM2, YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_same_src_dst() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xfd, 0x45, 0xc1, // VPSRLVQ YMM0, YMM0, YMM1
        0xc4, 0xe2, 0xf5, 0x45, 0xd0, // VPSRLVQ YMM2, YMM1, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_zero_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x45, 0x00, // VPSRLVD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // All zero shift counts
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvq_zero_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x45, 0x00, // VPSRLVQ YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // All zero shift counts
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpsrlvd_max_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x45, 0x00, // VPSRLVD YMM0, YMM1, [RAX]
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
fn test_vpsrlvq_max_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x45, 0x00, // VPSRLVQ YMM0, YMM1, [RAX]
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
fn test_vpsrlvd_overflow_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x45, 0x00, // VPSRLVD YMM0, YMM1, [RAX]
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
fn test_vpsrlvq_overflow_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x45, 0x00, // VPSRLVQ YMM0, YMM1, [RAX]
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
fn test_vpsrlvd_mixed_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x45, 0x00, // VPSRLVD YMM0, YMM1, [RAX]
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
fn test_vpsrlvq_mixed_shifts() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0xf5, 0x45, 0x00, // VPSRLVQ YMM0, YMM1, [RAX]
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
