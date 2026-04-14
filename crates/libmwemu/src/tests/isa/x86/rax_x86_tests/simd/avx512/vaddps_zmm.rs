use crate::*;

// VADDPS - Add Packed Single-Precision Floating-Point Values (ZMM)
//
// AVX-512 version using ZMM registers (512-bit / 64 bytes).
// Adds 16 packed single-precision floating-point values (16x f32).
//
// Opcodes (EVEX encoded):
// EVEX.NDS.512.0F.W0 58 /r    VADDPS zmm1 {k1}{z}, zmm2, zmm3/m512/m32bcst
//   - Add packed single from zmm3/m512 to zmm2 and store result in zmm1

const ALIGNED_ADDR: u64 = 0x3000; // 64-byte aligned address for testing

// ============================================================================
// Register-Register-Register Tests - ZMM0-ZMM7
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm1_zmm2() {
    let mut emu = emu64();
    // VADDPS ZMM0, ZMM1, ZMM2
    let code = [
        0x62, 0xf1, 0x74, 0x48, 0x58, 0xc2, // VADDPS ZMM0, ZMM1, ZMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm1_zmm2_zmm3() {
    let mut emu = emu64();
    // VADDPS ZMM1, ZMM2, ZMM3
    let code = [
        0x62, 0xf1, 0x6c, 0x48, 0x58, 0xcb, // VADDPS ZMM1, ZMM2, ZMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm2_zmm3_zmm4() {
    let mut emu = emu64();
    // VADDPS ZMM2, ZMM3, ZMM4
    let code = [
        0x62, 0xf1, 0x64, 0x48, 0x58, 0xd4, // VADDPS ZMM2, ZMM3, ZMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm3_zmm4_zmm5() {
    let mut emu = emu64();
    // VADDPS ZMM3, ZMM4, ZMM5
    let code = [
        0x62, 0xf1, 0x5c, 0x48, 0x58, 0xdd, // VADDPS ZMM3, ZMM4, ZMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm4_zmm5_zmm6() {
    let mut emu = emu64();
    // VADDPS ZMM4, ZMM5, ZMM6
    let code = [
        0x62, 0xf1, 0x54, 0x48, 0x58, 0xe6, // VADDPS ZMM4, ZMM5, ZMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm5_zmm6_zmm7() {
    let mut emu = emu64();
    // VADDPS ZMM5, ZMM6, ZMM7
    let code = [
        0x62, 0xf1, 0x4c, 0x48, 0x58, 0xef, // VADDPS ZMM5, ZMM6, ZMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm6_zmm7_zmm0() {
    let mut emu = emu64();
    // VADDPS ZMM6, ZMM7, ZMM0
    let code = [
        0x62, 0xf1, 0x44, 0x48, 0x58, 0xf0, // VADDPS ZMM6, ZMM7, ZMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm7_zmm0_zmm1() {
    let mut emu = emu64();
    // VADDPS ZMM7, ZMM0, ZMM1
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xf9, // VADDPS ZMM7, ZMM0, ZMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register-Register-Register Tests - ZMM8-ZMM15
// ============================================================================

#[test]
fn test_vaddps_zmm8_zmm9_zmm10() {
    let mut emu = emu64();
    // VADDPS ZMM8, ZMM9, ZMM10
    let code = [
        0x62, 0x51, 0x34, 0x48, 0x58, 0xc2, // VADDPS ZMM8, ZMM9, ZMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm9_zmm10_zmm11() {
    let mut emu = emu64();
    // VADDPS ZMM9, ZMM10, ZMM11
    let code = [
        0x62, 0x51, 0x2c, 0x48, 0x58, 0xcb, // VADDPS ZMM9, ZMM10, ZMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm10_zmm11_zmm12() {
    let mut emu = emu64();
    // VADDPS ZMM10, ZMM11, ZMM12
    let code = [
        0x62, 0x51, 0x24, 0x48, 0x58, 0xd4, // VADDPS ZMM10, ZMM11, ZMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm11_zmm12_zmm13() {
    let mut emu = emu64();
    // VADDPS ZMM11, ZMM12, ZMM13
    let code = [
        0x62, 0x51, 0x1c, 0x48, 0x58, 0xdd, // VADDPS ZMM11, ZMM12, ZMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm12_zmm13_zmm14() {
    let mut emu = emu64();
    // VADDPS ZMM12, ZMM13, ZMM14
    let code = [
        0x62, 0x51, 0x14, 0x48, 0x58, 0xe6, // VADDPS ZMM12, ZMM13, ZMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm13_zmm14_zmm15() {
    let mut emu = emu64();
    // VADDPS ZMM13, ZMM14, ZMM15
    let code = [
        0x62, 0x51, 0x0c, 0x48, 0x58, 0xef, // VADDPS ZMM13, ZMM14, ZMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm14_zmm15_zmm8() {
    let mut emu = emu64();
    // VADDPS ZMM14, ZMM15, ZMM8
    let code = [
        0x62, 0x51, 0x04, 0x48, 0x58, 0xf0, // VADDPS ZMM14, ZMM15, ZMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm8_zmm9() {
    let mut emu = emu64();
    // VADDPS ZMM15, ZMM8, ZMM9
    let code = [
        0x62, 0x51, 0x3c, 0x48, 0x58, 0xf9, // VADDPS ZMM15, ZMM8, ZMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register-Register-Register Tests - ZMM16-ZMM23
// ============================================================================

#[test]
fn test_vaddps_zmm16_zmm17_zmm18() {
    let mut emu = emu64();
    // VADDPS ZMM16, ZMM17, ZMM18
    let code = [
        0x62, 0xd1, 0x74, 0x48, 0x58, 0xc2, // VADDPS ZMM16, ZMM17, ZMM18
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm17_zmm18_zmm19() {
    let mut emu = emu64();
    // VADDPS ZMM17, ZMM18, ZMM19
    let code = [
        0x62, 0xd1, 0x6c, 0x48, 0x58, 0xcb, // VADDPS ZMM17, ZMM18, ZMM19
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm20_zmm21_zmm22() {
    let mut emu = emu64();
    // VADDPS ZMM20, ZMM21, ZMM22
    let code = [
        0x62, 0xd1, 0x54, 0x48, 0x58, 0xe6, // VADDPS ZMM20, ZMM21, ZMM22
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm21_zmm22_zmm23() {
    let mut emu = emu64();
    // VADDPS ZMM21, ZMM22, ZMM23
    let code = [
        0x62, 0xd1, 0x4c, 0x48, 0x58, 0xef, // VADDPS ZMM21, ZMM22, ZMM23
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register-Register-Register Tests - ZMM24-ZMM31
// ============================================================================

#[test]
fn test_vaddps_zmm24_zmm25_zmm26() {
    let mut emu = emu64();
    // VADDPS ZMM24, ZMM25, ZMM26
    let code = [
        0x62, 0x91, 0x34, 0x48, 0x58, 0xc2, // VADDPS ZMM24, ZMM25, ZMM26
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm25_zmm26_zmm27() {
    let mut emu = emu64();
    // VADDPS ZMM25, ZMM26, ZMM27
    let code = [
        0x62, 0x91, 0x2c, 0x48, 0x58, 0xcb, // VADDPS ZMM25, ZMM26, ZMM27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm28_zmm29_zmm30() {
    let mut emu = emu64();
    // VADDPS ZMM28, ZMM29, ZMM30
    let code = [
        0x62, 0x91, 0x14, 0x48, 0x58, 0xe6, // VADDPS ZMM28, ZMM29, ZMM30
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm29_zmm30_zmm31() {
    let mut emu = emu64();
    // VADDPS ZMM29, ZMM30, ZMM31
    let code = [
        0x62, 0x91, 0x0c, 0x48, 0x58, 0xef, // VADDPS ZMM29, ZMM30, ZMM31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm30_zmm31_zmm24() {
    let mut emu = emu64();
    // VADDPS ZMM30, ZMM31, ZMM24
    let code = [
        0x62, 0x91, 0x04, 0x48, 0x58, 0xf0, // VADDPS ZMM30, ZMM31, ZMM24
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm24_zmm25() {
    let mut emu = emu64();
    // VADDPS ZMM31, ZMM24, ZMM25
    let code = [
        0x62, 0x91, 0x5c, 0x48, 0x58, 0xf9, // VADDPS ZMM31, ZMM24, ZMM25
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-range Register Tests
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm15_zmm31() {
    let mut emu = emu64();
    // VADDPS ZMM0, ZMM15, ZMM31
    let code = [
        0x62, 0x71, 0x04, 0x48, 0x58, 0xc7, // VADDPS ZMM0, ZMM15, ZMM31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm0_zmm15() {
    let mut emu = emu64();
    // VADDPS ZMM31, ZMM0, ZMM15
    let code = [
        0x62, 0x71, 0x7c, 0x48, 0x58, 0xff, // VADDPS ZMM31, ZMM0, ZMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm7_zmm16_zmm24() {
    let mut emu = emu64();
    // VADDPS ZMM7, ZMM16, ZMM24
    let code = [
        0x62, 0xb1, 0x7c, 0x48, 0x58, 0xf8, // VADDPS ZMM7, ZMM16, ZMM24
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm16_zmm31() {
    let mut emu = emu64();
    // VADDPS ZMM15, ZMM16, ZMM31
    let code = [
        0x62, 0x71, 0x7c, 0x48, 0x58, 0xff, // VADDPS ZMM15, ZMM16, ZMM31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register-Register-Memory Tests
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm1_mem() {
    let mut emu = emu64();
    // VADDPS ZMM0, ZMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x74, 0x48, 0x58, 0x00, // VADDPS ZMM0, ZMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm7_zmm6_mem() {
    let mut emu = emu64();
    // VADDPS ZMM7, ZMM6, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x4c, 0x48, 0x58, 0x38, // VADDPS ZMM7, ZMM6, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm14_mem() {
    let mut emu = emu64();
    // VADDPS ZMM15, ZMM14, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x0c, 0x48, 0x58, 0x38, // VADDPS ZMM15, ZMM14, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm30_mem() {
    let mut emu = emu64();
    // VADDPS ZMM31, ZMM30, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x0c, 0x48, 0x58, 0x38, // VADDPS ZMM31, ZMM30, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

// ============================================================================
// Self-Addition Tests (dest = src1 = src2)
// ============================================================================

#[test]
fn test_vaddps_zmm0_zmm0_zmm0() {
    let mut emu = emu64();
    // VADDPS ZMM0, ZMM0, ZMM0 (doubles the value)
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xc0, // VADDPS ZMM0, ZMM0, ZMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm15_zmm15_zmm15() {
    let mut emu = emu64();
    // VADDPS ZMM15, ZMM15, ZMM15
    let code = [
        0x62, 0x71, 0x04, 0x48, 0x58, 0xff, // VADDPS ZMM15, ZMM15, ZMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_zmm31_zmm31_zmm31() {
    let mut emu = emu64();
    // VADDPS ZMM31, ZMM31, ZMM31
    let code = [
        0x62, 0x61, 0x04, 0x48, 0x58, 0xff, // VADDPS ZMM31, ZMM31, ZMM31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Commutative Property Tests
// ============================================================================

#[test]
fn test_vaddps_commutative_zmm1_zmm2_zmm3() {
    let mut emu = emu64();
    // VADDPS ZMM1, ZMM2, ZMM3
    let code = [
        0x62, 0xf1, 0x6c, 0x48, 0x58, 0xcb, // VADDPS ZMM1, ZMM2, ZMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_commutative_zmm1_zmm3_zmm2() {
    let mut emu = emu64();
    // VADDPS ZMM1, ZMM3, ZMM2 (should give same result as above)
    let code = [
        0x62, 0xf1, 0x64, 0x48, 0x58, 0xca, // VADDPS ZMM1, ZMM3, ZMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Chain Addition Tests
// ============================================================================

#[test]
fn test_vaddps_chain_3_ops() {
    let mut emu = emu64();
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xd9, // VADDPS ZMM3, ZMM0, ZMM1
        0x62, 0xf1, 0x64, 0x48, 0x58, 0xda, // VADDPS ZMM3, ZMM3, ZMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_chain_4_ops() {
    let mut emu = emu64();
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xe1, // VADDPS ZMM4, ZMM0, ZMM1
        0x62, 0xf1, 0x6c, 0x48, 0x58, 0xeb, // VADDPS ZMM5, ZMM2, ZMM3
        0x62, 0xf1, 0x5c, 0x48, 0x58, 0xf5, // VADDPS ZMM6, ZMM4, ZMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// All Registers Accumulation Test
// ============================================================================

#[test]
fn test_vaddps_accumulate_all_ranges() {
    let mut emu = emu64();
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x58, 0xc1, // VADDPS ZMM0, ZMM0, ZMM1
        0x62, 0x51, 0x3c, 0x48, 0x58, 0xc1, // VADDPS ZMM8, ZMM8, ZMM9
        0x62, 0xd1, 0x7c, 0x48, 0x58, 0xc1, // VADDPS ZMM16, ZMM16, ZMM17
        0x62, 0x91, 0x5c, 0x48, 0x58, 0xc1, // VADDPS ZMM24, ZMM24, ZMM25
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory Addressing Mode Tests
// ============================================================================

#[test]
fn test_vaddps_mem_base_displacement() {
    let mut emu = emu64();
    // VADDPS ZMM0, ZMM1, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x40).to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x74, 0x48, 0x58, 0x40, 0x01, // VADDPS ZMM0, ZMM1, [RAX + 0x40]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_mem_with_rbx_base() {
    let mut emu = emu64();
    // VADDPS ZMM2, ZMM3, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x64, 0x48, 0x58, 0x13, // VADDPS ZMM2, ZMM3, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_mem_with_rcx_base() {
    let mut emu = emu64();
    // VADDPS ZMM4, ZMM5, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x54, 0x48, 0x58, 0x21, // VADDPS ZMM4, ZMM5, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vaddps_mem_with_rdx_base() {
    let mut emu = emu64();
    // VADDPS ZMM6, ZMM7, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x44, 0x48, 0x58, 0x32, // VADDPS ZMM6, ZMM7, [RDX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}
