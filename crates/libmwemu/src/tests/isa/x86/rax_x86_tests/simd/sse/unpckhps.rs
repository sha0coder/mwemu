use crate::*;

// UNPCKHPS - Unpack and Interleave High Packed Single Precision Floating-Point Values
//
// Performs an interleaved unpack of the high single precision floating-point values
// from the first source operand and the second source operand.
//
// Operation:
// DEST[31:0]   := SRC1[95:64]
// DEST[63:32]  := SRC2[95:64]
// DEST[95:64]  := SRC1[127:96]
// DEST[127:96] := SRC2[127:96]
//
// Opcode: NP 0F 15 /r    UNPCKHPS xmm1, xmm2/m128

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_unpckhps_xmm0_xmm1() {
    let mut emu = emu64();
    // UNPCKHPS XMM0, XMM1
    let code = [
        0x0f, 0x15, 0xc1, // UNPCKHPS XMM0, XMM1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm1_xmm2() {
    let mut emu = emu64();
    // UNPCKHPS XMM1, XMM2
    let code = [
        0x0f, 0x15, 0xca, // UNPCKHPS XMM1, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm2_xmm3() {
    let mut emu = emu64();
    // UNPCKHPS XMM2, XMM3
    let code = [
        0x0f, 0x15, 0xd3, // UNPCKHPS XMM2, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm3_xmm4() {
    let mut emu = emu64();
    // UNPCKHPS XMM3, XMM4
    let code = [
        0x0f, 0x15, 0xdc, // UNPCKHPS XMM3, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm4_xmm5() {
    let mut emu = emu64();
    // UNPCKHPS XMM4, XMM5
    let code = [
        0x0f, 0x15, 0xe5, // UNPCKHPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm5_xmm6() {
    let mut emu = emu64();
    // UNPCKHPS XMM5, XMM6
    let code = [
        0x0f, 0x15, 0xee, // UNPCKHPS XMM5, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm6_xmm7() {
    let mut emu = emu64();
    // UNPCKHPS XMM6, XMM7
    let code = [
        0x0f, 0x15, 0xf7, // UNPCKHPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm7_xmm0() {
    let mut emu = emu64();
    // UNPCKHPS XMM7, XMM0
    let code = [
        0x0f, 0x15, 0xf8, // UNPCKHPS XMM7, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_unpckhps_xmm8_xmm9() {
    let mut emu = emu64();
    // UNPCKHPS XMM8, XMM9
    let code = [
        0x45, 0x0f, 0x15, 0xc1, // UNPCKHPS XMM8, XMM9
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm10_xmm11() {
    let mut emu = emu64();
    // UNPCKHPS XMM10, XMM11
    let code = [
        0x45, 0x0f, 0x15, 0xd3, // UNPCKHPS XMM10, XMM11
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm12_xmm13() {
    let mut emu = emu64();
    // UNPCKHPS XMM12, XMM13
    let code = [
        0x45, 0x0f, 0x15, 0xe5, // UNPCKHPS XMM12, XMM13
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm14_xmm15() {
    let mut emu = emu64();
    // UNPCKHPS XMM14, XMM15
    let code = [
        0x45, 0x0f, 0x15, 0xf7, // UNPCKHPS XMM14, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm0_xmm8() {
    let mut emu = emu64();
    // UNPCKHPS XMM0, XMM8
    let code = [
        0x44, 0x0f, 0x15, 0xc0, // UNPCKHPS XMM0, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm15_xmm0() {
    let mut emu = emu64();
    // UNPCKHPS XMM15, XMM0
    let code = [
        0x44, 0x0f, 0x15, 0xf8, // UNPCKHPS XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm8_xmm0() {
    let mut emu = emu64();
    // UNPCKHPS XMM8, XMM0
    let code = [
        0x44, 0x0f, 0x15, 0xc0, // UNPCKHPS XMM8, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm0_xmm15() {
    let mut emu = emu64();
    // UNPCKHPS XMM0, XMM15
    let code = [
        0x44, 0x0f, 0x15, 0xc7, // UNPCKHPS XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_unpckhps_xmm0_mem() {
    let mut emu = emu64();
    // UNPCKHPS XMM0, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x00, // UNPCKHPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let float_data: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    let mut bytes = Vec::new();
    for f in &float_data {
        bytes.extend_from_slice(&f.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm1_mem() {
    let mut emu = emu64();
    // UNPCKHPS XMM1, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x08, // UNPCKHPS XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let float_data: [f32; 4] = [5.0, 6.0, 7.0, 8.0];
    let mut bytes = Vec::new();
    for f in &float_data {
        bytes.extend_from_slice(&f.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm2_mem() {
    let mut emu = emu64();
    // UNPCKHPS XMM2, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x10, // UNPCKHPS XMM2, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm3_mem() {
    let mut emu = emu64();
    // UNPCKHPS XMM3, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x18, // UNPCKHPS XMM3, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);

    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm7_mem() {
    let mut emu = emu64();
    // UNPCKHPS XMM7, [mem]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x38, // UNPCKHPS XMM7, [RAX]
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
fn test_unpckhps_xmm0_mem_displacement() {
    let mut emu = emu64();
    // UNPCKHPS XMM0, [RAX + disp]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x40, 0x10, // UNPCKHPS XMM0, [RAX+0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm1_mem_rbx() {
    let mut emu = emu64();
    // UNPCKHPS XMM1, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x0b, // UNPCKHPS XMM1, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm2_mem_rcx() {
    let mut emu = emu64();
    // UNPCKHPS XMM2, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x11, // UNPCKHPS XMM2, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_xmm3_mem_rdx() {
    let mut emu = emu64();
    // UNPCKHPS XMM3, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x1a, // UNPCKHPS XMM3, [RDX]
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
fn test_unpckhps_all_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x00, // UNPCKHPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_all_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x00, // UNPCKHPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x00, // UNPCKHPS XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_sequential_bytes() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x15, 0x00, // UNPCKHPS XMM0, [RAX]
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
fn test_unpckhps_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x15, 0xc1, // UNPCKHPS XMM0, XMM1
        0x0f, 0x15, 0xd3, // UNPCKHPS XMM2, XMM3
        0x0f, 0x15, 0xe5, // UNPCKHPS XMM4, XMM5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_same_register() {
    let mut emu = emu64();
    // UNPCKHPS XMM0, XMM0 (unpack with itself)
    let code = [
        0x0f, 0x15, 0xc0, // UNPCKHPS XMM0, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_chain_operations() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x15, 0xc1, // UNPCKHPS XMM0, XMM1
        0x0f, 0x15, 0xc2, // UNPCKHPS XMM0, XMM2
        0x0f, 0x15, 0xc3, // UNPCKHPS XMM0, XMM3
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_all_register_pairs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x15, 0xc1, // UNPCKHPS XMM0, XMM1
        0x0f, 0x15, 0xd3, // UNPCKHPS XMM2, XMM3
        0x0f, 0x15, 0xe5, // UNPCKHPS XMM4, XMM5
        0x0f, 0x15, 0xf7, // UNPCKHPS XMM6, XMM7
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_unpckhps_reverse_pairs() {
    let mut emu = emu64();
    let code = [
        0x0f, 0x15, 0xc8, // UNPCKHPS XMM1, XMM0
        0x0f, 0x15, 0xda, // UNPCKHPS XMM3, XMM2
        0x0f, 0x15, 0xec, // UNPCKHPS XMM5, XMM4
        0x0f, 0x15, 0xfe, // UNPCKHPS XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
