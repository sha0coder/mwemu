use crate::*;

// MOVAPD - Move Aligned Packed Double Precision Floating-Point Values
//
// Moves 128 bits (2 double-precision floating-point values) from source to destination.
// When the operand is a memory location, it must be aligned on a 16-byte boundary.
// Otherwise, a general-protection exception (#GP) is generated.
//
// Opcodes:
// 66 0F 28 /r    MOVAPD xmm1, xmm2/m128    - Move aligned packed double from xmm2/mem to xmm1
// 66 0F 29 /r    MOVAPD xmm2/m128, xmm1    - Move aligned packed double from xmm1 to xmm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movapd_xmm0_to_xmm1() {
    let mut emu = emu64();
    // MOVAPD XMM1, XMM0
    let code = [
        0x66, 0x0f, 0x28, 0xc8, // MOVAPD XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm2_to_xmm3() {
    let mut emu = emu64();
    // MOVAPD XMM3, XMM2
    let code = [
        0x66, 0x0f, 0x28, 0xda, // MOVAPD XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm4_to_xmm5() {
    let mut emu = emu64();
    // MOVAPD XMM5, XMM4
    let code = [
        0x66, 0x0f, 0x28, 0xec, // MOVAPD XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm6_to_xmm7() {
    let mut emu = emu64();
    // MOVAPD XMM7, XMM6
    let code = [
        0x66, 0x0f, 0x28, 0xfe, // MOVAPD XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm8_to_xmm9() {
    let mut emu = emu64();
    // MOVAPD XMM9, XMM8 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x28, 0xc8, // MOVAPD XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm10_to_xmm11() {
    let mut emu = emu64();
    // MOVAPD XMM11, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x28, 0xda, // MOVAPD XMM11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm12_to_xmm13() {
    let mut emu = emu64();
    // MOVAPD XMM13, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x28, 0xec, // MOVAPD XMM13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm14_to_xmm15() {
    let mut emu = emu64();
    // MOVAPD XMM15, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x28, 0xfe, // MOVAPD XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm0_to_xmm15() {
    let mut emu = emu64();
    // MOVAPD XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x28, 0xf8, // MOVAPD XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm15_to_xmm0() {
    let mut emu = emu64();
    // MOVAPD XMM0, XMM15
    let code = [
        0x66, 0x44, 0x0f, 0x28, 0xc7, // MOVAPD XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_movapd_mem_to_xmm0_aligned() {
    let mut emu = emu64();
    // MOVAPD XMM0, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]);

    emu.run(None).unwrap();
}

#[test]
fn test_movapd_mem_to_xmm1_aligned() {
    let mut emu = emu64();
    // MOVAPD XMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x08, // MOVAPD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_mem_to_xmm7_aligned() {
    let mut emu = emu64();
    // MOVAPD XMM7, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x38, // MOVAPD XMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_mem_to_xmm8_aligned() {
    let mut emu = emu64();
    // MOVAPD XMM8, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x28, 0x00, // MOVAPD XMM8, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_mem_to_xmm15_aligned() {
    let mut emu = emu64();
    // MOVAPD XMM15, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x28, 0x38, // MOVAPD XMM15, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33]);
    emu.run(None).unwrap();
}

// ============================================================================
// Register to Memory Tests (Aligned)
// ============================================================================

#[test]
fn test_movapd_xmm0_to_mem_aligned() {
    let mut emu = emu64();
    // MOVAPD [aligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x29, 0x00, // MOVAPD [RAX], XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();

    let mut result = [0u8; 16];
    emu.maps.read_bytes_buff(&mut result, ALIGNED_ADDR);
}

#[test]
fn test_movapd_xmm1_to_mem_aligned() {
    let mut emu = emu64();
    // MOVAPD [aligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x29, 0x08, // MOVAPD [RAX], XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm7_to_mem_aligned() {
    let mut emu = emu64();
    // MOVAPD [aligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x29, 0x38, // MOVAPD [RAX], XMM7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm8_to_mem_aligned() {
    let mut emu = emu64();
    // MOVAPD [aligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x29, 0x00, // MOVAPD [RAX], XMM8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_xmm15_to_mem_aligned() {
    let mut emu = emu64();
    // MOVAPD [aligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x29, 0x38, // MOVAPD [RAX], XMM15
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// ============================================================================
// Data Pattern Tests
// ============================================================================

#[test]
fn test_movapd_all_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_all_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_double_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let double1: f64 = 1.0;
    let double2: f64 = 2.0;

    let mut data = Vec::new();
    data.extend_from_slice(&double1.to_le_bytes());
    data.extend_from_slice(&double2.to_le_bytes());

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_negative_doubles() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let double1: f64 = -1.5;
    let double2: f64 = -2.5;

    let mut data = Vec::new();
    data.extend_from_slice(&double1.to_le_bytes());
    data.extend_from_slice(&double2.to_le_bytes());

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_large_doubles() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let double1: f64 = 1.234567890123456e100;
    let double2: f64 = 9.876543210987654e-100;

    let mut data = Vec::new();
    data.extend_from_slice(&double1.to_le_bytes());
    data.extend_from_slice(&double2.to_le_bytes());

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_sequential_bytes() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    emu.run(None).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_movapd_base_displacement() {
    let mut emu = emu64();
    // MOVAPD XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x40, 0x10, // MOVAPD XMM0, [RAX + 0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_rip_relative() {
    let mut emu = emu64();
    // MOVAPD XMM0, [RIP + displacement]
    // displacement = 0x3000 - 0x1008 = 0x1FF8
    let code = [
        0x66, 0x0f, 0x28, 0x05, 0xF8, 0x1F, 0x00, 0x00, // MOVAPD XMM0, [RIP+0x1FF8] -> addr 0x3000
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_bytes_slice(0x3000, &[0xBB; 16]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_with_rbx_base() {
    let mut emu = emu64();
    // MOVAPD XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x03, // MOVAPD XMM0, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_with_rcx_base() {
    let mut emu = emu64();
    // MOVAPD XMM0, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x01, // MOVAPD XMM0, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_with_rdx_base() {
    let mut emu = emu64();
    // MOVAPD XMM0, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x02, // MOVAPD XMM0, [RDX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99]);
    emu.run(None).unwrap();
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[test]
fn test_movapd_roundtrip_reg_mem_reg() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x29, 0x00, // MOVAPD [RAX], XMM0
        0x66, 0x0f, 0x28, 0x08, // MOVAPD XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_chain_move() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x28, 0xc8, // MOVAPD XMM1, XMM0
        0x66, 0x0f, 0x28, 0xd1, // MOVAPD XMM2, XMM1
        0x66, 0x0f, 0x28, 0xda, // MOVAPD XMM3, XMM2
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Boundary Tests
// ============================================================================

#[test]
fn test_movapd_at_0x5000_aligned() {
    let mut emu = emu64();
    const TEST_ADDR: u64 = 0x5000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(TEST_ADDR, &[0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_at_0x4000_aligned() {
    let mut emu = emu64();
    const TEST_ADDR: u64 = 0x4000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(TEST_ADDR, &[0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_multiple_xmm_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x28, 0xc8, // MOVAPD XMM1, XMM0
        0x66, 0x0f, 0x28, 0xd0, // MOVAPD XMM2, XMM0
        0x66, 0x0f, 0x28, 0xd8, // MOVAPD XMM3, XMM0
        0x66, 0x0f, 0x28, 0xe0, // MOVAPD XMM4, XMM0
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movapd_overwrite_destination() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87, 0x78, 0x69, 0x5A, 0x4B, 0x3C, 0x2D, 0x1E, 0x0F]);

    emu.run(None).unwrap();
}

#[test]
fn test_movapd_with_special_double_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x28, 0x00, // MOVAPD XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let double1: f64 = f64::INFINITY;
    let double2: f64 = f64::NAN;

    let mut data = Vec::new();
    data.extend_from_slice(&double1.to_le_bytes());
    data.extend_from_slice(&double2.to_le_bytes());

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
