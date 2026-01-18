use crate::*;

// PSHUFHW - Shuffle Packed High Words
//
// Copies words from the high quadword of the source operand and inserts them
// in the high quadword of the destination operand at word locations selected
// with the immediate operand. The low quadword is copied unchanged.
//
// Each 2-bit field in the immediate operand selects the contents of one word:
// - Bits [1:0] select source word for DEST[79:64] (from high quadword)
// - Bits [3:2] select source word for DEST[95:80] (from high quadword)
// - Bits [5:4] select source word for DEST[111:96] (from high quadword)
// - Bits [7:6] select source word for DEST[127:112] (from high quadword)
// - DEST[63:0] = SRC[63:0] (low quadword unchanged)
//
// Opcode: F3 0F 70 /r ib    PSHUFHW xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with immediate value 0x00 (broadcast word 4)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x00() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x00
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x00, // PSHUFHW XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm2_xmm3_imm_0x00() {
    let mut emu = emu64();
    // PSHUFHW XMM2, XMM3, 0x00
    let code = [
        0xf3, 0x0f, 0x70, 0xd3, 0x00, // PSHUFHW XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x55 (broadcast word 5)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x55() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x55
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x55, // PSHUFHW XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm4_xmm5_imm_0x55() {
    let mut emu = emu64();
    // PSHUFHW XMM4, XMM5, 0x55
    let code = [
        0xf3, 0x0f, 0x70, 0xe5, 0x55, // PSHUFHW XMM4, XMM5, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xAA (broadcast word 6)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xaa() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xAA
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xaa, // PSHUFHW XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm6_xmm7_imm_0xaa() {
    let mut emu = emu64();
    // PSHUFHW XMM6, XMM7, 0xAA
    let code = [
        0xf3, 0x0f, 0x70, 0xf7, 0xaa, // PSHUFHW XMM6, XMM7, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xFF (broadcast word 7)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xff() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xFF
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xff, // PSHUFHW XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm1_xmm2_imm_0xff() {
    let mut emu = emu64();
    // PSHUFHW XMM1, XMM2, 0xFF
    let code = [
        0xf3, 0x0f, 0x70, 0xca, 0xff, // PSHUFHW XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xE4 (identity for high words)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xE4
    // 0xE4 = 11 10 01 00 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFHW XMM0, XMM1, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x1B (reverse high words)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x1B
    // 0x1B = 00 01 10 11 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x1b, // PSHUFHW XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm5_xmm6_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFHW XMM5, XMM6, 0x1B
    let code = [
        0xf3, 0x0f, 0x70, 0xee, 0x1b, // PSHUFHW XMM5, XMM6, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xB1 (swap pairs in high words)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xb1() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xB1
    // 0xB1 = 10 11 00 01 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xb1, // PSHUFHW XMM0, XMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm2_xmm3_imm_0xb1() {
    let mut emu = emu64();
    // PSHUFHW XMM2, XMM3, 0xB1
    let code = [
        0xf3, 0x0f, 0x70, 0xd3, 0xb1, // PSHUFHW XMM2, XMM3, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x4E (swap word pairs in high quadword)
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x4e() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x4E
    // 0x4E = 01 00 11 10 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x4e, // PSHUFHW XMM0, XMM1, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm7_xmm0_imm_0x4e() {
    let mut emu = emu64();
    // PSHUFHW XMM7, XMM0, 0x4E
    let code = [
        0xf3, 0x0f, 0x70, 0xf8, 0x4e, // PSHUFHW XMM7, XMM0, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with various immediate values
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x27() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x27
    // 0x27 = 00 10 01 11 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x27, // PSHUFHW XMM0, XMM1, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x39() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x39
    // 0x39 = 00 11 10 01 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x39, // PSHUFHW XMM0, XMM1, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x72() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x72
    // 0x72 = 01 11 00 10 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x72, // PSHUFHW XMM0, XMM1, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x93() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x93
    // 0x93 = 10 01 00 11 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x93, // PSHUFHW XMM0, XMM1, 0x93
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xc6() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xC6
    // 0xC6 = 11 00 01 10 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xc6, // PSHUFHW XMM0, XMM1, 0xC6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xd8() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xD8
    // 0xD8 = 11 01 10 00 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xd8, // PSHUFHW XMM0, XMM1, 0xD8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x44() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x44
    // 0x44 = 01 00 01 00 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x44, // PSHUFHW XMM0, XMM1, 0x44
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xee() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xEE
    // 0xEE = 11 10 11 10 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xee, // PSHUFHW XMM0, XMM1, 0xEE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x50() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x50
    // 0x50 = 01 01 00 00 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x50, // PSHUFHW XMM0, XMM1, 0x50
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xfa() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xFA
    // 0xFA = 11 11 10 10 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xfa, // PSHUFHW XMM0, XMM1, 0xFA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with different register pairs
// ============================================================================

#[test]
fn test_pshufhw_xmm1_xmm2_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM1, XMM2, 0xE4
    let code = [
        0xf3, 0x0f, 0x70, 0xca, 0xe4, // PSHUFHW XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm5_xmm6_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM5, XMM6, 0xE4
    let code = [
        0xf3, 0x0f, 0x70, 0xee, 0xe4, // PSHUFHW XMM5, XMM6, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm7_xmm0_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM7, XMM0, 0xE4
    let code = [
        0xf3, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFHW XMM7, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pshufhw_xmm8_xmm9_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM8, XMM9, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFHW XMM8, XMM9, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm10_xmm11_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM10, XMM11, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFHW XMM10, XMM11, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm12_xmm13_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM12, XMM13, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFHW XMM12, XMM13, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm14_xmm15_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM14, XMM15, 0xE4
    let code = [
        0xf3, 0x45, 0x0f, 0x70, 0xf7, 0xe4, // PSHUFHW XMM14, XMM15, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm8_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM8, 0xE4
    let code = [
        0xf3, 0x44, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFHW XMM0, XMM8, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm15_xmm0_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM15, XMM0, 0xE4
    let code = [
        0xf3, 0x44, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFHW XMM15, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_pshufhw_xmm0_mem_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM0, [mem], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x00, 0xe4, // PSHUFHW XMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let word_data: [u16; 8] = [0x1111, 0x2222, 0x3333, 0x4444, 0x5555, 0x6666, 0x7777, 0x8888];
    let mut bytes = Vec::new();
    for w in &word_data {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm1_mem_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFHW XMM1, [mem], 0x1B
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x08, 0x1b, // PSHUFHW XMM1, [RAX], 0x1B
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let word_data: [u16; 8] = [0xAAAA, 0xBBBB, 0xCCCC, 0xDDDD, 0xEEEE, 0xFFFF, 0x0000, 0x1111];
    let mut bytes = Vec::new();
    for w in &word_data {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm2_mem_imm_0x00() {
    let mut emu = emu64();
    // PSHUFHW XMM2, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x10, 0x00, // PSHUFHW XMM2, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm3_mem_imm_0xff() {
    let mut emu = emu64();
    // PSHUFHW XMM3, [mem], 0xFF
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x18, 0xff, // PSHUFHW XMM3, [RAX], 0xFF
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm7_mem_imm_0x4e() {
    let mut emu = emu64();
    // PSHUFHW XMM7, [mem], 0x4E
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x38, 0x4e, // PSHUFHW XMM7, [RAX], 0x4E
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
fn test_pshufhw_xmm0_mem_displacement_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM0, [RAX + disp], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x40, 0x10, 0xe4, // PSHUFHW XMM0, [RAX+0x10], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm1_mem_rbx_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM1, [RBX], 0xE4
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0xf3, 0x0f, 0x70, 0x0b, 0xe4, // PSHUFHW XMM1, [RBX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88]);
    emu.run(None).unwrap();
}

// ============================================================================
// Sequential shuffle tests
// ============================================================================

#[test]
fn test_pshufhw_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFHW XMM0, XMM1, 0xE4
        0xf3, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFHW XMM2, XMM3, 0xE4
        0xf3, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFHW XMM4, XMM5, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_same_register_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM0, 0xE4 (shuffle with itself)
    let code = [
        0xf3, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFHW XMM0, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_same_register_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFHW XMM1, XMM1, 0x1B (shuffle with itself, reverse)
    let code = [
        0xf3, 0x0f, 0x70, 0xc9, 0x1b, // PSHUFHW XMM1, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional permutation tests
// ============================================================================

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x0f() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x0F
    // 0x0F = 00 00 11 11 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x0f, // PSHUFHW XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xf0() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xF0
    // 0xF0 = 11 11 00 00 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xf0, // PSHUFHW XMM0, XMM1, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0xa5() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0xA5
    // 0xA5 = 10 10 01 01 (binary)
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0xa5, // PSHUFHW XMM0, XMM1, 0xA5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufhw_xmm0_xmm1_imm_0x2d() {
    let mut emu = emu64();
    // PSHUFHW XMM0, XMM1, 0x2D
    let code = [
        0xf3, 0x0f, 0x70, 0xc1, 0x2d, // PSHUFHW XMM0, XMM1, 0x2D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
