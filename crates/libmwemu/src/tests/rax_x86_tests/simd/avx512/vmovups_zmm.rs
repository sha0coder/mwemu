use crate::*;

// VMOVUPS - Move Unaligned Packed Single-Precision Floating-Point Values (ZMM)
//
// AVX-512 version using ZMM registers (512-bit / 64 bytes).
// Moves 16 single-precision floating-point values (16x f32).
// Memory operands do NOT need to be aligned - can use any byte alignment.
//
// Opcodes (EVEX encoded):
// EVEX.512.0F.W0 10 /r    VMOVUPS zmm1 {k1}{z}, zmm2/m512    - Move unaligned packed single from zmm2/m512 to zmm1
// EVEX.512.0F.W0 11 /r    VMOVUPS zmm2/m512 {k1}{z}, zmm1    - Move unaligned packed single from zmm1 to zmm2/m512

const ALIGNED_ADDR: u64 = 0x3000;     // 64-byte aligned
const UNALIGNED_ADDR: u64 = 0x3008;   // Intentionally unaligned (8-byte offset)
const UNALIGNED_ADDR2: u64 = 0x3011;  // Intentionally unaligned (17-byte offset)

// ============================================================================
// Register to Register Tests - ZMM0-ZMM7
// ============================================================================

#[test]
fn test_vmovups_zmm0_to_zmm1() {
    let mut emu = emu64();
    // VMOVUPS ZMM1, ZMM0
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xc8, // VMOVUPS ZMM1, ZMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm1_to_zmm2() {
    let mut emu = emu64();
    // VMOVUPS ZMM2, ZMM1
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xd1, // VMOVUPS ZMM2, ZMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm2_to_zmm3() {
    let mut emu = emu64();
    // VMOVUPS ZMM3, ZMM2
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xda, // VMOVUPS ZMM3, ZMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm3_to_zmm4() {
    let mut emu = emu64();
    // VMOVUPS ZMM4, ZMM3
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xe3, // VMOVUPS ZMM4, ZMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm4_to_zmm5() {
    let mut emu = emu64();
    // VMOVUPS ZMM5, ZMM4
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xec, // VMOVUPS ZMM5, ZMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm5_to_zmm6() {
    let mut emu = emu64();
    // VMOVUPS ZMM6, ZMM5
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xf5, // VMOVUPS ZMM6, ZMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm6_to_zmm7() {
    let mut emu = emu64();
    // VMOVUPS ZMM7, ZMM6
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xfe, // VMOVUPS ZMM7, ZMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register to Register Tests - ZMM8-ZMM15
// ============================================================================

#[test]
fn test_vmovups_zmm8_to_zmm9() {
    let mut emu = emu64();
    // VMOVUPS ZMM9, ZMM8
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xc8, // VMOVUPS ZMM9, ZMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm9_to_zmm10() {
    let mut emu = emu64();
    // VMOVUPS ZMM10, ZMM9
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xd1, // VMOVUPS ZMM10, ZMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm10_to_zmm11() {
    let mut emu = emu64();
    // VMOVUPS ZMM11, ZMM10
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xda, // VMOVUPS ZMM11, ZMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm11_to_zmm12() {
    let mut emu = emu64();
    // VMOVUPS ZMM12, ZMM11
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xe3, // VMOVUPS ZMM12, ZMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm12_to_zmm13() {
    let mut emu = emu64();
    // VMOVUPS ZMM13, ZMM12
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xec, // VMOVUPS ZMM13, ZMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm13_to_zmm14() {
    let mut emu = emu64();
    // VMOVUPS ZMM14, ZMM13
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xf5, // VMOVUPS ZMM14, ZMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm14_to_zmm15() {
    let mut emu = emu64();
    // VMOVUPS ZMM15, ZMM14
    let code = [
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xfe, // VMOVUPS ZMM15, ZMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register to Register Tests - ZMM16-ZMM31
// ============================================================================

#[test]
fn test_vmovups_zmm16_to_zmm17() {
    let mut emu = emu64();
    // VMOVUPS ZMM17, ZMM16
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x10, 0xc8, // VMOVUPS ZMM17, ZMM16
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm20_to_zmm21() {
    let mut emu = emu64();
    // VMOVUPS ZMM21, ZMM20
    let code = [
        0x62, 0xd1, 0x7c, 0x48, 0x10, 0xec, // VMOVUPS ZMM21, ZMM20
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm24_to_zmm25() {
    let mut emu = emu64();
    // VMOVUPS ZMM25, ZMM24
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x10, 0xc8, // VMOVUPS ZMM25, ZMM24
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm28_to_zmm29() {
    let mut emu = emu64();
    // VMOVUPS ZMM29, ZMM28
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x10, 0xec, // VMOVUPS ZMM29, ZMM28
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm30_to_zmm31() {
    let mut emu = emu64();
    // VMOVUPS ZMM31, ZMM30
    let code = [
        0x62, 0x91, 0x7c, 0x48, 0x10, 0xfe, // VMOVUPS ZMM31, ZMM30
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Cross-range Register Tests
// ============================================================================

#[test]
fn test_vmovups_zmm0_to_zmm31() {
    let mut emu = emu64();
    // VMOVUPS ZMM31, ZMM0
    let code = [
        0x62, 0x61, 0x7c, 0x48, 0x10, 0xf8, // VMOVUPS ZMM31, ZMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm31_to_zmm0() {
    let mut emu = emu64();
    // VMOVUPS ZMM0, ZMM31
    let code = [
        0x62, 0x61, 0x7c, 0x48, 0x10, 0xc7, // VMOVUPS ZMM0, ZMM31
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm15_to_zmm16() {
    let mut emu = emu64();
    // VMOVUPS ZMM16, ZMM15
    let code = [
        0x62, 0xe1, 0x7c, 0x48, 0x10, 0xc7, // VMOVUPS ZMM16, ZMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests - Aligned
// ============================================================================

#[test]
fn test_vmovups_aligned_mem_to_zmm0() {
    let mut emu = emu64();
    // VMOVUPS ZMM0, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x00, // VMOVUPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_aligned_mem_to_zmm15() {
    let mut emu = emu64();
    // VMOVUPS ZMM15, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM15, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFFu8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_aligned_mem_to_zmm31() {
    let mut emu = emu64();
    // VMOVUPS ZMM31, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM31, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAAu8; 64]);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests - Unaligned (8-byte offset)
// ============================================================================

#[test]
fn test_vmovups_unaligned8_mem_to_zmm0() {
    let mut emu = emu64();
    // VMOVUPS ZMM0, [unaligned_addr] (8-byte misalignment)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x00, // VMOVUPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x11u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_unaligned8_mem_to_zmm7() {
    let mut emu = emu64();
    // VMOVUPS ZMM7, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x22u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_unaligned8_mem_to_zmm15() {
    let mut emu = emu64();
    // VMOVUPS ZMM15, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM15, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x33u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_unaligned8_mem_to_zmm31() {
    let mut emu = emu64();
    // VMOVUPS ZMM31, [unaligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM31, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x44u8; 64]);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests - Unaligned (17-byte offset - odd alignment)
// ============================================================================

#[test]
fn test_vmovups_unaligned17_mem_to_zmm0() {
    let mut emu = emu64();
    // VMOVUPS ZMM0, [unaligned_addr2] (17-byte misalignment)
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x00, // VMOVUPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR2, &[0x55u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_unaligned17_mem_to_zmm15() {
    let mut emu = emu64();
    // VMOVUPS ZMM15, [unaligned_addr2]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM15, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR2, &[0x66u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_unaligned17_mem_to_zmm31() {
    let mut emu = emu64();
    // VMOVUPS ZMM31, [unaligned_addr2]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x10, 0x38, // VMOVUPS ZMM31, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR2, &[0x77u8; 64]);
    emu.run(None).unwrap();
}

// ============================================================================
// Register to Memory Tests - Aligned
// ============================================================================

#[test]
fn test_vmovups_zmm0_to_aligned_mem() {
    let mut emu = emu64();
    // VMOVUPS [aligned_addr], ZMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x00, // VMOVUPS [RAX], ZMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();

    let mut result = [0u8; 64];
    emu.maps.read_bytes_buff(&mut result, ALIGNED_ADDR);
}

#[test]
fn test_vmovups_zmm15_to_aligned_mem() {
    let mut emu = emu64();
    // VMOVUPS [aligned_addr], ZMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x11, 0x38, // VMOVUPS [RAX], ZMM15
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm31_to_aligned_mem() {
    let mut emu = emu64();
    // VMOVUPS [aligned_addr], ZMM31
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x11, 0x38, // VMOVUPS [RAX], ZMM31
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// ============================================================================
// Register to Memory Tests - Unaligned
// ============================================================================

#[test]
fn test_vmovups_zmm0_to_unaligned8_mem() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], ZMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x00, // VMOVUPS [RAX], ZMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm15_to_unaligned8_mem() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], ZMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x71, 0x7c, 0x48, 0x11, 0x38, // VMOVUPS [RAX], ZMM15
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm31_to_unaligned8_mem() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr], ZMM31
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x11, 0x38, // VMOVUPS [RAX], ZMM31
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm0_to_unaligned17_mem() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr2], ZMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x00, // VMOVUPS [RAX], ZMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_zmm31_to_unaligned17_mem() {
    let mut emu = emu64();
    // VMOVUPS [unaligned_addr2], ZMM31
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0x61, 0x7c, 0x48, 0x11, 0x38, // VMOVUPS [RAX], ZMM31
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// ============================================================================
// Data Pattern Tests
// ============================================================================

#[test]
fn test_vmovups_all_zeros_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x00, // VMOVUPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0x00u8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_all_ones_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x00, // VMOVUPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(UNALIGNED_ADDR, &[0xFFu8; 64]);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_alternating_pattern_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x00, // VMOVUPS ZMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let mut data = [0u8; 64];
    for i in 0..64 {
        data[i] = if i % 2 == 0 { 0xAA } else { 0x55 };
    }
    emu.maps.write_bytes_slice(UNALIGNED_ADDR2, &data);
    emu.run(None).unwrap();
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_vmovups_roundtrip_unaligned() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&UNALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x62, 0xf1, 0x7c, 0x48, 0x11, 0x00, // VMOVUPS [RAX], ZMM0
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0x08, // VMOVUPS ZMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_vmovups_chain_move_all_registers() {
    let mut emu = emu64();
    let code = [
        0x62, 0xf1, 0x7c, 0x48, 0x10, 0xc8, // VMOVUPS ZMM1, ZMM0
        0x62, 0x51, 0x7c, 0x48, 0x10, 0xc1, // VMOVUPS ZMM8, ZMM1
        0x62, 0xd1, 0x7c, 0x48, 0x10, 0xc0, // VMOVUPS ZMM16, ZMM8
        0x62, 0x91, 0x7c, 0x48, 0x10, 0xc0, // VMOVUPS ZMM24, ZMM16
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
