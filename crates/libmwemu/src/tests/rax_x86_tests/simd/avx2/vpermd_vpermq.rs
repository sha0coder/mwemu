use crate::*;

// VPERMD - Permute Doublewords (AVX2)
// VPERMQ - Permute Quadwords (AVX2)
//
// VPERMD: Permutes 32-bit doublewords from source using indices in control.
// Each dword in the control operand selects a dword from the source (0-7).
// Only bits [2:0] of each control dword are used.
//
// VPERMQ: Permutes 64-bit quadwords from source using immediate byte.
// Bits [1:0] select qword for position 0, [3:2] for position 1,
// [5:4] for position 2, [7:6] for position 3.
//
// Opcodes:
// VEX.256.66.0F38.W0 36 /r          VPERMD ymm1, ymm2, ymm3/m256
// VEX.256.66.0F3A.W1 00 /r ib       VPERMQ ymm1, ymm2/m256, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// VPERMD Tests - Permute Doublewords
// ============================================================================

#[test]
fn test_vpermd_ymm0_ymm1_ymm2_identity() {
    let mut emu = emu64();
    // VPERMD YMM0, YMM1, YMM2 (YMM1 has indices, YMM2 has data)
    let code = [
        0xc4, 0xe2, 0x75, 0x36, 0xc2, // VPERMD YMM0, YMM1, YMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm3_ymm4_ymm5() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x5d, 0x36, 0xdd, // VPERMD YMM3, YMM4, YMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm6_ymm7_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x45, 0x36, 0xf0, // VPERMD YMM6, YMM7, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm2_ymm3_ymm4() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x65, 0x36, 0xd4, // VPERMD YMM2, YMM3, YMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm1_ymm2_ymm3() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x6d, 0x36, 0xcb, // VPERMD YMM1, YMM2, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMD Tests with extended registers
// ============================================================================

#[test]
fn test_vpermd_ymm8_ymm9_ymm10() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x35, 0x36, 0xc2, // VPERMD YMM8, YMM9, YMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm11_ymm12_ymm13() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x1d, 0x36, 0xdd, // VPERMD YMM11, YMM12, YMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm14_ymm15_ymm0() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x62, 0x05, 0x36, 0xf0, // VPERMD YMM14, YMM15, YMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm0_ymm1_ymm15() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc2, 0x75, 0x36, 0xc7, // VPERMD YMM0, YMM1, YMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm15_ymm8_ymm9() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x3d, 0x36, 0xf9, // VPERMD YMM15, YMM8, YMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm9_ymm10_ymm11() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x2d, 0x36, 0xcb, // VPERMD YMM9, YMM10, YMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm10_ymm11_ymm12() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x25, 0x36, 0xd4, // VPERMD YMM10, YMM11, YMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm12_ymm13_ymm14() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x42, 0x15, 0x36, 0xe6, // VPERMD YMM12, YMM13, YMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMD Tests with memory operands
// ============================================================================

#[test]
fn test_vpermd_ymm0_ymm1_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x36, 0x00, // VPERMD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm2_ymm3_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x65, 0x36, 0x10, // VPERMD YMM2, YMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm4_ymm5_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x55, 0x36, 0x20, // VPERMD YMM4, YMM5, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_ymm8_ymm9_mem() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x62, 0x35, 0x36, 0x00, // VPERMD YMM8, YMM9, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i * 2).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMQ Tests - Permute Quadwords (with immediate)
// ============================================================================

#[test]
fn test_vpermq_ymm0_ymm1_identity() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0xE4 (identity: 3,2,1,0)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0xe4, // VPERMQ YMM0, YMM1, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm3_ymm4_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xdc, 0xe4, // VPERMQ YMM3, YMM4, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm6_ymm7_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xf7, 0xe4, // VPERMQ YMM6, YMM7, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_reverse() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x1B (reverse: 0,1,2,3)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x1b, // VPERMQ YMM0, YMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm2_ymm3_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xd3, 0x1b, // VPERMQ YMM2, YMM3, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm5_ymm6_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xee, 0x1b, // VPERMQ YMM5, YMM6, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_broadcast_q0() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x00 (broadcast qword 0)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x00, // VPERMQ YMM0, YMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_broadcast_q1() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x55 (broadcast qword 1)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x55, // VPERMQ YMM0, YMM1, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_broadcast_q2() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0xAA (broadcast qword 2)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0xaa, // VPERMQ YMM0, YMM1, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_broadcast_q3() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0xFF (broadcast qword 3)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0xff, // VPERMQ YMM0, YMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_swap_pairs() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x4E (swap pairs: 1,0,3,2)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x4e, // VPERMQ YMM0, YMM1, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_swap_halves() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0xB1 (swap halves: 2,3,0,1)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0xb1, // VPERMQ YMM0, YMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_imm_0x27() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x27 (0,2,1,3)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x27, // VPERMQ YMM0, YMM1, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_imm_0x39() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x39 (0,3,2,1)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x39, // VPERMQ YMM0, YMM1, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_imm_0x72() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x72 (1,3,0,2)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x72, // VPERMQ YMM0, YMM1, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_imm_0x93() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0x93 (2,1,0,3)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x93, // VPERMQ YMM0, YMM1, 0x93
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_imm_0xC6() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0xC6 (3,0,1,2)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0xc6, // VPERMQ YMM0, YMM1, 0xC6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm1_imm_0xD8() {
    let mut emu = emu64();
    // VPERMQ YMM0, YMM1, 0xD8 (3,1,2,0)
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0xd8, // VPERMQ YMM0, YMM1, 0xD8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMQ Tests with extended registers
// ============================================================================

#[test]
fn test_vpermq_ymm8_ymm9_identity() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0xfd, 0x00, 0xc1, 0xe4, // VPERMQ YMM8, YMM9, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm10_ymm11_reverse() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0xfd, 0x00, 0xd3, 0x1b, // VPERMQ YMM10, YMM11, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm12_ymm13_broadcast() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0xfd, 0x00, 0xe5, 0x00, // VPERMQ YMM12, YMM13, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm14_ymm15_swap() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0xfd, 0x00, 0xf7, 0x4e, // VPERMQ YMM14, YMM15, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm15_ymm8_custom() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0xfd, 0x00, 0xf8, 0x39, // VPERMQ YMM15, YMM8, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm0_ymm15_mix() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xc3, 0xfd, 0x00, 0xc7, 0xb1, // VPERMQ YMM0, YMM15, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm15_ymm0_mix() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x63, 0xfd, 0x00, 0xf8, 0x72, // VPERMQ YMM15, YMM0, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// VPERMQ Tests with memory operands
// ============================================================================

#[test]
fn test_vpermq_ymm0_mem_identity() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0xfd, 0x00, 0x00, 0xe4, // VPERMQ YMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm1_mem_reverse() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0xfd, 0x00, 0x08, 0x1b, // VPERMQ YMM1, [RAX], 0x1B
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i * 2).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm2_mem_broadcast() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0xfd, 0x00, 0x10, 0x00, // VPERMQ YMM2, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm3_mem_swap() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0xfd, 0x00, 0x18, 0x4e, // VPERMQ YMM3, [RAX], 0x4E
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = vec![0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55];
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_ymm8_mem_custom() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0x63, 0xfd, 0x00, 0x00, 0x93, // VPERMQ YMM8, [RAX], 0x93
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data: Vec<u8> = (0..32).map(|i| i as u8).collect();
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Chained operations and edge cases
// ============================================================================

#[test]
fn test_vpermd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x36, 0xc2, // VPERMD YMM0, YMM1, YMM2
        0xc4, 0xe2, 0x7d, 0x36, 0xc3, // VPERMD YMM0, YMM0, YMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_chain_operations() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc1, 0x1b, // VPERMQ YMM0, YMM1, 0x1B
        0xc4, 0xe3, 0xfd, 0x00, 0xc0, 0x1b, // VPERMQ YMM0, YMM0, 0x1B (should restore)
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_same_register() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x75, 0x36, 0xc1, // VPERMD YMM0, YMM1, YMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_same_register() {
    let mut emu = emu64();
    let code = [
        0xc4, 0xe3, 0xfd, 0x00, 0xc0, 0x4e, // VPERMQ YMM0, YMM0, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_all_extended_regs() {
    let mut emu = emu64();
    let code = [
        0xc4, 0x43, 0xfd, 0x00, 0xff, 0x27, // VPERMQ YMM15, YMM15, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermd_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe2, 0x75, 0x36, 0x00, // VPERMD YMM0, YMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_vpermq_mem_unaligned() {
    let mut emu = emu64();
    let code = [0x48, 0xb8];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR + 1).to_le_bytes());
    full_code.extend_from_slice(&[
        0xc4, 0xe3, 0xfd, 0x00, 0x00, 0xe4, // VPERMQ YMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}
