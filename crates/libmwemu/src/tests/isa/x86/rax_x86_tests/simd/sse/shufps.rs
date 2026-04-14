use crate::*;

// SHUFPS - Packed Interleave Shuffle of Quadruplets of Single Precision Floating-Point Values
//
// Selects single precision floating-point values from two source operands using an 8-bit immediate
// control byte and stores the results in the destination operand.
//
// The immediate byte consists of four 2-bit fields:
// - Bits [1:0] select from first quadruplet of SRC1 -> DEST[31:0]
// - Bits [3:2] select from first quadruplet of SRC1 -> DEST[63:32]
// - Bits [5:4] select from first quadruplet of SRC2 -> DEST[95:64]
// - Bits [7:6] select from first quadruplet of SRC2 -> DEST[127:96]
//
// Opcode: NP 0F C6 /r ib    SHUFPS xmm1, xmm2/m128, imm8

const ALIGNED_ADDR: u64 = 0x3000;

// ============================================================================
// Tests with immediate value 0x00 (all elements from position 0)
// ============================================================================

#[test]
fn test_shufps_xmm0_xmm1_imm_0x00() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x00
    let code = [
        0x0f, 0xc6, 0xc1, 0x00, // SHUFPS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm2_xmm3_imm_0x00() {
    let mut emu = emu64();
    // SHUFPS XMM2, XMM3, 0x00
    let code = [
        0x0f, 0xc6, 0xd3, 0x00, // SHUFPS XMM2, XMM3, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xFF (all elements from position 3)
// ============================================================================

#[test]
fn test_shufps_xmm0_xmm1_imm_0xff() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xFF
    let code = [
        0x0f, 0xc6, 0xc1, 0xff, // SHUFPS XMM0, XMM1, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm4_xmm5_imm_0xff() {
    let mut emu = emu64();
    // SHUFPS XMM4, XMM5, 0xFF
    let code = [
        0x0f, 0xc6, 0xe5, 0xff, // SHUFPS XMM4, XMM5, 0xFF
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0xE4 (identity/no shuffle)
// ============================================================================

#[test]
fn test_shufps_xmm0_xmm1_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xE4
    // 0xE4 = 11 10 01 00 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0xe4, // SHUFPS XMM0, XMM1, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm6_xmm7_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM6, XMM7, 0xE4
    let code = [
        0x0f, 0xc6, 0xf7, 0xe4, // SHUFPS XMM6, XMM7, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with immediate value 0x1B (reverse order)
// ============================================================================

#[test]
fn test_shufps_xmm0_xmm1_imm_0x1b() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x1B
    // 0x1B = 00 01 10 11 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x1b, // SHUFPS XMM0, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm2_xmm3_imm_0x1b() {
    let mut emu = emu64();
    // SHUFPS XMM2, XMM3, 0x1B
    let code = [
        0x0f, 0xc6, 0xd3, 0x1b, // SHUFPS XMM2, XMM3, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with various immediate values
// ============================================================================

#[test]
fn test_shufps_xmm0_xmm1_imm_0x44() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x44
    // 0x44 = 01 00 01 00 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x44, // SHUFPS XMM0, XMM1, 0x44
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xee() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xEE
    // 0xEE = 11 10 11 10 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0xee, // SHUFPS XMM0, XMM1, 0xEE
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x88() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x88
    // 0x88 = 10 00 10 00 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x88, // SHUFPS XMM0, XMM1, 0x88
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xdd() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xDD
    // 0xDD = 11 01 11 01 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0xdd, // SHUFPS XMM0, XMM1, 0xDD
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x50() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x50
    // 0x50 = 01 01 00 00 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x50, // SHUFPS XMM0, XMM1, 0x50
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xa0() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xA0
    // 0xA0 = 10 10 00 00 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0xa0, // SHUFPS XMM0, XMM1, 0xA0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xf0() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xF0
    // 0xF0 = 11 11 00 00 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0xf0, // SHUFPS XMM0, XMM1, 0xF0
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x0f() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x0F
    // 0x0F = 00 00 11 11 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x0f, // SHUFPS XMM0, XMM1, 0x0F
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x27() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x27
    // 0x27 = 00 10 01 11 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x27, // SHUFPS XMM0, XMM1, 0x27
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x72() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x72
    // 0x72 = 01 11 00 10 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0x72, // SHUFPS XMM0, XMM1, 0x72
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xb1() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xB1
    // 0xB1 = 10 11 00 01 (binary)
    let code = [
        0x0f, 0xc6, 0xc1, 0xb1, // SHUFPS XMM0, XMM1, 0xB1
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with different register pairs
// ============================================================================

#[test]
fn test_shufps_xmm1_xmm2_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM1, XMM2, 0xE4
    let code = [
        0x0f, 0xc6, 0xca, 0xe4, // SHUFPS XMM1, XMM2, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm3_xmm4_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM3, XMM4, 0xE4
    let code = [
        0x0f, 0xc6, 0xdc, 0xe4, // SHUFPS XMM3, XMM4, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm5_xmm6_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM5, XMM6, 0xE4
    let code = [
        0x0f, 0xc6, 0xee, 0xe4, // SHUFPS XMM5, XMM6, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm7_xmm0_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM7, XMM0, 0xE4
    let code = [
        0x0f, 0xc6, 0xf8, 0xe4, // SHUFPS XMM7, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Tests with high XMM registers (XMM8-XMM15)
// ============================================================================

#[test]
fn test_shufps_xmm8_xmm9_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM8, XMM9, 0xE4
    let code = [
        0x45, 0x0f, 0xc6, 0xc1, 0xe4, // SHUFPS XMM8, XMM9, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm10_xmm11_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM10, XMM11, 0xE4
    let code = [
        0x45, 0x0f, 0xc6, 0xd3, 0xe4, // SHUFPS XMM10, XMM11, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm12_xmm13_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM12, XMM13, 0xE4
    let code = [
        0x45, 0x0f, 0xc6, 0xe5, 0xe4, // SHUFPS XMM12, XMM13, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm14_xmm15_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM14, XMM15, 0xE4
    let code = [
        0x45, 0x0f, 0xc6, 0xf7, 0xe4, // SHUFPS XMM14, XMM15, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm8_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM8, 0xE4
    let code = [
        0x44, 0x0f, 0xc6, 0xc0, 0xe4, // SHUFPS XMM0, XMM8, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm15_xmm0_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM15, XMM0, 0xE4
    let code = [
        0x44, 0x0f, 0xc6, 0xf8, 0xe4, // SHUFPS XMM15, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Memory operand tests
// ============================================================================

#[test]
fn test_shufps_xmm0_mem_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM0, [mem], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x00, 0xe4, // SHUFPS XMM0, [RAX], 0xE4
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
fn test_shufps_xmm1_mem_imm_0x1b() {
    let mut emu = emu64();
    // SHUFPS XMM1, [mem], 0x1B
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x08, 0x1b, // SHUFPS XMM1, [RAX], 0x1B
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
fn test_shufps_xmm2_mem_imm_0x44() {
    let mut emu = emu64();
    // SHUFPS XMM2, [mem], 0x44
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x10, 0x44, // SHUFPS XMM2, [RAX], 0x44
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm3_mem_imm_0xee() {
    let mut emu = emu64();
    // SHUFPS XMM3, [mem], 0xEE
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x18, 0xee, // SHUFPS XMM3, [RAX], 0xEE
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA]);

    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm7_mem_imm_0x00() {
    let mut emu = emu64();
    // SHUFPS XMM7, [mem], 0x00
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x38, 0x00, // SHUFPS XMM7, [RAX], 0x00
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55]);

    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm7_mem_imm_0xff() {
    let mut emu = emu64();
    // SHUFPS XMM7, [mem], 0xFF
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x38, 0xff, // SHUFPS XMM7, [RAX], 0xFF
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);

    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33]);

    emu.run(None).unwrap();
}

// ============================================================================
// Additional immediate value coverage tests
// ============================================================================

#[test]
fn test_shufps_xmm0_xmm1_imm_0x39() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x39
    let code = [
        0x0f, 0xc6, 0xc1, 0x39, // SHUFPS XMM0, XMM1, 0x39
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x4e() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x4E
    let code = [
        0x0f, 0xc6, 0xc1, 0x4e, // SHUFPS XMM0, XMM1, 0x4E
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x93() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x93
    let code = [
        0x0f, 0xc6, 0xc1, 0x93, // SHUFPS XMM0, XMM1, 0x93
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xc6() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xC6
    let code = [
        0x0f, 0xc6, 0xc1, 0xc6, // SHUFPS XMM0, XMM1, 0xC6
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0xd8() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xD8
    let code = [
        0x0f, 0xc6, 0xc1, 0xd8, // SHUFPS XMM0, XMM1, 0xD8
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm0_xmm1_imm_0x2d() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x2D
    let code = [
        0x0f, 0xc6, 0xc1, 0x2d, // SHUFPS XMM0, XMM1, 0x2D
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

// ============================================================================
// Addressing mode tests
// ============================================================================

#[test]
fn test_shufps_xmm0_mem_displacement_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM0, [RAX + disp], 0xE4
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&(ALIGNED_ADDR - 0x10).to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x40, 0x10, 0xe4, // SHUFPS XMM0, [RAX+0x10], 0xE4
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &[0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77]);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_xmm1_mem_rbx_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM1, [RBX], 0xE4
    let code = [
        0x48, 0xbb, // MOV RBX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0xc6, 0x0b, 0xe4, // SHUFPS XMM1, [RBX], 0xE4
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
fn test_shufps_sequential_operations() {
    let mut emu = emu64();
    let code = [
        0x0f, 0xc6, 0xc1, 0xe4, // SHUFPS XMM0, XMM1, 0xE4
        0x0f, 0xc6, 0xd3, 0xe4, // SHUFPS XMM2, XMM3, 0xE4
        0x0f, 0xc6, 0xe5, 0xe4, // SHUFPS XMM4, XMM5, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_same_register_imm_0xe4() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM0, 0xE4 (shuffle with itself)
    let code = [
        0x0f, 0xc6, 0xc0, 0xe4, // SHUFPS XMM0, XMM0, 0xE4
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_same_register_imm_0x1b() {
    let mut emu = emu64();
    // SHUFPS XMM1, XMM1, 0x1B (shuffle with itself, reverse)
    let code = [
        0x0f, 0xc6, 0xc9, 0x1b, // SHUFPS XMM1, XMM1, 0x1B
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_broadcast_element_0() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x00 - broadcast element 0
    let code = [
        0x0f, 0xc6, 0xc1, 0x00, // SHUFPS XMM0, XMM1, 0x00
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_broadcast_element_1() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0x55 - broadcast element 1
    let code = [
        0x0f, 0xc6, 0xc1, 0x55, // SHUFPS XMM0, XMM1, 0x55
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}

#[test]
fn test_shufps_broadcast_element_2() {
    let mut emu = emu64();
    // SHUFPS XMM0, XMM1, 0xAA - broadcast element 2
    let code = [
        0x0f, 0xc6, 0xc1, 0xaa, // SHUFPS XMM0, XMM1, 0xAA
        0xf4, // HLT
    ];
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
}
