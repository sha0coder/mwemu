use crate::*;

// CMPSS - Compare Scalar Single Precision Floating-Point Value
//
// Performs a scalar single-precision floating-point comparison.
// Returns a mask of all 1s (0xFFFFFFFF) if comparison is true, or all 0s if false.
//
// Opcode: F3 0F C2 /r ib - CMPSS xmm1, xmm2/m32, imm8
//
// Comparison predicates (imm8 bits 2:0 for SSE):
// 0 (EQ_OQ)   - Equal (ordered, non-signaling)
// 1 (LT_OS)   - Less-than (ordered, signaling)
// 2 (LE_OS)   - Less-than-or-equal (ordered, signaling)
// 3 (UNORD_Q) - Unordered (non-signaling)
// 4 (NEQ_UQ)  - Not-equal (unordered, non-signaling)
// 5 (NLT_US)  - Not-less-than (unordered, signaling)
// 6 (NLE_US)  - Not-less-than-or-equal (unordered, signaling)
// 7 (ORD_Q)   - Ordered (non-signaling)

const ALIGNED_ADDR: u64 = 0x3000;
const ALIGNED_ADDR2: u64 = 0x3100;

// Helper to create float bit patterns
fn float_bits(val: f32) -> [u8; 4] {
    val.to_le_bytes()
}

// ============================================================================
// Predicate 0: EQ (Equal, ordered)
// ============================================================================

#[test]
fn test_cmpss_eq_equal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x00, // CMPSS XMM0, XMM0, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let data = float_bits(5.0);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &data);
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_eq_different_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x00, // CMPSS XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_eq_positive_and_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x00, // CMPSS XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_eq_same_denormal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x00, // CMPSS XMM0, XMM0, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let denormal = f32::from_bits(1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(denormal));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 1: LT (Less-than, ordered)
// ============================================================================

#[test]
fn test_cmpss_lt_true() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x01, // CMPSS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_lt_false() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x01, // CMPSS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(3.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_lt_equal_false() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x01, // CMPSS XMM0, XMM0, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(5.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_lt_negative_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x01, // CMPSS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-2.0));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 2: LE (Less-than or equal, ordered)
// ============================================================================

#[test]
fn test_cmpss_le_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x02, // CMPSS XMM0, XMM1, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.5));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_le_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x02, // CMPSS XMM0, XMM0, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(3.14));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_le_greater() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x02, // CMPSS XMM0, XMM1, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_le_with_zeros() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x02, // CMPSS XMM0, XMM1, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(0.0));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 3: UNORD (Unordered)
// ============================================================================

#[test]
fn test_cmpss_unord_ordered() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x03, // CMPSS XMM0, XMM1, 3 (UNORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_unord_same_value() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x03, // CMPSS XMM0, XMM0, 3 (UNORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(7.5));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 4: NEQ (Not equal, unordered)
// ============================================================================

#[test]
fn test_cmpss_neq_different() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x04, // CMPSS XMM0, XMM1, 4 (NEQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_neq_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x04, // CMPSS XMM0, XMM0, 4 (NEQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_neq_tiny_difference() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x04, // CMPSS XMM0, XMM1, 4 (NEQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1.0000001));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 5: NLT (Not less-than, unordered)
// ============================================================================

#[test]
fn test_cmpss_nlt_greater() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x05, // CMPSS XMM0, XMM1, 5 (NLT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(5.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nlt_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x05, // CMPSS XMM0, XMM0, 5 (NLT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(4.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nlt_less() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x05, // CMPSS XMM0, XMM1, 5 (NLT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(3.0));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 6: NLE (Not less-than or equal, unordered)
// ============================================================================

#[test]
fn test_cmpss_nle_greater() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x06, // CMPSS XMM0, XMM1, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(6.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nle_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x06, // CMPSS XMM0, XMM0, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.5));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nle_less_or_equal() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x06, // CMPSS XMM0, XMM1, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(3.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nle_negative_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x06, // CMPSS XMM0, XMM1, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-3.0));
    emu.run(None).unwrap();
}

// ============================================================================
// Predicate 7: ORD (Ordered)
// ============================================================================

#[test]
fn test_cmpss_ord_both_ordered() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x07, // CMPSS XMM0, XMM1, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(10.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(20.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_ord_same_value() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0xf3, 0x0f, 0xc2, 0xc0, 0x07, // CMPSS XMM0, XMM0, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(99.99));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_ord_very_small_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x07, // CMPSS XMM0, XMM1, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e-10));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2e-10));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_ord_large_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x07, // CMPSS XMM0, XMM1, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e10));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2e10));
    emu.run(None).unwrap();
}

// ============================================================================
// Additional Edge Cases and Special Values
// ============================================================================

#[test]
fn test_cmpss_eq_infinities() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x00, // CMPSS XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(f32::INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_lt_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x01, // CMPSS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_lt_negative_infinity() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x01, // CMPSS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(f32::NEG_INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_neq_large_numbers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x04, // CMPSS XMM0, XMM1, 4 (NEQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e30));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(2e30));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_le_mixed_signs() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x02, // CMPSS XMM0, XMM1, 2 (LE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-100.5));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(100.5));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nlt_large_positive_numbers() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x05, // CMPSS XMM0, XMM1, 5 (NLT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(1e20));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1e19));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_nle_zero_comparison() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x06, // CMPSS XMM0, XMM1, 6 (NLE)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(0.1));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_ord_both_negative_zero() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x07, // CMPSS XMM0, XMM1, 7 (ORD)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(-0.0));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(-0.0));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_eq_mixed_special_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x00, // CMPSS XMM0, XMM1, 0 (EQ)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(f32::NEG_INFINITY));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(f32::INFINITY));
    emu.run(None).unwrap();
}

#[test]
fn test_cmpss_lt_with_denormal_values() {
    let mut emu = emu64();
    let code = [
        0x48, 0xb8, // MOV RAX, imm64
    ];
    let mut full_code = code.to_vec();
    full_code.extend_from_slice(&ALIGNED_ADDR.to_le_bytes());
    full_code.extend_from_slice(&[
        0x48, 0xbb, // MOV RBX, imm64
    ]);
    full_code.extend_from_slice(&ALIGNED_ADDR2.to_le_bytes());
    full_code.extend_from_slice(&[
        0x0f, 0x28, 0x00, // MOVAPS XMM0, [RAX]
        0x0f, 0x28, 0x0b, // MOVAPS XMM1, [RBX]
        0xf3, 0x0f, 0xc2, 0xc1, 0x01, // CMPSS XMM0, XMM1, 1 (LT)
        0xf4, // HLT
    ]);

    emu.load_code_bytes(&full_code);
    let denormal = f32::from_bits(1);
    emu.maps.write_bytes_slice(ALIGNED_ADDR, &float_bits(denormal));
    emu.maps.write_bytes_slice(ALIGNED_ADDR2, &float_bits(1.0));
    emu.run(None).unwrap();
}
