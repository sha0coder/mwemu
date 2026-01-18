use crate::*;

// UNPCKHPD - Unpack and Interleave High Packed Double Precision Floating-Point Values
//
// Performs an interleaved unpack of the high double precision floating-point values
// from the first source operand and the second source operand.
//
// Operation:
// DEST[63:0]   := SRC1[127:64]
// DEST[127:64] := SRC2[127:64]
//
// Opcode: 66 0F 15 /r    UNPCKHPD xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_unpckhpd_xmm0_xmm1() {
    let mut emu = emu64();
    // UNPCKHPD XMM0, XMM1
    let code = [
        0x66, 0x0f, 0x15, 0xc1, // UNPCKHPD XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm1_xmm2() {
    let mut emu = emu64();
    // UNPCKHPD XMM1, XMM2
    let code = [
        0x66, 0x0f, 0x15, 0xca, // UNPCKHPD XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm2_xmm3() {
    let mut emu = emu64();
    // UNPCKHPD XMM2, XMM3
    let code = [
        0x66, 0x0f, 0x15, 0xd3, // UNPCKHPD XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm3_xmm4() {
    let mut emu = emu64();
    // UNPCKHPD XMM3, XMM4
    let code = [
        0x66, 0x0f, 0x15, 0xdc, // UNPCKHPD XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm4_xmm5() {
    let mut emu = emu64();
    // UNPCKHPD XMM4, XMM5
    let code = [
        0x66, 0x0f, 0x15, 0xe5, // UNPCKHPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm5_xmm6() {
    let mut emu = emu64();
    // UNPCKHPD XMM5, XMM6
    let code = [
        0x66, 0x0f, 0x15, 0xee, // UNPCKHPD XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm6_xmm7() {
    let mut emu = emu64();
    // UNPCKHPD XMM6, XMM7
    let code = [
        0x66, 0x0f, 0x15, 0xf7, // UNPCKHPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm7_xmm0() {
    let mut emu = emu64();
    // UNPCKHPD XMM7, XMM0
    let code = [
        0x66, 0x0f, 0x15, 0xf8, // UNPCKHPD XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_unpckhpd_xmm8_xmm9() {
    let mut emu = emu64();
    // UNPCKHPD XMM8, XMM9
    let code = [
        0x66, 0x45, 0x0f, 0x15, 0xc1, // UNPCKHPD XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm10_xmm11() {
    let mut emu = emu64();
    // UNPCKHPD XMM10, XMM11
    let code = [
        0x66, 0x45, 0x0f, 0x15, 0xd3, // UNPCKHPD XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm12_xmm13() {
    let mut emu = emu64();
    // UNPCKHPD XMM12, XMM13
    let code = [
        0x66, 0x45, 0x0f, 0x15, 0xe5, // UNPCKHPD XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm14_xmm15() {
    let mut emu = emu64();
    // UNPCKHPD XMM14, XMM15
    let code = [
        0x66, 0x45, 0x0f, 0x15, 0xf7, // UNPCKHPD XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm0_xmm8() {
    let mut emu = emu64();
    // UNPCKHPD XMM0, XMM8
    let code = [
        0x66, 0x44, 0x0f, 0x15, 0xc0, // UNPCKHPD XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm15_xmm0() {
    let mut emu = emu64();
    // UNPCKHPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x15, 0xf8, // UNPCKHPD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm8_xmm0() {
    let mut emu = emu64();
    // UNPCKHPD XMM8, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x15, 0xc0, // UNPCKHPD XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm0_xmm15() {
    let mut emu = emu64();
    // UNPCKHPD XMM0, XMM15
    let code = [
        0x66, 0x44, 0x0f, 0x15, 0xc7, // UNPCKHPD XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_unpckhpd_xmm0_mem() {
    let mut emu = emu64();
    // UNPCKHPD XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x00, // UNPCKHPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let double_data: [f64; 2] = [1.0, 2.0];
    let mut bytes = Vec::new();
    for d in &double_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm1_mem() {
    let mut emu = emu64();
    // UNPCKHPD XMM1, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x08, // UNPCKHPD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let double_data: [f64; 2] = [3.0, 4.0];
    let mut bytes = Vec::new();
    for d in &double_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm2_mem() {
    let mut emu = emu64();
    // UNPCKHPD XMM2, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x10, // UNPCKHPD XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm7_mem() {
    let mut emu = emu64();
    // UNPCKHPD XMM7, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x38, // UNPCKHPD XMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);

    emu.run(None).unwrap();
}

// ============================================================================
// Addressing mode tests
// ============================================================================

#[test]
fn test_unpckhpd_xmm0_mem_displacement() {
    let mut emu = emu64();
    // UNPCKHPD XMM0, [RAX + disp]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x40, 0x10, // UNPCKHPD XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm1_mem_rbx() {
    let mut emu = emu64();
    // UNPCKHPD XMM1, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x0b, // UNPCKHPD XMM1, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm2_mem_rcx() {
    let mut emu = emu64();
    // UNPCKHPD XMM2, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x11, // UNPCKHPD XMM2, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_xmm3_mem_rdx() {
    let mut emu = emu64();
    // UNPCKHPD XMM3, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x1a, // UNPCKHPD XMM3, [RDX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB]);
    emu.run(None).unwrap();
}

// ============================================================================
// Data pattern tests
// ============================================================================

#[test]
fn test_unpckhpd_all_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x00, // UNPCKHPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_all_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x00, // UNPCKHPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x00, // UNPCKHPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_sequential_bytes() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x15, 0x00, // UNPCKHPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequential operations tests
// ============================================================================

#[test]
fn test_unpckhpd_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x15, 0xc1, // UNPCKHPD XMM0, XMM1
        0x66, 0x0f, 0x15, 0xd3, // UNPCKHPD XMM2, XMM3
        0x66, 0x0f, 0x15, 0xe5, // UNPCKHPD XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_same_register() {
    let mut emu = emu64();
    // UNPCKHPD XMM0, XMM0 (unpack with itself)
    let code = [
        0x66, 0x0f, 0x15, 0xc0, // UNPCKHPD XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_chain_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x15, 0xc1, // UNPCKHPD XMM0, XMM1
        0x66, 0x0f, 0x15, 0xc2, // UNPCKHPD XMM0, XMM2
        0x66, 0x0f, 0x15, 0xc3, // UNPCKHPD XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_all_register_pairs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x15, 0xc1, // UNPCKHPD XMM0, XMM1
        0x66, 0x0f, 0x15, 0xd3, // UNPCKHPD XMM2, XMM3
        0x66, 0x0f, 0x15, 0xe5, // UNPCKHPD XMM4, XMM5
        0x66, 0x0f, 0x15, 0xf7, // UNPCKHPD XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhpd_reverse_pairs() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x15, 0xc8, // UNPCKHPD XMM1, XMM0
        0x66, 0x0f, 0x15, 0xda, // UNPCKHPD XMM3, XMM2
        0x66, 0x0f, 0x15, 0xec, // UNPCKHPD XMM5, XMM4
        0x66, 0x0f, 0x15, 0xfe, // UNPCKHPD XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
