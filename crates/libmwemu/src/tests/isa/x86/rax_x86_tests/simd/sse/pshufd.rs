use crate::*;

// PSHUFD - Shuffle Packed Doublewords
//
// Copies doublewords from source operand and inserts them in the destination operand
// at the locations selected with the order operand (immediate byte).
//
// Each 2-bit field in the immediate operand selects the contents of one doubleword location:
// - Bits [1:0] select source doubleword for DEST[31:0]
// - Bits [3:2] select source doubleword for DEST[63:32]
// - Bits [5:4] select source doubleword for DEST[95:64]
// - Bits [7:6] select source doubleword for DEST[127:96]
//
// Opcode: 66 0F 70 /r ib    PSHUFD xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with immediate value 0x00 (broadcast element 0)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x00() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x00
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x00, // PSHUFD XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm2_xmm3_imm_0x00() {
    let mut emu = emu64();
    // PSHUFD XMM2, XMM3, 0x00
    let code = [
        0x66, 0x0f, 0x70, 0xd3, 0x00, // PSHUFD XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x55 (broadcast element 1)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x55() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x55
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x55, // PSHUFD XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm4_xmm5_imm_0x55() {
    let mut emu = emu64();
    // PSHUFD XMM4, XMM5, 0x55
    let code = [
        0x66, 0x0f, 0x70, 0xe5, 0x55, // PSHUFD XMM4, XMM5, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xAA (broadcast element 2)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xaa() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xAA
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xaa, // PSHUFD XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm6_xmm7_imm_0xaa() {
    let mut emu = emu64();
    // PSHUFD XMM6, XMM7, 0xAA
    let code = [
        0x66, 0x0f, 0x70, 0xf7, 0xaa, // PSHUFD XMM6, XMM7, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xFF (broadcast element 3)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xff() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xFF
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xff, // PSHUFD XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm1_xmm2_imm_0xff() {
    let mut emu = emu64();
    // PSHUFD XMM1, XMM2, 0xFF
    let code = [
        0x66, 0x0f, 0x70, 0xca, 0xff, // PSHUFD XMM1, XMM2, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xE4 (identity/no shuffle)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xE4
    // 0xE4 = 11 10 01 00 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFD XMM0, XMM1, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x1B (reverse order)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x1B
    // 0x1B = 00 01 10 11 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x1b, // PSHUFD XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm5_xmm6_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFD XMM5, XMM6, 0x1B
    let code = [
        0x66, 0x0f, 0x70, 0xee, 0x1b, // PSHUFD XMM5, XMM6, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x4E (swap pairs)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x4e() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x4E
    // 0x4E = 01 00 11 10 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x4e, // PSHUFD XMM0, XMM1, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm7_xmm0_imm_0x4e() {
    let mut emu = emu64();
    // PSHUFD XMM7, XMM0, 0x4E
    let code = [
        0x66, 0x0f, 0x70, 0xf8, 0x4e, // PSHUFD XMM7, XMM0, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xB1 (swap within pairs)
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xb1() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xB1
    // 0xB1 = 10 11 00 01 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xb1, // PSHUFD XMM0, XMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm2_xmm3_imm_0xb1() {
    let mut emu = emu64();
    // PSHUFD XMM2, XMM3, 0xB1
    let code = [
        0x66, 0x0f, 0x70, 0xd3, 0xb1, // PSHUFD XMM2, XMM3, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with various immediate values
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x27() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x27
    // 0x27 = 00 10 01 11 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x27, // PSHUFD XMM0, XMM1, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x39() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x39
    // 0x39 = 00 11 10 01 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x39, // PSHUFD XMM0, XMM1, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x72() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x72
    // 0x72 = 01 11 00 10 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x72, // PSHUFD XMM0, XMM1, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x93() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x93
    // 0x93 = 10 01 00 11 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x93, // PSHUFD XMM0, XMM1, 0x93
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xc6() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xC6
    // 0xC6 = 11 00 01 10 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xc6, // PSHUFD XMM0, XMM1, 0xC6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xd8() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xD8
    // 0xD8 = 11 01 10 00 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xd8, // PSHUFD XMM0, XMM1, 0xD8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x44() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x44
    // 0x44 = 01 00 01 00 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x44, // PSHUFD XMM0, XMM1, 0x44
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xee() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xEE
    // 0xEE = 11 10 11 10 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xee, // PSHUFD XMM0, XMM1, 0xEE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x50() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x50
    // 0x50 = 01 01 00 00 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x50, // PSHUFD XMM0, XMM1, 0x50
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xfa() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xFA
    // 0xFA = 11 11 10 10 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xfa, // PSHUFD XMM0, XMM1, 0xFA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with different register pairs
// ============================================================================

#[test]
fn test_pshufd_xmm1_xmm2_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM1, XMM2, 0xE4
    let code = [
        0x66, 0x0f, 0x70, 0xca, 0xe4, // PSHUFD XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm5_xmm6_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM5, XMM6, 0xE4
    let code = [
        0x66, 0x0f, 0x70, 0xee, 0xe4, // PSHUFD XMM5, XMM6, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm7_xmm0_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM7, XMM0, 0xE4
    let code = [
        0x66, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFD XMM7, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_pshufd_xmm8_xmm9_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM8, XMM9, 0xE4
    let code = [
        0x66, 0x45, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFD XMM8, XMM9, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm10_xmm11_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM10, XMM11, 0xE4
    let code = [
        0x66, 0x45, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFD XMM10, XMM11, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm12_xmm13_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM12, XMM13, 0xE4
    let code = [
        0x66, 0x45, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFD XMM12, XMM13, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm14_xmm15_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM14, XMM15, 0xE4
    let code = [
        0x66, 0x45, 0x0f, 0x70, 0xf7, 0xe4, // PSHUFD XMM14, XMM15, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm8_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM8, 0xE4
    let code = [
        0x66, 0x44, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFD XMM0, XMM8, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm15_xmm0_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM15, XMM0, 0xE4
    let code = [
        0x66, 0x44, 0x0f, 0x70, 0xf8, 0xe4, // PSHUFD XMM15, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_pshufd_xmm0_mem_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM0, [mem], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x00, 0xe4, // PSHUFD XMM0, [RAX], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let dword_data: [u32; 4] = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
    let mut bytes = Vec::new();
    for d in &dword_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm1_mem_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFD XMM1, [mem], 0x1B
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x08, 0x1b, // PSHUFD XMM1, [RAX], 0x1B
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    let dword_data: [u32; 4] = [0xAABBCCDD, 0xEEFF0011, 0x22334455, 0x66778899];
    let mut bytes = Vec::new();
    for d in &dword_data {
        bytes.extend_from_slice(&d.to_le_bytes());
    }
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &bytes);

    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm2_mem_imm_0x00() {
    let mut emu = emu64();
    // PSHUFD XMM2, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x10, 0x00, // PSHUFD XMM2, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm3_mem_imm_0xff() {
    let mut emu = emu64();
    // PSHUFD XMM3, [mem], 0xFF
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x18, 0xff, // PSHUFD XMM3, [RAX], 0xFF
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm7_mem_imm_0x4e() {
    let mut emu = emu64();
    // PSHUFD XMM7, [mem], 0x4E
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x38, 0x4e, // PSHUFD XMM7, [RAX], 0x4E
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
fn test_pshufd_xmm0_mem_displacement_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM0, [RAX + disp], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x40, 0x10, 0xe4, // PSHUFD XMM0, [RAX+0x10], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm1_mem_rbx_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM1, [RBX], 0xE4
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x66, 0x0f, 0x70, 0x0b, 0xe4, // PSHUFD XMM1, [RBX], 0xE4
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
fn test_pshufd_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xe4, // PSHUFD XMM0, XMM1, 0xE4
        0x66, 0x0f, 0x70, 0xd3, 0xe4, // PSHUFD XMM2, XMM3, 0xE4
        0x66, 0x0f, 0x70, 0xe5, 0xe4, // PSHUFD XMM4, XMM5, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_same_register_imm_0xe4() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM0, 0xE4 (shuffle with itself)
    let code = [
        0x66, 0x0f, 0x70, 0xc0, 0xe4, // PSHUFD XMM0, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_same_register_imm_0x1b() {
    let mut emu = emu64();
    // PSHUFD XMM1, XMM1, 0x1B (shuffle with itself, reverse)
    let code = [
        0x66, 0x0f, 0x70, 0xc9, 0x1b, // PSHUFD XMM1, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Additional permutation tests
// ============================================================================

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x0f() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x0F
    // 0x0F = 00 00 11 11 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x0f, // PSHUFD XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xf0() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xF0
    // 0xF0 = 11 11 00 00 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xf0, // PSHUFD XMM0, XMM1, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0xa5() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0xA5
    // 0xA5 = 10 10 01 01 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0xa5, // PSHUFD XMM0, XMM1, 0xA5
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x5a() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x5A
    // 0x5A = 01 01 10 10 (binary)
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x5a, // PSHUFD XMM0, XMM1, 0x5A
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_pshufd_xmm0_xmm1_imm_0x2d() {
    let mut emu = emu64();
    // PSHUFD XMM0, XMM1, 0x2D
    let code = [
        0x66, 0x0f, 0x70, 0xc1, 0x2d, // PSHUFD XMM0, XMM1, 0x2D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
