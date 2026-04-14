use crate::*;

// MOVDQA - Move Aligned Double Quadword (128-bit Integer)
//
// Moves 128 bits of packed integer data from source to destination.
// When the operand is a memory location, it must be aligned on a 16-byte boundary.
// Otherwise, a general-protection exception (#GP) is generated.
//
// Opcodes:
// 66 0F 6F /r    MOVDQA xmm1, xmm2/m128    - Move aligned packed integer from xmm2/mem to xmm1
// 66 0F 7F /r    MOVDQA xmm2/m128, xmm1    - Move aligned packed integer from xmm1 to xmm2/mem

const ALIGNED_ADDR: u64 = 0x3000; // 16-byte aligned address for testing

// ============================================================================
// Register to Register Tests
// ============================================================================

#[test]
fn test_movdqa_xmm0_to_xmm1() {
    let mut emu = emu64();
    // MOVDQA XMM1, XMM0
    let code = [
        0x66, 0x0f, 0x6f, 0xc8, // MOVDQA XMM1, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm2_to_xmm3() {
    let mut emu = emu64();
    // MOVDQA XMM3, XMM2
    let code = [
        0x66, 0x0f, 0x6f, 0xda, // MOVDQA XMM3, XMM2
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm4_to_xmm5() {
    let mut emu = emu64();
    // MOVDQA XMM5, XMM4
    let code = [
        0x66, 0x0f, 0x6f, 0xec, // MOVDQA XMM5, XMM4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm6_to_xmm7() {
    let mut emu = emu64();
    // MOVDQA XMM7, XMM6
    let code = [
        0x66, 0x0f, 0x6f, 0xfe, // MOVDQA XMM7, XMM6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm8_to_xmm9() {
    let mut emu = emu64();
    // MOVDQA XMM9, XMM8 (requires REX prefix)
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xc8, // MOVDQA XMM9, XMM8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm10_to_xmm11() {
    let mut emu = emu64();
    // MOVDQA XMM11, XMM10
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xda, // MOVDQA XMM11, XMM10
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm12_to_xmm13() {
    let mut emu = emu64();
    // MOVDQA XMM13, XMM12
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xec, // MOVDQA XMM13, XMM12
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm14_to_xmm15() {
    let mut emu = emu64();
    // MOVDQA XMM15, XMM14
    let code = [
        0x66, 0x45, 0x0f, 0x6f, 0xfe, // MOVDQA XMM15, XMM14
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm0_to_xmm15() {
    let mut emu = emu64();
    // MOVDQA XMM15, XMM0
    let code = [
        0x66, 0x44, 0x0f, 0x6f, 0xf8, // MOVDQA XMM15, XMM0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm15_to_xmm0() {
    let mut emu = emu64();
    // MOVDQA XMM0, XMM15
    let code = [
        0x66, 0x44, 0x0f, 0x6f, 0xc7, // MOVDQA XMM0, XMM15
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory to Register Tests (Aligned)
// ============================================================================

#[test]
fn test_movdqa_mem_to_xmm0_aligned() {
    let mut emu = emu64();
    // MOVDQA XMM0, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]);

    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_mem_to_xmm1_aligned() {
    let mut emu = emu64();
    // MOVDQA XMM1, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_mem_to_xmm7_aligned() {
    let mut emu = emu64();
    // MOVDQA XMM7, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x38, // MOVDQA XMM7, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_mem_to_xmm8_aligned() {
    let mut emu = emu64();
    // MOVDQA XMM8, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x00, // MOVDQA XMM8, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_mem_to_xmm15_aligned() {
    let mut emu = emu64();
    // MOVDQA XMM15, [aligned_addr]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x6f, 0x38, // MOVDQA XMM15, [RAX]
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
fn test_movdqa_xmm0_to_mem_aligned() {
    let mut emu = emu64();
    // MOVDQA [aligned_addr], XMM0
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x00, // MOVDQA [RAX], XMM0
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();

    let mut result = [0u8; 16];
    emu.maps.read_bytes_buff(&mut result, ALIGNED_ADDR);
}

#[test]
fn test_movdqa_xmm1_to_mem_aligned() {
    let mut emu = emu64();
    // MOVDQA [aligned_addr], XMM1
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x08, // MOVDQA [RAX], XMM1
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm7_to_mem_aligned() {
    let mut emu = emu64();
    // MOVDQA [aligned_addr], XMM7
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x38, // MOVDQA [RAX], XMM7
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm8_to_mem_aligned() {
    let mut emu = emu64();
    // MOVDQA [aligned_addr], XMM8
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x7f, 0x00, // MOVDQA [RAX], XMM8
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_xmm15_to_mem_aligned() {
    let mut emu = emu64();
    // MOVDQA [aligned_addr], XMM15
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x44, 0x0f, 0x7f, 0x38, // MOVDQA [RAX], XMM15
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

// ============================================================================
// Integer Data Pattern Tests
// ============================================================================

#[test]
fn test_movdqa_all_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_all_ones() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_packed_bytes() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_packed_words() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let words: [u16; 8] = [0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777, 0x8888];
    let mut data = Vec::new();
    for word in &words {
        data.extend_from_slice(&word.to_le_bytes());
    }

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_packed_dwords() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let dwords: [u32; 4] = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
    let mut data = Vec::new();
    for dword in &dwords {
        data.extend_from_slice(&dword.to_le_bytes());
    }

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_packed_qwords() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let qwords: [u64; 2] = [0x1111111111111111, 0x2222222222222222];
    let mut data = Vec::new();
    for qword in &qwords {
        data.extend_from_slice(&qword.to_le_bytes());
    }

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_alternating_pattern() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x55]);
    emu.run(None).unwrap();
}

// ============================================================================
// Addressing Mode Tests
// ============================================================================

#[test]
fn test_movdqa_base_displacement() {
    let mut emu = emu64();
    // MOVDQA XMM0, [RAX + displacement]
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x40, 0x10, // MOVDQA XMM0, [RAX + 0x10]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12, 0x12]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_rip_relative() {
    let mut emu = emu64();
    // MOVDQA XMM0, [RIP + displacement]
    let code = [
        0x66, 0x0f, 0x6f, 0x05, 0x08, 0x00, 0x00, 0x00, // MOVDQA XMM0, [RIP+8]
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.maps.write_bytes_slice(0x1010, &[0xAA; 16]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_with_rbx_base() {
    let mut emu = emu64();
    // MOVDQA XMM0, [RBX]
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x03, // MOVDQA XMM0, [RBX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_with_rcx_base() {
    let mut emu = emu64();
    // MOVDQA XMM0, [RCX]
    let code = [
        0x48, 0xb9, // MOV RCX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x01, // MOVDQA XMM0, [RCX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_with_rdx_base() {
    let mut emu = emu64();
    // MOVDQA XMM0, [RDX]
    let code = [
        0x48, 0xba, // MOV RDX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x02, // MOVDQA XMM0, [RDX]
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
fn test_movdqa_roundtrip_reg_mem_reg() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x7f, 0x00, // MOVDQA [RAX], XMM0
        0x66, 0x0f, 0x6f, 0x08, // MOVDQA XMM1, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_chain_move() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6f, 0xc8, // MOVDQA XMM1, XMM0
        0x66, 0x0f, 0x6f, 0xd1, // MOVDQA XMM2, XMM1
        0x66, 0x0f, 0x6f, 0xda, // MOVDQA XMM3, XMM2
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Boundary Tests
// ============================================================================

#[test]
fn test_movdqa_at_0x2000_aligned() {
    let mut emu = emu64();
    const TEST_ADDR: u64 = 0x2000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(TEST_ADDR, &[0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB, 0xBB]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_at_0x4000_aligned() {
    let mut emu = emu64();
    const TEST_ADDR: u64 = 0x4000;
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&TEST_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(TEST_ADDR, &[0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC]);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_multiple_xmm_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x6f, 0xc8, // MOVDQA XMM1, XMM0
        0x66, 0x0f, 0x6f, 0xd0, // MOVDQA XMM2, XMM0
        0x66, 0x0f, 0x6f, 0xd8, // MOVDQA XMM3, XMM0
        0x66, 0x0f, 0x6f, 0xe0, // MOVDQA XMM4, XMM0
        0xf4, // HLT
    ];

    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_overwrite_destination() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xF0, 0xE1, 0xD2, 0xC3, 0xB4, 0xA5, 0x96, 0x87, 0x78, 0x69, 0x5A, 0x4B, 0x3C, 0x2D, 0x1E, 0x0F]);

    emu.run(None).unwrap();
}

#[test]
fn test_movdqa_max_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x6f, 0x00, // MOVDQA XMM0, [RAX]
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let qwords: [u64; 2] = [i64::MAX as u64, u64::MAX];
    let mut data = Vec::new();
    for qword in &qwords {
        data.extend_from_slice(&qword.to_le_bytes());
    }

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}
